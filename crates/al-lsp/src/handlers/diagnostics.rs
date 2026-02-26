use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use std::collections::HashMap;
use tower_lsp::Client;

use al_syntax::ast::{extract_name, node_text, AlObjectKind, AlSymbol, AlSymbolKind};
use al_syntax::document::DocumentState;
use al_syntax::navigation::node_at_offset;

use crate::state::WorldState;
use crate::handlers::completion::resolve_object_type_from_expression;

pub async fn publish_diagnostics(
    client: &Client,
    state: &WorldState,
    uri: &Url,
    doc: &DocumentState,
) {
    let mut diagnostics = doc.diagnostics.clone();
    // Semantic member diagnostics are expensive and low-value while syntax is broken.
    if diagnostics.is_empty() {
        diagnostics.extend(collect_semantic_member_diagnostics(state, uri, doc));
    }
    client
        .publish_diagnostics(uri.clone(), diagnostics, None)
        .await;
}

fn collect_semantic_member_diagnostics(
    state: &WorldState,
    caller_uri: &Url,
    doc: &DocumentState,
) -> Vec<Diagnostic> {
    let source = doc.source();
    if !source.contains('.') {
        return Vec::new();
    }

    let mut object_member_lookup_cache: ObjectMemberLookupCache = HashMap::new();
    let mut diagnostics = Vec::new();
    collect_member_diagnostics_recursive(
        state,
        &mut object_member_lookup_cache,
        caller_uri,
        doc,
        &source,
        doc.tree.root_node(),
        &mut diagnostics,
    );
    diagnostics
}

fn collect_member_diagnostics_recursive(
    state: &WorldState,
    object_member_lookup_cache: &mut ObjectMemberLookupCache,
    caller_uri: &Url,
    doc: &DocumentState,
    source: &str,
    node: tree_sitter::Node<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match node.kind() {
        "method_call" => {
            if let (Some(object_node), Some(member_node)) = (
                node.child_by_field_name("object"),
                node.child_by_field_name("method"),
            ) {
                validate_member_access(
                    state,
                    object_member_lookup_cache,
                    caller_uri,
                    doc,
                    source,
                    node.start_byte(),
                    object_node,
                    member_node,
                    true,
                    diagnostics,
                );
            }
        }
        "member_access" => {
            if let (Some(object_node), Some(member_node)) = (
                node.child_by_field_name("object"),
                node.child_by_field_name("member"),
            ) {
                validate_member_access(
                    state,
                    object_member_lookup_cache,
                    caller_uri,
                    doc,
                    source,
                    node.start_byte(),
                    object_node,
                    member_node,
                    false,
                    diagnostics,
                );
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_member_diagnostics_recursive(
            state,
            object_member_lookup_cache,
            caller_uri,
            doc,
            source,
            child,
            diagnostics,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcedureAccess {
    Public,
    Local,
    Protected,
    Internal,
}

#[derive(Debug, Clone)]
struct MemberMatch {
    kind: AlSymbolKind,
    access: ProcedureAccess,
    target_uri: Url,
}

#[derive(Debug, Clone)]
struct MemberEntry {
    kind: AlSymbolKind,
    access: ProcedureAccess,
}

#[derive(Debug, Clone)]
struct ObjectDeclEntry {
    uri: Url,
    members: HashMap<String, Vec<MemberEntry>>,
}

type ObjectMemberLookupCache = HashMap<(String, String), Option<Vec<ObjectDeclEntry>>>;

#[derive(Debug, Clone)]
struct ObjectContext {
    kind: String,
    name: String,
    extends: Option<(String, String)>,
}

fn resolve_object_entries(
    state: &WorldState,
    target_kind: &str,
    target_object_name: &str,
) -> Vec<ObjectDeclEntry> {
    let mut entries = Vec::new();
    let object_key = (
        target_kind.to_ascii_lowercase(),
        target_object_name.to_ascii_lowercase(),
    );

    if let Some(indexed_entries) = state.object_index.get(&object_key) {
        for indexed in indexed_entries.iter() {
            let Some(target_doc) = state.documents.get(&indexed.uri) else {
                continue;
            };
            let object_symbol = target_doc.symbol_table.symbols.iter().find(|object| {
                object.start_byte == indexed.object_start_byte
                    && matches!(
                        object.kind,
                        AlSymbolKind::Object(kind)
                            if kind.label().eq_ignore_ascii_case(target_kind)
                                && object.name.eq_ignore_ascii_case(target_object_name)
                    )
            });
            if let Some(object_symbol) = object_symbol {
                entries.push(build_object_decl_entry(
                    &indexed.uri,
                    &target_doc,
                    object_symbol,
                ));
            }
        }
        if !entries.is_empty() {
            return entries;
        }
    }

    // Fallback path for tests/legacy code paths that bypass state indexing.
    for entry in state.documents.iter() {
        let uri = entry.key().clone();
        let target_doc = entry.value();

        for object in &target_doc.symbol_table.symbols {
            let AlSymbolKind::Object(kind) = object.kind else {
                continue;
            };
            if !kind.label().eq_ignore_ascii_case(target_kind)
                || !object.name.eq_ignore_ascii_case(target_object_name)
            {
                continue;
            }

            entries.push(build_object_decl_entry(&uri, &target_doc, object));
        }
    }

    entries
}

fn is_simple_identifier_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn table_field_declared_in_source(
    state: &WorldState,
    table_name: &str,
    field_name: &str,
) -> bool {
    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();
        for symbol in &doc.symbol_table.symbols {
            let AlSymbolKind::Object(kind) = symbol.kind else {
                continue;
            };
            if kind.label() != "table" || !symbol.name.eq_ignore_ascii_case(table_name) {
                continue;
            }

            let start = symbol.start_byte.min(source.len());
            let end = symbol.end_byte.min(source.len());
            if start >= end {
                continue;
            }
            let object_text = &source[start..end];

            // Common AL form: field(<id>; "<Field Name>"; <Type>)
            let quoted_pattern = format!("; \"{}\";", field_name);
            if object_text.contains(&quoted_pattern) {
                return true;
            }

            // Unquoted identifier field name.
            if is_simple_identifier_name(field_name) {
                let bare_pattern = format!("; {};", field_name);
                if object_text.contains(&bare_pattern) {
                    return true;
                }
            }
        }
    }
    false
}

fn build_object_decl_entry(
    uri: &Url,
    target_doc: &DocumentState,
    object: &AlSymbol,
) -> ObjectDeclEntry {
    let mut members: HashMap<String, Vec<MemberEntry>> = HashMap::new();
    let mut source_cache: Option<String> = None;

    for child in &object.children {
        let access = if matches!(child.kind, AlSymbolKind::Procedure) {
            let source = source_cache.get_or_insert_with(|| target_doc.source());
            procedure_access_modifier(target_doc, source, child)
        } else {
            ProcedureAccess::Public
        };
        members
            .entry(child.name.to_ascii_lowercase())
            .or_default()
            .push(MemberEntry {
                kind: child.kind,
                access,
            });
    }

    ObjectDeclEntry {
        uri: uri.clone(),
        members,
    }
}

fn validate_member_access(
    state: &WorldState,
    object_member_lookup_cache: &mut ObjectMemberLookupCache,
    caller_uri: &Url,
    doc: &DocumentState,
    source: &str,
    scope_byte: usize,
    object_node: tree_sitter::Node<'_>,
    member_node: tree_sitter::Node<'_>,
    is_method_call: bool,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let object_node = unwrap_primary_expression(object_node);
    let member_node = unwrap_primary_expression(member_node);

    if !matches!(member_node.kind(), "identifier" | "quoted_identifier") {
        return;
    }

    let member_name = extract_name(member_node, source);

    let Some((target_kind, target_object_name)) = resolve_object_type_from_expression(
        state,
        doc,
        source,
        object_node,
        scope_byte,
        0,
    ) else {
        return;
    };

    if is_known_builtin_method(&target_kind, &member_name) {
        return;
    }
    if !is_method_call
        && target_kind.eq_ignore_ascii_case("table")
        && is_known_system_table_field(&member_name)
    {
        return;
    }

    let caller_object = enclosing_object_context(node_at_offset(&doc.tree, scope_byte), source);
    let object_key = (
        target_kind.to_ascii_lowercase(),
        target_object_name.to_ascii_lowercase(),
    );
    if !object_member_lookup_cache.contains_key(&object_key) {
        let resolved = resolve_object_entries(state, &target_kind, &target_object_name);
        if resolved.is_empty() {
            object_member_lookup_cache.insert(object_key.clone(), None);
        } else {
            object_member_lookup_cache.insert(object_key.clone(), Some(resolved));
        }
    }
    let Some(object_decls) = object_member_lookup_cache
        .get(&object_key)
        .and_then(|value| value.as_ref())
    else {
        return;
    };

    let mut matching_members: Vec<MemberMatch> = Vec::new();
    let member_key = member_name.to_ascii_lowercase();
    for object_decl in object_decls {
        if let Some(entries) = object_decl.members.get(&member_key) {
            for entry in entries {
                matching_members.push(MemberMatch {
                    kind: entry.kind,
                    access: entry.access,
                    target_uri: object_decl.uri.clone(),
                });
            }
        }
    }

    let range = ts_range_to_lsp_range(member_node.start_position(), member_node.end_position());

    if matching_members.is_empty() {
        if !is_method_call
            && target_kind.eq_ignore_ascii_case("table")
            && table_field_declared_in_source(state, &target_object_name, &member_name)
        {
            return;
        }

        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("al-lsp".to_string()),
            message: format!(
                "Unknown member `{}` on {} `{}`",
                member_name, target_kind, target_object_name
            ),
            ..Default::default()
        });
        return;
    }

    if is_method_call {
        if let Some(proc_match) = matching_members
            .iter()
            .find(|m| matches!(m.kind, AlSymbolKind::Procedure))
        {
            if !procedure_accessible(
                proc_match.access,
                caller_object.as_ref(),
                &target_kind,
                &target_object_name,
                caller_uri,
                &proc_match.target_uri,
            ) {
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("al-lsp".to_string()),
                    message: inaccessible_procedure_message(proc_match.access, &member_name),
                    ..Default::default()
                });
            }
            return;
        }

        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("al-lsp".to_string()),
            message: format!("Member `{}` is not callable", member_name),
            ..Default::default()
        });
        return;
    }

    if let Some(member_match) = matching_members.iter().find(|m| {
        matches!(
            m.kind,
            AlSymbolKind::Field | AlSymbolKind::Procedure | AlSymbolKind::EnumValue
        )
    }) {
        if matches!(member_match.kind, AlSymbolKind::Procedure)
            && !procedure_accessible(
                member_match.access,
                caller_object.as_ref(),
                &target_kind,
                &target_object_name,
                caller_uri,
                &member_match.target_uri,
            )
        {
            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("al-lsp".to_string()),
                message: inaccessible_procedure_message(member_match.access, &member_name),
                ..Default::default()
            });
        }
    }
}

fn unwrap_primary_expression(mut node: tree_sitter::Node<'_>) -> tree_sitter::Node<'_> {
    while node.kind() == "primary_expression" {
        let mut cursor = node.walk();
        let mut first_named = None;
        for child in node.named_children(&mut cursor) {
            first_named = Some(child);
            break;
        }
        let Some(child) = first_named else {
            break;
        };
        node = child;
    }
    node
}

fn ts_range_to_lsp_range(start: tree_sitter::Point, end: tree_sitter::Point) -> Range {
    Range {
        start: Position {
            line: start.row as u32,
            character: start.column as u32,
        },
        end: Position {
            line: end.row as u32,
            character: end.column as u32,
        },
    }
}

fn is_known_record_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "addlink"
            | "ascending"
            | "calcfields"
            | "calcsums"
            | "changecompany"
            | "copy"
            | "copyfilters"
            | "count"
            | "delete"
            | "deleteall"
            | "deletelinks"
            | "find"
            | "findfirst"
            | "findlast"
            | "findset"
            | "fieldcaption"
            | "get"
            | "getbyrecordid"
            | "getbysystemid"
            | "getfilters"
            | "getposition"
            | "hasfilter"
            | "init"
            | "insert"
            | "isempty"
            | "locktable"
            | "mark"
            | "markedonly"
            | "modify"
            | "modifyall"
            | "next"
            | "readisolation"
            | "recordid"
            | "rename"
            | "reset"
            | "setautocalcfields"
            | "setcurrentkey"
            | "setfilter"
            | "setloadfields"
            | "setposition"
            | "setrange"
            | "setrecfilter"
            | "tablecaption"
            | "tablename"
            | "testfield"
            | "transferfields"
            | "validate"
    )
}

fn is_known_codeunit_method(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(), "run")
}

fn is_known_page_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "run"
            | "runmodal"
            | "setrecord"
            | "getrecord"
            | "update"
            | "close"
            | "settableview"
            | "lookupmode"
    )
}

fn is_known_report_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "run"
            | "runmodal"
            | "print"
            | "saveas"
            | "execute"
            | "settableview"
            | "setrecord"
            | "userequestpage"
    )
}

fn is_known_xmlport_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "run" | "import" | "export" | "setsource" | "setdestination"
    )
}

fn is_known_query_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "open" | "read" | "close" | "saveasxml" | "setfilter"
    )
}

fn is_known_enum_method(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(), "names" | "ordinals")
}

fn is_known_list_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "add"
            | "addrange"
            | "contains"
            | "count"
            | "get"
            | "getrange"
            | "indexof"
            | "insert"
            | "remove"
            | "removeat"
            | "set"
    )
}

fn is_known_dictionary_method(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "add"
            | "containskey"
            | "containsvalue"
            | "count"
            | "get"
            | "keys"
            | "remove"
            | "set"
            | "values"
    )
}

fn is_known_builtin_method(object_kind: &str, method_name: &str) -> bool {
    match object_kind.to_ascii_lowercase().as_str() {
        "table" => is_known_record_method(method_name),
        "codeunit" => is_known_codeunit_method(method_name),
        "page" => is_known_page_method(method_name),
        "report" => is_known_report_method(method_name),
        "xmlport" => is_known_xmlport_method(method_name),
        "query" => is_known_query_method(method_name),
        "enum" => is_known_enum_method(method_name),
        "list" => is_known_list_method(method_name),
        "dictionary" => is_known_dictionary_method(method_name),
        _ => false,
    }
}

fn is_known_system_table_field(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "systemid"
            | "systemcreatedat"
            | "systemcreatedby"
            | "systemmodifiedat"
            | "systemmodifiedby"
    )
}

fn enclosing_object_context(
    mut node: Option<tree_sitter::Node<'_>>,
    source: &str,
) -> Option<ObjectContext> {
    while let Some(n) = node {
        if let Some(kind) = AlObjectKind::from_node_kind(n.kind()) {
            if let Some(name) = first_object_name(n, source) {
                let extends = match n.kind() {
                    "table_extension_declaration" => second_object_name(n, source)
                        .map(|base_name| ("table".to_string(), base_name)),
                    "page_extension_declaration" => second_object_name(n, source)
                        .map(|base_name| ("page".to_string(), base_name)),
                    "enum_extension_declaration" => second_object_name(n, source)
                        .map(|base_name| ("enum".to_string(), base_name)),
                    _ => None,
                };
                return Some(ObjectContext {
                    kind: kind.label().to_string(),
                    name,
                    extends,
                });
            }
        }
        node = n.parent();
    }
    None
}

fn first_object_name(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if matches!(child.kind(), "identifier" | "quoted_identifier") {
            return Some(extract_name(child, source));
        }
    }
    None
}

fn second_object_name(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    let mut names = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if matches!(child.kind(), "identifier" | "quoted_identifier") {
            names.push(extract_name(child, source));
        }
    }
    if names.len() >= 2 {
        Some(names[1].clone())
    } else {
        None
    }
}

fn is_same_object(caller: Option<&ObjectContext>, target_kind: &str, target_name: &str) -> bool {
    caller.is_some_and(|caller| {
        caller.kind.eq_ignore_ascii_case(target_kind)
            && caller.name.eq_ignore_ascii_case(target_name)
    })
}

fn procedure_access_modifier(doc: &DocumentState, source: &str, sym: &AlSymbol) -> ProcedureAccess {
    let Some(mut node) = node_at_offset(&doc.tree, sym.start_byte) else {
        return ProcedureAccess::Public;
    };

    loop {
        if matches!(node.kind(), "procedure_declaration" | "interface_method") {
            let Some(name_node) = node.child_by_field_name("name") else {
                return ProcedureAccess::Public;
            };
            if !extract_name(name_node, source).eq_ignore_ascii_case(&sym.name) {
                return ProcedureAccess::Public;
            }
            if let Some(access_node) = node.child_by_field_name("access") {
                return parse_procedure_access(node_text(access_node, source));
            }
            let header = node_text(node, source).trim_start().to_ascii_lowercase();
            if header.starts_with("local procedure") {
                return ProcedureAccess::Local;
            }
            if header.starts_with("protected procedure") {
                return ProcedureAccess::Protected;
            }
            if header.starts_with("internal procedure") {
                return ProcedureAccess::Internal;
            }
            return ProcedureAccess::Public;
        }
        let Some(parent) = node.parent() else {
            return ProcedureAccess::Public;
        };
        node = parent;
    }
}

fn parse_procedure_access(access_text: &str) -> ProcedureAccess {
    match access_text.trim().to_ascii_lowercase().as_str() {
        "local" => ProcedureAccess::Local,
        "protected" => ProcedureAccess::Protected,
        "internal" => ProcedureAccess::Internal,
        _ => ProcedureAccess::Public,
    }
}

fn procedure_accessible(
    access: ProcedureAccess,
    caller_object: Option<&ObjectContext>,
    target_kind: &str,
    target_name: &str,
    caller_uri: &Url,
    target_uri: &Url,
) -> bool {
    match access {
        ProcedureAccess::Public => true,
        ProcedureAccess::Local => is_same_object(caller_object, target_kind, target_name),
        ProcedureAccess::Protected => {
            if is_same_object(caller_object, target_kind, target_name) {
                return true;
            }
            caller_object.is_some_and(|ctx| {
                ctx.extends.as_ref().is_some_and(|(base_kind, base_name)| {
                    base_kind.eq_ignore_ascii_case(target_kind)
                        && base_name.eq_ignore_ascii_case(target_name)
                })
            })
        }
        ProcedureAccess::Internal => same_internal_scope(caller_uri, target_uri),
    }
}

fn same_internal_scope(caller_uri: &Url, target_uri: &Url) -> bool {
    let caller_scheme = caller_uri.scheme();
    let target_scheme = target_uri.scheme();

    if caller_scheme == "alpackage" && target_scheme == "alpackage" {
        return caller_uri
            .host_str()
            .zip(target_uri.host_str())
            .is_some_and(|(a, b)| a.eq_ignore_ascii_case(b));
    }

    if caller_scheme == "alpackage" || target_scheme == "alpackage" {
        return false;
    }

    true
}

fn inaccessible_procedure_message(access: ProcedureAccess, member_name: &str) -> String {
    match access {
        ProcedureAccess::Local => format!(
            "Procedure `{}` is local and cannot be accessed from this scope",
            member_name
        ),
        ProcedureAccess::Protected => format!(
            "Procedure `{}` is protected and cannot be accessed from this scope",
            member_name
        ),
        ProcedureAccess::Internal => format!(
            "Procedure `{}` is internal and cannot be accessed from this extension",
            member_name
        ),
        ProcedureAccess::Public => format!(
            "Procedure `{}` cannot be accessed from this scope",
            member_name
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::Url;

    #[test]
    fn test_semantic_diagnostic_unknown_record_member() {
        let source = r#"table 50100 Customer
{
    fields
    {
        field(1; Name; Text[100]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.DoesNotExist();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            diags
                .iter()
                .any(|d| d.message.contains("Unknown member `DoesNotExist`")),
            "expected unknown member diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_local_procedure_inaccessible() {
        let source = r#"table 50100 Customer
{
    procedure PublicProc()
    begin
    end;

    local procedure HiddenProc()
    begin
    end;
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.HiddenProc();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            diags
                .iter()
                .any(|d| d.message.contains("local and cannot be accessed")),
            "expected local access diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_record_method() {
        let source = r#"table 50100 Customer
{
    fields
    {
        field(1; Name; Text[100]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.FindFirst();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| d.message.contains("FindFirst")),
            "did not expect diagnostics for built-in Record method, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_record_method_without_parentheses() {
        let source = r#"table 50100 Customer
{
    fields
    {
        field(1; Name; Text[100]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.Reset;
        Rec.FindSet;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Reset") || d.message.contains("FindSet")),
            "did not expect diagnostics for no-parens built-in Record methods, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_record_tablename_without_parentheses() {
        let source = r#"table 50100 Customer
{
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
        NameTxt: Text;
    begin
        NameTxt := Rec.TableName;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| d.message.contains("TableName")),
            "did not expect diagnostics for TableName built-in Record method, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_page_method() {
        let source = r#"page 50100 "Customer Card"
{
}

codeunit 50100 Test
{
    procedure RunPage()
    var
        P: Page "Customer Card";
    begin
        P.RunModal();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| d.message.contains("RunModal")),
            "did not expect diagnostics for built-in Page method, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_page_settableview_and_lookupmode() {
        let source = r#"table 50100 "Dummy Setup"
{
    fields
    {
        field(1; Code; Code[20]) { }
    }
}

page 50100 "Dummy Setup Card"
{
    SourceTable = "Dummy Setup";
}

codeunit 50100 Test
{
    procedure Run()
    var
        SetupRec: Record "Dummy Setup";
        SetupPage: Page "Dummy Setup Card";
        Result: Action;
    begin
        SetupPage.SetTableView(SetupRec);
        SetupPage.LookupMode(true);
        Result := SetupPage.RunModal;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| {
                d.message.contains("SetTableView")
                    || d.message.contains("LookupMode")
                    || d.message.contains("RunModal")
            }),
            "did not expect diagnostics for built-in Page methods, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_enum_names_and_ordinals() {
        let source = r#"enum 50100 "Dummy State"
{
    value(0; Open) { }
    value(1; Closed) { }
}

codeunit 50100 Test
{
    procedure Run()
    var
        CurrentState: Enum "Dummy State";
    begin
        CurrentState.Names;
        CurrentState.Ordinals;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Names") || d.message.contains("Ordinals")),
            "did not expect diagnostics for built-in Enum members, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_fields_after_tablerelation_if_else() {
        let source = r#"table 50100 "Dummy Hospitality Setup"
{
    fields
    {
        field(1; "Restaurant No."; Code[20]) { }
        field(2; "Access To Other Restaurant"; Code[20]) { }
        field(3; "Dining Area ID"; Code[20])
        {
            TableRelation = IF ("Access To Other Restaurant" = FILTER('')) "Dummy Dining Area".ID WHERE("Restaurant No." = FIELD("Restaurant No."))
                            ELSE
                            IF ("Access To Other Restaurant" = FILTER(<> '')) "Dummy Dining Area".ID WHERE("Restaurant No." = FIELD("Access To Other Restaurant"));
        }
        field(4; "Display/Printing Mode"; Integer) { }
        field(5; "Service Flow ID"; Code[20]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Setup: Record "Dummy Hospitality Setup";
    begin
        if Setup."Display/Printing Mode" = 0 then
            Setup."Service Flow ID" := 'FLOW-1';
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| {
                d.message.contains("Display/Printing Mode")
                    || d.message.contains("Service Flow ID")
            }),
            "did not expect diagnostics for valid quoted fields, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_record_enum_field_qualified_value() {
        let source = r#"enum 50100 "Dummy Trigger Mode"
{
    value(0; "On Item Added") { }
    value(1; "Manual") { }
}

table 50100 "Dummy Hospitality Setup"
{
    fields
    {
        field(1; "Display/Printing Mode"; Enum "Dummy Trigger Mode") { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Setup: Record "Dummy Hospitality Setup";
    begin
        if Setup."Display/Printing Mode" = Setup."Display/Printing Mode"::"On Item Added" then;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| {
                d.message.contains("Display/Printing Mode")
                    || d.message.contains("On Item Added")
            }),
            "did not expect diagnostics for Record.EnumField::EnumValue access, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_record_enum_field_after_tablerelation_if_else_field() {
        let source = r#"enum 50100 "Dummy KDS-Trigger Sends"
{
    value(0; "On Item Added") { }
    value(1; "Manual") { }
}

table 50100 "Dummy Hospitality Type"
{
    fields
    {
        field(8; "Dining Area ID"; Code[20])
        {
            TableRelation = IF ("Access To Other Restaurant" = FILTER('')) "Dummy Dining Area".ID WHERE("Restaurant No." = FIELD("Restaurant No."))
            ELSE
            IF ("Access To Other Restaurant" = FILTER(<> '')) "Dummy Dining Area".ID WHERE("Restaurant No." = FIELD("Access To Other Restaurant"));
        }
        field(9; "Access To Other Restaurant"; Code[20]) { }
        field(10; "KDS Display/Printing"; Enum "Dummy KDS-Trigger Sends") { }
        field(11; "Restaurant No."; Code[20]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        HospType: Record "Dummy Hospitality Type";
    begin
        if HospType."KDS Display/Printing" = HospType."KDS Display/Printing"::"On Item Added" then;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("KDS Display/Printing")),
            "did not expect diagnostics for KDS Display/Printing access after TableRelation IF/ELSE, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_query_method() {
        let source = r#"query 50100 "Customer Sales"
{
}

codeunit 50100 Test
{
    procedure RunQuery()
    var
        Q: Query "Customer Sales";
    begin
        Q.Open();
        Q.Read();
        Q.Close();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Open") || d.message.contains("Read") || d.message.contains("Close")),
            "did not expect diagnostics for built-in Query methods, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_list_method() {
        let source = r#"codeunit 50100 Test
{
    procedure Run()
    var
        Tags: List of [Text];
    begin
        Tags.Add('A');
        Tags.Count();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Add") || d.message.contains("Count")),
            "did not expect diagnostics for built-in List methods, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_builtin_dictionary_method() {
        let source = r#"codeunit 50100 Test
{
    procedure Run()
    var
        Tags: Dictionary of [Text, Text];
    begin
        Tags.Add('A', 'B');
        Tags.ContainsKey('A');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Add") || d.message.contains("ContainsKey")),
            "did not expect diagnostics for built-in Dictionary methods, got: {diags:?}"
        );
    }

    #[test]
    fn test_no_semantic_diagnostic_for_codeunit_builtin_run_invocation() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        FunctionSetup2: Record "Function Setup";
        MenuLine2_l: Record "Menu Line";
    begin
        CODEUNIT.Run(FunctionSetup2."Run Codeunit", MenuLine2_l);
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            !diags.iter().any(|d| d.message.contains("Run")),
            "did not expect diagnostics for CODEUNIT built-in Run method, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_unknown_record_field_member_access() {
        let source = r#"table 50100 Customer
{
    fields
    {
        field(1; Name; Text[100]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
        S: Text;
    begin
        S := Rec.DoesNotExistField;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            diags
                .iter()
                .any(|d| d.message.contains("Unknown member `DoesNotExistField`")),
            "expected unknown member diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_field_not_callable() {
        let source = r#"table 50100 Customer
{
    fields
    {
        field(1; Name; Text[100]) { }
    }
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.Name();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            diags.iter().any(|d| d.message.contains("not callable")),
            "expected not callable diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_protected_procedure_accessible_from_tableextension() {
        let target_source = r#"table 50100 Customer
{
    protected procedure HiddenProc()
    begin
    end;
}"#;
        let caller_source = r#"tableextension 50101 CustomerExt extends Customer
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.HiddenProc();
    end;
}"#;
        let target_uri = Url::parse("file:///test/customer.al").unwrap();
        let caller_uri = Url::parse("file:///test/caller.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(target_uri, DocumentState::new(target_source).unwrap());
        state.documents.insert(
            caller_uri.clone(),
            DocumentState::new(caller_source).unwrap(),
        );
        let doc = state.documents.get(&caller_uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &caller_uri, &doc);
        assert!(
            !diags.iter().any(|d| d.message.contains("protected")),
            "did not expect protected-access diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_protected_procedure_inaccessible_outside_extension_scope() {
        let source = r#"table 50100 Customer
{
    protected procedure HiddenProc()
    begin
    end;
}

codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.HiddenProc();
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());
        let doc = state.documents.get(&uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &uri, &doc);
        assert!(
            diags.iter().any(|d| d.message.contains("protected")),
            "expected protected-access diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_internal_procedure_inaccessible_across_package_boundary() {
        let target_source = r#"table 50100 Customer
{
    internal procedure HiddenProc()
    begin
    end;
}"#;
        let caller_source = r#"codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.HiddenProc();
    end;
}"#;
        let target_uri = Url::parse("alpackage://DepPkg/customer.al").unwrap();
        let caller_uri = Url::parse("file:///workspace/caller.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(target_uri, DocumentState::new(target_source).unwrap());
        state.documents.insert(
            caller_uri.clone(),
            DocumentState::new(caller_source).unwrap(),
        );
        let doc = state.documents.get(&caller_uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &caller_uri, &doc);
        assert!(
            diags.iter().any(|d| d.message.contains("internal")),
            "expected internal-access diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn test_semantic_diagnostic_internal_procedure_accessible_within_same_alpackage() {
        let target_source = r#"table 50100 Customer
{
    internal procedure HiddenProc()
    begin
    end;
}"#;
        let caller_source = r#"codeunit 50100 Test
{
    procedure Run()
    var
        Rec: Record Customer;
    begin
        Rec.HiddenProc();
    end;
}"#;
        let target_uri = Url::parse("alpackage://DepPkg/customer.al").unwrap();
        let caller_uri = Url::parse("alpackage://DepPkg/caller.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(target_uri, DocumentState::new(target_source).unwrap());
        state.documents.insert(
            caller_uri.clone(),
            DocumentState::new(caller_source).unwrap(),
        );
        let doc = state.documents.get(&caller_uri).unwrap();
        let diags = collect_semantic_member_diagnostics(&state, &caller_uri, &doc);
        assert!(
            !diags.iter().any(|d| d.message.contains("internal")),
            "did not expect internal-access diagnostic, got: {diags:?}"
        );
    }
}
