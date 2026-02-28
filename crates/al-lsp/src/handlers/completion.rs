use std::collections::HashSet;

use lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse};

use al_syntax::ast::{extract_name, AlSymbolKind};
use al_syntax::document::DocumentState;
use al_syntax::navigation::{
    extract_type_object_name, identifier_context_at_offset, node_at_offset,
};
use al_syntax::symbols::al_keywords;

use crate::convert::lsp_position_to_byte_offset;
use crate::handlers::events::event_subscriber_completion_items;
use crate::state::WorldState;

#[derive(Debug, Clone)]
pub(crate) enum EnumContext {
    Direct {
        qualifier_name: String,
        qualifier_byte_offset: usize,
    },
    Member {
        object_byte_offset: usize,
        member_name: String,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct EnumValueUsage {
    pub context: EnumContext,
    pub value_name: String,
    pub start: tree_sitter::Point,
    pub end: tree_sitter::Point,
}

#[derive(Debug, Clone)]
enum PropertyCompletionContext {
    Names { scope: &'static str },
    Values { property_name: String },
}

pub fn handle_completion(
    state: &WorldState,
    params: CompletionParams,
) -> Option<CompletionResponse> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();
    let prefix_lower = extract_prefix(&source, byte_offset).to_lowercase();

    let enum_context = enum_context_at_offset(&doc.tree, &source, byte_offset);
    let where_value_context =
        where_value_completion_context_at_offset(&doc.tree, &source, byte_offset);
    let property_context = property_completion_context_at_offset(&doc.tree, &source, byte_offset);
    let event_items = event_subscriber_completion_items(state, &source, byte_offset, &prefix_lower);
    let dot_target = dot_target_at_offset(state, &doc, &source, byte_offset);

    drop(doc);

    if let Some(items) = event_items {
        if !items.is_empty() {
            return Some(CompletionResponse::Array(items));
        }
    }

    if let Some(enum_context) = enum_context {
        if let Some(enum_name) = resolve_enum_name_from_context(state, &uri, &enum_context) {
            let enum_items = collect_enum_value_completions(state, &enum_name, &prefix_lower);
            if !enum_items.is_empty() {
                return Some(CompletionResponse::Array(enum_items));
            }
        }
    }

    if let Some((object_kind, object_name)) = dot_target {
        let object_items =
            collect_object_member_completions(state, &object_kind, &object_name, &prefix_lower);
        if !object_items.is_empty() {
            return Some(CompletionResponse::Array(object_items));
        }
    }

    if where_value_context {
        let where_items = collect_where_value_expression_completions(&prefix_lower);
        if !where_items.is_empty() {
            return Some(CompletionResponse::Array(where_items));
        }
    }

    if let Some(property_context) = property_context {
        let property_items = match property_context {
            PropertyCompletionContext::Names { scope } => {
                collect_property_name_completions(scope, &prefix_lower)
            }
            PropertyCompletionContext::Values { property_name } => {
                collect_property_value_completions(state, &property_name, &prefix_lower)
            }
        };
        if !property_items.is_empty() {
            return Some(CompletionResponse::Array(property_items));
        }
    }

    let doc = state.documents.get(&uri)?;
    let mut items = Vec::new();

    // Add reachable symbols.
    let reachable = doc.symbol_table.reachable_symbols(byte_offset);
    for sym in reachable {
        if !prefix_lower.is_empty() && !sym.name.to_lowercase().starts_with(&prefix_lower) {
            continue;
        }

        items.push(CompletionItem {
            label: sym.name.clone(),
            kind: Some(completion_item_kind(sym.kind)),
            detail: sym.type_info.clone(),
            ..Default::default()
        });
    }

    // Add keywords.
    for &kw in al_keywords() {
        if !prefix_lower.is_empty() && !kw.to_lowercase().starts_with(&prefix_lower) {
            continue;
        }

        items.push(CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        });
    }

    if items.is_empty() {
        return None;
    }

    Some(CompletionResponse::Array(items))
}

fn collect_object_member_completions(
    state: &WorldState,
    object_kind: &str,
    object_name: &str,
    prefix_lower: &str,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let mut seen = HashSet::new();

    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            if let AlSymbolKind::Object(kind) = symbol.kind {
                if !kind.label().eq_ignore_ascii_case(object_kind)
                    || !symbol.name.eq_ignore_ascii_case(object_name)
                {
                    continue;
                }

                for child in &symbol.children {
                    if !prefix_lower.is_empty()
                        && !child.name.to_lowercase().starts_with(prefix_lower)
                    {
                        continue;
                    }
                    let key = child.name.to_lowercase();
                    if !seen.insert(key) {
                        continue;
                    }

                    items.push(CompletionItem {
                        label: child.name.clone(),
                        kind: Some(completion_item_kind(child.kind)),
                        detail: child.type_info.clone(),
                        ..Default::default()
                    });
                }
            }
        }
    }

    if let Some(methods) = builtin_methods_for_object_kind(object_kind) {
        for method in methods {
            if !prefix_lower.is_empty() && !method.to_lowercase().starts_with(prefix_lower) {
                continue;
            }
            let key = method.to_lowercase();
            if !seen.insert(key) {
                continue;
            }
            items.push(CompletionItem {
                label: (*method).to_string(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(format!(
                    "Built-in {} method",
                    object_kind.trim_end_matches("_builtin")
                )),
                ..Default::default()
            });
        }
    }

    items
}

fn collect_enum_value_completions(
    state: &WorldState,
    enum_name: &str,
    prefix_lower: &str,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let mut seen = HashSet::new();

    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            let AlSymbolKind::Object(kind) = symbol.kind else {
                continue;
            };
            if kind.label() != "enum" || !symbol.name.eq_ignore_ascii_case(enum_name) {
                continue;
            }

            for child in &symbol.children {
                if !matches!(child.kind, AlSymbolKind::EnumValue) {
                    continue;
                }
                if !prefix_lower.is_empty() && !child.name.to_lowercase().starts_with(prefix_lower)
                {
                    continue;
                }
                let key = child.name.to_lowercase();
                if !seen.insert(key) {
                    continue;
                }

                items.push(CompletionItem {
                    label: child.name.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: child.type_info.clone(),
                    ..Default::default()
                });
            }
        }
    }

    items
}

fn property_names_for_scope(scope: &str) -> &'static [&'static str] {
    match scope {
        "table" => &[
            "Access",
            "Caption",
            "DataCaptionFields",
            "DataClassification",
            "DataPerCompany",
            "DrillDownPageID",
            "LookupPageID",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
            "PasteIsValid",
            "Permissions",
            "ReplicateData",
        ],
        "codeunit" => &[
            "Access",
            "EventSubscriberInstance",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
            "Permissions",
            "SingleInstance",
            "Subtype",
            "TableNo",
        ],
        "page" => &[
            "Access",
            "ApplicationArea",
            "Caption",
            "DeleteAllowed",
            "Editable",
            "InsertAllowed",
            "LinksAllowed",
            "ModifyAllowed",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
            "PageType",
            "SourceTable",
            "UsageCategory",
        ],
        "field" => &[
            "ApplicationArea",
            "AutoFormatType",
            "BlankZero",
            "CalcFormula",
            "Caption",
            "DataClassification",
            "Editable",
            "FieldClass",
            "InitValue",
            "NotBlank",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
            "OptionCaption",
            "OptionMembers",
            "TableRelation",
            "ToolTip",
            "ValidateTableRelation",
        ],
        "key" => &[
            "Clustered",
            "Enabled",
            "MaintainSQLIndex",
            "SumIndexFields",
            "Unique",
        ],
        "enum" => &[
            "Access",
            "AssignmentCompatibility",
            "Caption",
            "Extensible",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
        ],
        "enumvalue" => &[
            "Caption",
            "Implementation",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
        ],
        "report" => &[
            "Access",
            "ApplicationArea",
            "Caption",
            "DefaultLayout",
            "ExcelLayout",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
            "RDLCLayout",
            "UsageCategory",
            "WordLayout",
        ],
        "query" => &[
            "Access",
            "Caption",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
        ],
        "xmlport" => &[
            "Access",
            "Caption",
            "Direction",
            "Encoding",
            "FormatEvaluate",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
        ],
        "interface" => &[
            "Access",
            "Caption",
            "ObsoleteReason",
            "ObsoleteState",
            "ObsoleteTag",
        ],
        _ => &[],
    }
}

fn collect_property_name_completions(scope: &str, prefix_lower: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    for &name in property_names_for_scope(scope) {
        if !prefix_lower.is_empty() && !name.to_lowercase().starts_with(prefix_lower) {
            continue;
        }
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some(format!("{scope} property")),
            ..Default::default()
        });
    }
    items
}

fn property_value_literals(property_name: &str) -> &'static [&'static str] {
    match property_name.to_ascii_lowercase().as_str() {
        "dataclassification" => &[
            "ToBeClassified",
            "CustomerContent",
            "EndUserIdentifiableInformation",
            "AccountData",
            "EndUserPseudonymousIdentifiers",
            "OrganizationIdentifiableInformation",
            "SystemMetadata",
        ],
        "obsoletestate" => &["No", "Pending", "Removed"],
        "singleinstance" => &["true", "false"],
        "editable" => &["true", "false"],
        "insertallowed" => &["true", "false"],
        "modifyallowed" => &["true", "false"],
        "deleteallowed" => &["true", "false"],
        "linksallowed" => &["true", "false"],
        "enabled" => &["true", "false"],
        "clustered" => &["true", "false"],
        "maintainsqlindex" => &["true", "false"],
        "notblank" => &["true", "false"],
        "blankzero" => &["true", "false"],
        "validatetablerelation" => &["true", "false"],
        "fieldclass" => &["Normal", "FlowField", "FlowFilter"],
        "pagetype" => &[
            "Card",
            "CardPart",
            "List",
            "ListPart",
            "Document",
            "Worksheet",
            "RoleCenter",
            "StandardDialog",
            "ConfirmationDialog",
            "NavigatePage",
            "API",
            "HeadlinePart",
        ],
        "usagecategory" => &[
            "None",
            "Lists",
            "Tasks",
            "ReportsAndAnalysis",
            "Documents",
            "History",
            "Administration",
        ],
        "eventsubscriberinstance" => &["StaticAutomatic", "Manual"],
        "subtype" => &["Normal", "Install", "Upgrade", "Test", "TestRunner"],
        "direction" => &["Import", "Export", "Both"],
        "defaultlayout" => &["None", "RDLC", "Word", "Excel"],
        "accesstoplevel" => &["Internal", "Public"],
        "access" => &["Internal", "Public"],
        "calcformula" => &["Lookup", "Sum", "Average", "Count", "Exist", "Min", "Max"],
        _ => &[],
    }
}

fn collect_object_name_value_completions(
    state: &WorldState,
    object_kind: &str,
    prefix_lower: &str,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let mut seen = HashSet::new();

    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            let AlSymbolKind::Object(kind) = symbol.kind else {
                continue;
            };
            if !kind.label().eq_ignore_ascii_case(object_kind) {
                continue;
            }
            if !prefix_lower.is_empty() && !symbol.name.to_lowercase().starts_with(prefix_lower) {
                continue;
            }
            let key = symbol.name.to_lowercase();
            if !seen.insert(key) {
                continue;
            }
            items.push(CompletionItem {
                label: symbol.name.clone(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some(format!("{object_kind} object")),
                ..Default::default()
            });
        }
    }

    items
}

fn collect_property_value_completions(
    state: &WorldState,
    property_name: &str,
    prefix_lower: &str,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let mut seen = HashSet::new();

    for &value in property_value_literals(property_name) {
        if !prefix_lower.is_empty() && !value.to_lowercase().starts_with(prefix_lower) {
            continue;
        }
        let key = value.to_lowercase();
        if !seen.insert(key) {
            continue;
        }
        items.push(CompletionItem {
            label: value.to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some(format!("{property_name} value")),
            ..Default::default()
        });
    }

    let dynamic_kind = match property_name.to_ascii_lowercase().as_str() {
        "sourcetable" | "tableno" => Some("table"),
        "tablerelation" => Some("table"),
        "lookuppageid" | "drilldownpageid" | "cardpageid" | "listpageid" => Some("page"),
        "runobject" => Some("codeunit"),
        _ => None,
    };

    if let Some(kind) = dynamic_kind {
        let dynamic_items = collect_object_name_value_completions(state, kind, prefix_lower);
        for item in dynamic_items {
            let key = item.label.to_lowercase();
            if seen.insert(key) {
                items.push(item);
            }
        }
    }

    items
}

fn collect_where_value_expression_completions(prefix_lower: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    for value in ["FIELD(", "CONST(", "FILTER("] {
        if !prefix_lower.is_empty() && !value.to_lowercase().starts_with(prefix_lower) {
            continue;
        }
        items.push(CompletionItem {
            label: value.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("WHERE value expression".to_string()),
            ..Default::default()
        });
    }
    items
}

pub(crate) fn resolve_enum_name_from_context(
    state: &WorldState,
    uri: &lsp_types::Url,
    context: &EnumContext,
) -> Option<String> {
    match context {
        EnumContext::Direct {
            qualifier_name,
            qualifier_byte_offset,
        } => {
            if has_enum_object(state, qualifier_name) {
                return Some(qualifier_name.clone());
            }

            let doc = state.documents.get(uri)?;
            let source = doc.source();
            let ctx = identifier_context_at_offset(
                &doc.tree,
                &source,
                &doc.symbol_table,
                *qualifier_byte_offset,
            )?;
            let sym = ctx.symbol?;
            let type_info = sym.type_info.as_deref()?;
            let (object_kind, enum_name) = extract_type_object_name(type_info)?;
            if object_kind.eq_ignore_ascii_case("enum") {
                return Some(enum_name.to_string());
            }
            None
        }
        EnumContext::Member {
            object_byte_offset,
            member_name,
        } => {
            let doc = state.documents.get(uri)?;
            let source = doc.source();
            let ctx = identifier_context_at_offset(
                &doc.tree,
                &source,
                &doc.symbol_table,
                *object_byte_offset,
            )?;
            let sym = ctx.symbol?;
            let type_info = sym.type_info.as_deref()?;
            let (object_kind, table_name) = extract_type_object_name(type_info)?;
            if !object_kind.eq_ignore_ascii_case("table") {
                return None;
            }

            find_table_field_enum_type(state, table_name, member_name)
        }
    }
}

fn has_enum_object(state: &WorldState, enum_name: &str) -> bool {
    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            if let AlSymbolKind::Object(kind) = symbol.kind {
                if kind.label() == "enum" && symbol.name.eq_ignore_ascii_case(enum_name) {
                    return true;
                }
            }
        }
    }
    false
}

fn find_table_field_enum_type(
    state: &WorldState,
    table_name: &str,
    field_name: &str,
) -> Option<String> {
    let type_info = find_table_field_type(state, table_name, field_name)?;
    let (object_kind, enum_name) = extract_type_object_name(&type_info)?;
    if object_kind.eq_ignore_ascii_case("enum") {
        return Some(enum_name.to_string());
    }
    None
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

fn parse_field_type_after_marker(object_text: &str, marker_end: usize) -> Option<String> {
    if marker_end >= object_text.len() {
        return None;
    }
    let bytes = object_text.as_bytes();
    let mut cursor = marker_end;
    while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    if cursor >= bytes.len() {
        return None;
    }
    let close_paren_rel = object_text[cursor..].find(')')?;
    let close_paren = cursor + close_paren_rel;
    if close_paren <= cursor {
        return None;
    }
    let type_info = object_text[cursor..close_paren].trim();
    if type_info.is_empty() {
        return None;
    }
    Some(type_info.to_string())
}

fn find_table_field_type_in_object_text(object_text: &str, field_name: &str) -> Option<String> {
    let quoted_marker = format!("; \"{}\";", field_name);
    let mut search_from = 0usize;
    while let Some(pos) = object_text[search_from..].find(&quoted_marker) {
        let marker_start = search_from + pos;
        let marker_end = marker_start + quoted_marker.len();
        if let Some(type_info) = parse_field_type_after_marker(object_text, marker_end) {
            return Some(type_info);
        }
        search_from = marker_end;
    }

    if is_simple_identifier_name(field_name) {
        let bare_marker = format!("; {};", field_name);
        let mut search_from = 0usize;
        while let Some(pos) = object_text[search_from..].find(&bare_marker) {
            let marker_start = search_from + pos;
            let marker_end = marker_start + bare_marker.len();
            if let Some(type_info) = parse_field_type_after_marker(object_text, marker_end) {
                return Some(type_info);
            }
            search_from = marker_end;
        }
    }

    None
}

pub(crate) fn find_table_field_type(
    state: &WorldState,
    table_name: &str,
    field_name: &str,
) -> Option<String> {
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

            for child in &symbol.children {
                if !matches!(child.kind, AlSymbolKind::Field)
                    || !child.name.eq_ignore_ascii_case(field_name)
                {
                    continue;
                }

                let type_info = child.type_info.as_deref()?;
                return Some(type_info.to_string());
            }

            let object_start = symbol.start_byte.min(source.len());
            let object_end = symbol.end_byte.min(source.len());
            if object_start < object_end {
                let object_text = &source[object_start..object_end];
                if let Some(type_info) =
                    find_table_field_type_in_object_text(object_text, field_name)
                {
                    return Some(type_info);
                }
            }
        }
    }
    None
}

fn completion_item_kind(kind: AlSymbolKind) -> CompletionItemKind {
    match kind {
        AlSymbolKind::Object(_) => CompletionItemKind::CLASS,
        AlSymbolKind::Procedure => CompletionItemKind::FUNCTION,
        AlSymbolKind::Trigger => CompletionItemKind::EVENT,
        AlSymbolKind::Variable => CompletionItemKind::VARIABLE,
        AlSymbolKind::Parameter => CompletionItemKind::VARIABLE,
        AlSymbolKind::Field => CompletionItemKind::FIELD,
        AlSymbolKind::Key => CompletionItemKind::KEYWORD,
        AlSymbolKind::EnumValue => CompletionItemKind::ENUM_MEMBER,
    }
}

pub(crate) struct MemberAccessTarget {
    pub object_kind: String,
    pub object_name: String,
    pub member_name: String,
    pub is_method_call: bool,
}

fn dot_target_at_offset(
    state: &WorldState,
    doc: &DocumentState,
    source: &str,
    byte_offset: usize,
) -> Option<(String, String)> {
    if let Some((object_node, scope_byte)) =
        object_node_for_member_access(&doc.tree, source, byte_offset)
    {
        if let Some(target) =
            resolve_object_type_from_expression(state, doc, source, object_node, scope_byte, 0)
        {
            return Some(target);
        }
    }

    let (object_start, object_name) = parse_object_before_trailing_dot(source, byte_offset)?;
    resolve_object_type_from_symbol_name(doc, source, &object_name, object_start)
}

pub(crate) fn member_access_target_at_offset(
    state: &WorldState,
    doc: &DocumentState,
    source: &str,
    byte_offset: usize,
) -> Option<MemberAccessTarget> {
    let node = node_at_offset(&doc.tree, byte_offset)?;
    if !matches!(node.kind(), "identifier" | "quoted_identifier") {
        return None;
    }

    let parent = node.parent()?;
    let (member_node, object_node, is_method_call) = match parent.kind() {
        "method_call" => (
            parent.child_by_field_name("method")?,
            parent.child_by_field_name("object")?,
            true,
        ),
        "member_access" => (
            parent.child_by_field_name("member")?,
            parent.child_by_field_name("object")?,
            false,
        ),
        _ => return None,
    };
    if member_node.id() != node.id() {
        return None;
    }

    let (object_kind, object_name) = resolve_object_type_from_expression(
        state,
        doc,
        source,
        object_node,
        parent.start_byte(),
        0,
    )?;
    Some(MemberAccessTarget {
        object_kind,
        object_name,
        member_name: extract_name(node, source),
        is_method_call,
    })
}

fn object_node_for_member_access<'a>(
    tree: &'a tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<(tree_sitter::Node<'a>, usize)> {
    for probe in [
        byte_offset.min(source.len()),
        byte_offset.saturating_sub(1).min(source.len()),
    ] {
        if let Some(node) = node_at_offset(tree, probe) {
            let mut current = Some(node);
            while let Some(n) = current {
                if matches!(n.kind(), "member_access" | "method_call") {
                    let object_node = n.child_by_field_name("object")?;
                    return Some((object_node, n.start_byte()));
                }
                current = n.parent();
            }
        }
    }
    None
}

fn parse_object_before_trailing_dot(source: &str, byte_offset: usize) -> Option<(usize, String)> {
    let bytes = source.as_bytes();
    if bytes.is_empty() || byte_offset == 0 {
        return None;
    }

    let mut dot = byte_offset.min(bytes.len());
    while dot > 0 && bytes[dot - 1].is_ascii_whitespace() {
        dot -= 1;
    }
    if dot == 0 || bytes[dot - 1] != b'.' {
        return None;
    }

    let mut left = dot - 1;
    while left > 0 && bytes[left - 1].is_ascii_whitespace() {
        left -= 1;
    }
    if left == 0 {
        return None;
    }

    if bytes[left - 1] == b')' {
        let mut depth = 1usize;
        let mut cursor = left - 1;
        while cursor > 0 {
            cursor -= 1;
            match bytes[cursor] {
                b')' => depth += 1,
                b'(' => {
                    depth -= 1;
                    if depth == 0 {
                        return parse_identifier_segment(source, cursor);
                    }
                }
                _ => {}
            }
        }
        return None;
    }

    parse_identifier_segment(source, left)
}

fn resolve_object_type_from_symbol_name(
    doc: &DocumentState,
    source: &str,
    object_name: &str,
    scope_byte: usize,
) -> Option<(String, String)> {
    if object_name.eq_ignore_ascii_case("codeunit") {
        return Some(("codeunit_builtin".to_string(), "CODEUNIT".to_string()));
    }
    if object_name.eq_ignore_ascii_case("page") {
        return Some(("page_builtin".to_string(), "PAGE".to_string()));
    }
    if object_name.eq_ignore_ascii_case("report") {
        return Some(("report_builtin".to_string(), "REPORT".to_string()));
    }
    if object_name.eq_ignore_ascii_case("xmlport") {
        return Some(("xmlport_builtin".to_string(), "XMLPORT".to_string()));
    }
    if object_name.eq_ignore_ascii_case("query") {
        return Some(("query_builtin".to_string(), "QUERY".to_string()));
    }

    let symbol = doc
        .symbol_table
        .lookup_in_scope(object_name, scope_byte)
        .into_iter()
        .find(|sym| {
            matches!(
                sym.kind,
                AlSymbolKind::Variable | AlSymbolKind::Parameter | AlSymbolKind::Procedure
            )
        });
    let type_info_owned = symbol
        .and_then(|sym| sym.type_info.clone())
        .or_else(|| fallback_procedure_return_type_from_source(source, object_name));
    let type_info = type_info_owned.as_deref()?;
    let (object_kind, object_name) = extract_type_object_name(type_info)?;
    Some((object_kind.to_string(), object_name.to_string()))
}

pub(crate) fn resolve_object_type_from_expression(
    state: &WorldState,
    doc: &DocumentState,
    source: &str,
    expr_node: tree_sitter::Node<'_>,
    scope_byte: usize,
    depth: u8,
) -> Option<(String, String)> {
    if depth > 8 {
        return None;
    }

    let expr_node = unwrap_primary_expression(expr_node);
    match expr_node.kind() {
        "identifier" | "quoted_identifier" => resolve_object_type_from_symbol_name(
            doc,
            source,
            &extract_name(expr_node, source),
            scope_byte,
        ),
        "function_call" => {
            let function_node = expr_node
                .child_by_field_name("function")
                .or_else(|| expr_node.child_by_field_name("name"))?;
            let function_name = extract_name(function_node, source);
            resolve_object_type_from_symbol_name(
                doc,
                source,
                &function_name,
                expr_node.start_byte(),
            )
        }
        "method_call" => {
            let object_node = expr_node.child_by_field_name("object")?;
            let method_node = expr_node.child_by_field_name("method")?;
            let method_name = extract_name(method_node, source);
            let (object_kind, object_name) = resolve_object_type_from_expression(
                state,
                doc,
                source,
                object_node,
                expr_node.start_byte(),
                depth + 1,
            )?;
            let member_type =
                find_object_member_type(state, &object_kind, &object_name, &method_name, true)?;
            let (member_kind, member_name) = extract_type_object_name(&member_type)?;
            Some((member_kind.to_string(), member_name.to_string()))
        }
        "member_access" => {
            let object_node = expr_node.child_by_field_name("object")?;
            let member_node = expr_node.child_by_field_name("member")?;
            let member_name = extract_name(member_node, source);
            let (object_kind, object_name) = resolve_object_type_from_expression(
                state,
                doc,
                source,
                object_node,
                expr_node.start_byte(),
                depth + 1,
            )?;
            let member_type =
                find_object_member_type(state, &object_kind, &object_name, &member_name, false)?;
            let (member_kind, member_name) = extract_type_object_name(&member_type)?;
            Some((member_kind.to_string(), member_name.to_string()))
        }
        _ => None,
    }
}

fn fallback_procedure_return_type_from_source(
    source: &str,
    procedure_name: &str,
) -> Option<String> {
    let lower = source.to_ascii_lowercase();
    let name_lower = procedure_name.to_ascii_lowercase();
    let mut cursor = 0usize;

    while cursor < lower.len() {
        let Some(proc_rel) = lower[cursor..].find("procedure") else {
            break;
        };
        let proc_start = cursor + proc_rel;
        let line_end = source[proc_start..]
            .find('\n')
            .map(|i| proc_start + i)
            .unwrap_or(source.len());
        let signature = &source[proc_start..line_end];
        let signature_lower = signature.to_ascii_lowercase();
        if let Some(name_pos) = signature_lower.find(&name_lower) {
            let after_name = &signature[name_pos + procedure_name.len()..];
            if let Some(close_paren) = after_name.find(')') {
                let after_params = &after_name[close_paren + 1..];
                if let Some(colon) = after_params.find(':') {
                    let return_type = after_params[colon + 1..]
                        .trim()
                        .trim_end_matches(';')
                        .trim();
                    if !return_type.is_empty() {
                        return Some(return_type.to_string());
                    }
                }
            }
        }
        cursor = proc_start + "procedure".len();
    }

    None
}

fn find_object_member_type(
    state: &WorldState,
    object_kind: &str,
    object_name: &str,
    member_name: &str,
    require_procedure: bool,
) -> Option<String> {
    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            let AlSymbolKind::Object(kind) = symbol.kind else {
                continue;
            };
            if !kind.label().eq_ignore_ascii_case(object_kind)
                || !symbol.name.eq_ignore_ascii_case(object_name)
            {
                continue;
            }
            for child in &symbol.children {
                if !child.name.eq_ignore_ascii_case(member_name) {
                    continue;
                }
                if require_procedure && !matches!(child.kind, AlSymbolKind::Procedure) {
                    continue;
                }
                if let Some(type_info) = child.type_info.as_deref() {
                    return Some(type_info.to_string());
                }
            }

            if !require_procedure && object_kind.eq_ignore_ascii_case("table") {
                if let Some(type_info) = find_table_field_type(state, object_name, member_name) {
                    return Some(type_info);
                }
            }
        }
    }
    None
}

fn builtin_record_methods() -> &'static [&'static str] {
    &[
        "AddLink",
        "Ascending",
        "CalcFields",
        "CalcSums",
        "ChangeCompany",
        "Copy",
        "CopyFilters",
        "Count",
        "Delete",
        "DeleteAll",
        "DeleteLinks",
        "Find",
        "FindFirst",
        "FindLast",
        "FindSet",
        "FieldCaption",
        "Get",
        "GetByRecordId",
        "GetBySystemId",
        "GetFilters",
        "GetPosition",
        "HasFilter",
        "Init",
        "Insert",
        "IsEmpty",
        "LockTable",
        "Mark",
        "MarkedOnly",
        "Modify",
        "ModifyAll",
        "Next",
        "ReadIsolation",
        "RecordId",
        "Rename",
        "Reset",
        "SetAutoCalcFields",
        "SetCurrentKey",
        "SetFilter",
        "SetLoadFields",
        "SetPosition",
        "SetRange",
        "SetRecFilter",
        "TableCaption",
        "TableName",
        "TestField",
        "TransferFields",
        "Validate",
    ]
}

fn builtin_codeunit_methods() -> &'static [&'static str] {
    &["Run"]
}

fn builtin_page_methods() -> &'static [&'static str] {
    &[
        "Run",
        "RunModal",
        "SetRecord",
        "GetRecord",
        "Update",
        "Close",
        "SetTableView",
        "LookupMode",
    ]
}

fn builtin_report_methods() -> &'static [&'static str] {
    &[
        "Run",
        "RunModal",
        "Print",
        "SaveAs",
        "Execute",
        "SetTableView",
        "SetRecord",
        "UseRequestPage",
    ]
}

fn builtin_xmlport_methods() -> &'static [&'static str] {
    &["Run", "Import", "Export", "SetSource", "SetDestination"]
}

fn builtin_query_methods() -> &'static [&'static str] {
    &["Open", "Read", "Close", "SaveAsXml", "SetFilter"]
}

fn builtin_enum_methods() -> &'static [&'static str] {
    &["Names", "Ordinals"]
}

fn builtin_list_methods() -> &'static [&'static str] {
    &[
        "Add", "AddRange", "Contains", "Count", "Get", "GetRange", "IndexOf", "Insert", "Remove",
        "RemoveAt", "Set",
    ]
}

fn builtin_dictionary_methods() -> &'static [&'static str] {
    &[
        "Add",
        "ContainsKey",
        "ContainsValue",
        "Count",
        "Get",
        "Keys",
        "Remove",
        "Set",
        "Values",
    ]
}

fn builtin_methods_for_object_kind(object_kind: &str) -> Option<&'static [&'static str]> {
    match object_kind.to_ascii_lowercase().as_str() {
        "table" => Some(builtin_record_methods()),
        "codeunit" => Some(builtin_codeunit_methods()),
        "page" | "page_builtin" => Some(builtin_page_methods()),
        "report" | "report_builtin" => Some(builtin_report_methods()),
        "xmlport" | "xmlport_builtin" => Some(builtin_xmlport_methods()),
        "query" | "query_builtin" => Some(builtin_query_methods()),
        "enum" | "enum_builtin" => Some(builtin_enum_methods()),
        "list" => Some(builtin_list_methods()),
        "dictionary" => Some(builtin_dictionary_methods()),
        "codeunit_builtin" => Some(builtin_codeunit_methods()),
        _ => None,
    }
}

/// Extract the word prefix before the cursor position.
fn extract_prefix(source: &str, byte_offset: usize) -> &str {
    let before = &source[..byte_offset.min(source.len())];
    let start = before
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    &before[start..]
}

fn property_scope_for_container(kind: &str) -> Option<&'static str> {
    match kind {
        "table_declaration" | "table_extension_declaration" => Some("table"),
        "codeunit_declaration" => Some("codeunit"),
        "page_declaration" | "page_extension_declaration" => Some("page"),
        "report_declaration" => Some("report"),
        "query_declaration" => Some("query"),
        "xmlport_declaration" => Some("xmlport"),
        "interface_declaration" => Some("interface"),
        "field_declaration" | "page_field" => Some("field"),
        "key_declaration" => Some("key"),
        "enum_declaration" | "enum_extension_declaration" => Some("enum"),
        "enum_value_declaration" => Some("enumvalue"),
        _ => None,
    }
}

fn line_prefix_at_offset(source: &str, byte_offset: usize) -> &str {
    let cursor = byte_offset.min(source.len());
    let line_start = source[..cursor].rfind('\n').map(|i| i + 1).unwrap_or(0);
    &source[line_start..cursor]
}

fn is_relation_semantic_property(name: &str) -> bool {
    name.eq_ignore_ascii_case("tablerelation") || name.eq_ignore_ascii_case("calcformula")
}

fn property_name_prefix_context(line_prefix: &str) -> bool {
    let trimmed = line_prefix.trim_start();
    if trimmed.is_empty() {
        return true;
    }
    if trimmed.contains('=') {
        return false;
    }
    if trimmed.contains(':') || trimmed.contains('(') || trimmed.contains("::") {
        return false;
    }
    trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '"')
}

fn property_value_context_from_line(source: &str, byte_offset: usize) -> Option<String> {
    let line_prefix = line_prefix_at_offset(source, byte_offset);
    let eq_index = line_prefix.rfind('=')?;
    let left = line_prefix[..eq_index].trim_end();
    if left.is_empty() {
        return None;
    }
    let (_, property_name) = parse_identifier_segment(left, left.len())?;
    Some(property_name)
}

fn property_name_for_node(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    let mut current = Some(node);
    while let Some(n) = current {
        if n.kind() == "property" {
            let name_node = n.child_by_field_name("name")?;
            return Some(extract_name(name_node, source));
        }
        current = n.parent();
    }
    None
}

fn after_equals_in_node_slice(
    node: tree_sitter::Node<'_>,
    source: &str,
    byte_offset: usize,
) -> bool {
    let end = byte_offset.min(node.end_byte()).max(node.start_byte());
    let snippet = &source[node.start_byte()..end];
    snippet.contains('=')
}

fn where_value_context_from_line(source: &str, byte_offset: usize) -> bool {
    let line_prefix = line_prefix_at_offset(source, byte_offset);
    let lower = line_prefix.to_ascii_lowercase();

    let where_pos = lower.rfind("where(").or_else(|| lower.rfind("where ("));
    let Some(where_pos) = where_pos else {
        return false;
    };

    let bytes = line_prefix.as_bytes();
    let mut cursor = bytes.len();
    while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
        cursor -= 1;
    }

    let mut idx = cursor;
    while idx > 0 {
        let b = bytes[idx - 1];
        if b == b'=' {
            return idx - 1 > where_pos;
        }
        if b == b',' || b == b'(' {
            return false;
        }
        idx -= 1;
    }

    false
}

fn where_value_completion_context_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> bool {
    for probe in [
        byte_offset.min(source.len()),
        byte_offset.saturating_sub(1).min(source.len()),
    ] {
        let Some(node) = node_at_offset(tree, probe) else {
            continue;
        };
        let Some(property_name) = property_name_for_node(node, source) else {
            continue;
        };
        if !is_relation_semantic_property(&property_name) {
            continue;
        }

        if find_ancestor_of_kind(node, "where_value_expression").is_some() {
            return true;
        }

        if let Some(where_condition) = find_ancestor_of_kind(node, "where_condition") {
            if after_equals_in_node_slice(where_condition, source, probe) {
                return true;
            }
        }
    }

    where_value_context_from_line(source, byte_offset)
}

fn property_completion_context_from_node(
    node: tree_sitter::Node<'_>,
    source: &str,
    byte_offset: usize,
) -> Option<PropertyCompletionContext> {
    if let Some(property_node) = find_ancestor_of_kind(node, "property") {
        let name_node = property_node.child_by_field_name("name")?;
        let property_name = extract_name(name_node, source);
        if let Some(value_node) = property_node.child_by_field_name("value") {
            if byte_offset >= value_node.start_byte() {
                return Some(PropertyCompletionContext::Values { property_name });
            }
        }
        let snippet_end = byte_offset
            .min(property_node.end_byte())
            .max(property_node.start_byte());
        let snippet = &source[property_node.start_byte()..snippet_end];
        if snippet.contains('=') {
            return Some(PropertyCompletionContext::Values { property_name });
        }
        return None;
    }

    if let Some(scope) = property_scope_at_node(node) {
        let line_prefix = line_prefix_at_offset(source, byte_offset);
        if property_name_prefix_context(line_prefix) {
            return Some(PropertyCompletionContext::Names { scope });
        }
    }

    None
}

fn property_scope_at_node(node: tree_sitter::Node<'_>) -> Option<&'static str> {
    let mut current = Some(node);
    while let Some(n) = current {
        match n.kind() {
            "procedure_declaration" | "trigger_declaration" | "var_section" | "block" => {
                return None;
            }
            _ => {}
        }

        if let Some(scope) = property_scope_for_container(n.kind()) {
            return Some(scope);
        }

        current = n.parent();
    }
    None
}

fn property_completion_context_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<PropertyCompletionContext> {
    let line_value_context = property_value_context_from_line(source, byte_offset);

    for probe in [
        byte_offset.min(source.len()),
        byte_offset.saturating_sub(1).min(source.len()),
    ] {
        if let Some(node) = node_at_offset(tree, probe) {
            if let Some(ctx) = property_completion_context_from_node(node, source, probe) {
                if line_value_context.is_some()
                    && matches!(ctx, PropertyCompletionContext::Names { .. })
                {
                    continue;
                }
                return Some(ctx);
            }
        }
    }

    if let Some(property_name) = line_value_context {
        return Some(PropertyCompletionContext::Values { property_name });
    }

    None
}

pub(crate) fn enum_context_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<EnumContext> {
    for probe in [
        byte_offset.min(source.len()),
        byte_offset.saturating_sub(1).min(source.len()),
    ] {
        if let Some(node) = node_at_offset(tree, probe) {
            let mut current = Some(node);
            while let Some(n) = current {
                if n.kind() == "qualified_enum_value" {
                    if let Some(enum_node) = n.child_by_field_name("enum") {
                        if let Some(ctx) = enum_context_from_qualifier_node(enum_node, source) {
                            return Some(ctx);
                        }
                    }
                    break;
                }
                current = n.parent();
            }
        }
    }

    enum_context_from_source_fallback(source, byte_offset)
}

pub(crate) fn enum_value_usage_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<EnumValueUsage> {
    for probe in [
        byte_offset.min(source.len()),
        byte_offset.saturating_sub(1).min(source.len()),
    ] {
        let Some(node) = node_at_offset(tree, probe) else {
            continue;
        };
        let Some(qev) = find_ancestor_of_kind(node, "qualified_enum_value") else {
            continue;
        };
        if let Some(usage) = enum_value_usage_from_node(qev, source) {
            return Some(usage);
        }
    }
    None
}

pub(crate) fn enum_value_usages_in_tree(
    tree: &tree_sitter::Tree,
    source: &str,
) -> Vec<EnumValueUsage> {
    let mut usages = Vec::new();
    collect_enum_value_usages(tree.root_node(), source, &mut usages);
    usages
}

pub(crate) fn enum_value_declaration_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<(String, String)> {
    let node = node_at_offset(tree, byte_offset)?;
    if !matches!(node.kind(), "identifier" | "quoted_identifier") {
        return None;
    }

    let decl = node.parent()?;
    if decl.kind() != "enum_value_declaration" {
        return None;
    }
    let name_node = decl.child_by_field_name("name")?;
    if name_node.id() != node.id() {
        return None;
    }

    let enum_name = enum_name_from_enum_value_declaration(decl, source)?;
    let value_name = extract_name(node, source);
    Some((enum_name, value_name))
}

pub(crate) fn enum_value_target_at_offset(
    state: &WorldState,
    uri: &lsp_types::Url,
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<(String, String)> {
    if let Some(usage) = enum_value_usage_at_offset(tree, source, byte_offset) {
        let enum_name = resolve_enum_name_from_context(state, uri, &usage.context)?;
        return Some((enum_name, usage.value_name));
    }

    enum_value_declaration_at_offset(tree, source, byte_offset)
}

fn enum_context_from_qualifier_node(
    qualifier_node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<EnumContext> {
    let qualifier_node = unwrap_primary_expression(qualifier_node);
    match qualifier_node.kind() {
        "identifier" | "quoted_identifier" => Some(EnumContext::Direct {
            qualifier_name: extract_name(qualifier_node, source),
            qualifier_byte_offset: qualifier_node.start_byte(),
        }),
        "member_access" => {
            let object = unwrap_primary_expression(qualifier_node.child_by_field_name("object")?);
            let member = unwrap_primary_expression(qualifier_node.child_by_field_name("member")?);
            let (object_byte_offset, _object_name) = extract_identifier_info(object, source)?;
            let (_member_byte_offset, member_name) = extract_identifier_info(member, source)?;
            Some(EnumContext::Member {
                object_byte_offset,
                member_name,
            })
        }
        "enum_qualified_name" => {
            let enum_name_node = qualifier_node.child_by_field_name("name")?;
            Some(EnumContext::Direct {
                qualifier_name: extract_name(enum_name_node, source),
                qualifier_byte_offset: enum_name_node.start_byte(),
            })
        }
        _ => None,
    }
}

fn unwrap_primary_expression(mut node: tree_sitter::Node<'_>) -> tree_sitter::Node<'_> {
    while node.kind() == "primary_expression" {
        let mut cursor = node.walk();
        let next = node.named_children(&mut cursor).next();
        match next {
            Some(child) => node = child,
            None => break,
        }
    }
    node
}

fn extract_identifier_info(node: tree_sitter::Node<'_>, source: &str) -> Option<(usize, String)> {
    let node = unwrap_primary_expression(node);
    if matches!(node.kind(), "identifier" | "quoted_identifier") {
        return Some((node.start_byte(), extract_name(node, source)));
    }
    None
}

fn enum_context_from_source_fallback(source: &str, byte_offset: usize) -> Option<EnumContext> {
    let bytes = source.as_bytes();
    if bytes.is_empty() || byte_offset == 0 {
        return None;
    }

    let mut cursor = byte_offset.min(bytes.len());
    while cursor > 0 && matches!(bytes[cursor - 1], b';' | b')' | b']' | b',') {
        cursor -= 1;
    }
    while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
        cursor -= 1;
    }
    while cursor > 0 && is_ident_char(bytes[cursor - 1]) {
        cursor -= 1;
    }
    while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
        cursor -= 1;
    }

    if cursor < 2 || bytes[cursor - 1] != b':' || bytes[cursor - 2] != b':' {
        return None;
    }

    let mut qualifier_end = cursor - 2;
    while qualifier_end > 0 && bytes[qualifier_end - 1].is_ascii_whitespace() {
        qualifier_end -= 1;
    }
    if qualifier_end == 0 {
        return None;
    }

    let (qualifier_start, qualifier_name) = parse_identifier_segment(source, qualifier_end)?;
    let mut left = qualifier_start;
    while left > 0 && bytes[left - 1].is_ascii_whitespace() {
        left -= 1;
    }
    if left > 0 && bytes[left - 1] == b'.' {
        let (object_start, _object_name) = parse_identifier_segment(source, left - 1)?;
        return Some(EnumContext::Member {
            object_byte_offset: object_start,
            member_name: qualifier_name,
        });
    }

    Some(EnumContext::Direct {
        qualifier_name,
        qualifier_byte_offset: qualifier_start,
    })
}

fn is_ident_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn parse_identifier_segment(source: &str, end: usize) -> Option<(usize, String)> {
    let bytes = source.as_bytes();
    if end == 0 || end > bytes.len() {
        return None;
    }

    let mut cursor = end;
    while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
        cursor -= 1;
    }
    if cursor == 0 {
        return None;
    }

    if bytes[cursor - 1] == b'"' {
        let mut start_quote = cursor - 1;
        while start_quote > 0 {
            start_quote -= 1;
            if bytes[start_quote] == b'"' {
                let name = &source[start_quote + 1..cursor - 1];
                if !name.is_empty() {
                    return Some((start_quote, name.to_string()));
                }
                return None;
            }
        }
        return None;
    }

    let mut start = cursor;
    while start > 0 && is_ident_char(bytes[start - 1]) {
        start -= 1;
    }
    if start == cursor {
        return None;
    }

    Some((start, source[start..cursor].to_string()))
}

fn enum_value_usage_from_node(node: tree_sitter::Node<'_>, source: &str) -> Option<EnumValueUsage> {
    let enum_node = node.child_by_field_name("enum")?;
    let value_node = node.child_by_field_name("value")?;
    let context = enum_context_from_qualifier_node(enum_node, source)?;
    let value_name = extract_name(value_node, source);
    Some(EnumValueUsage {
        context,
        value_name,
        start: value_node.start_position(),
        end: value_node.end_position(),
    })
}

fn collect_enum_value_usages(
    node: tree_sitter::Node<'_>,
    source: &str,
    usages: &mut Vec<EnumValueUsage>,
) {
    if node.kind() == "qualified_enum_value" {
        if let Some(usage) = enum_value_usage_from_node(node, source) {
            usages.push(usage);
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_enum_value_usages(child, source, usages);
    }
}

fn find_ancestor_of_kind<'a>(
    node: tree_sitter::Node<'a>,
    kind: &str,
) -> Option<tree_sitter::Node<'a>> {
    let mut current = Some(node);
    while let Some(n) = current {
        if n.kind() == kind {
            return Some(n);
        }
        current = n.parent();
    }
    None
}

fn enum_name_from_enum_value_declaration(
    enum_value_decl: tree_sitter::Node<'_>,
    source: &str,
) -> Option<String> {
    let parent = enum_value_decl.parent()?;
    match parent.kind() {
        "enum_declaration" => first_object_name(parent, source),
        "enum_extension_declaration" => second_object_name(parent, source),
        _ => None,
    }
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
        return Some(names[1].clone());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Url};

    fn make_completion_params(uri: Url, line: u32, character: u32) -> CompletionParams {
        CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        }
    }

    fn items_from(resp: CompletionResponse) -> Vec<CompletionItem> {
        match resp {
            CompletionResponse::Array(items) => items,
            CompletionResponse::List(list) => list.items,
        }
    }

    fn cursor_after(source: &str, marker: &str) -> (u32, u32) {
        let end = source
            .find(marker)
            .map(|i| i + marker.len())
            .unwrap_or_else(|| panic!("marker not found: {marker}"));
        let line = source[..end].bytes().filter(|&b| b == b'\n').count() as u32;
        let line_start = source[..end].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let character = (end - line_start) as u32;
        (line, character)
    }

    #[test]
    fn test_completion_dot_record_variable_cross_doc() {
        let table_source = r#"table 18 Customer
{
    fields
    {
        field(1; Name; Text[100])
        {
        }
    }

    procedure HasAddress(): Boolean
    begin
    end;
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Cust: Record Customer;
    begin
        Cust.
    end;
}"#;
        let table_uri = Url::parse("file:///test/customer.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor after `Cust.` (line 6, col 13)
        let params = make_completion_params(codeunit_uri, 6, 13);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Name"),
            "expected table field in completion, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "HasAddress"),
            "expected table procedure in completion, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_record_variable_prefix_filter() {
        let table_source = r#"table 18 Customer
{
    fields
    {
        field(1; Name; Text[100])
        {
        }
        field(2; Number; Text[30])
        {
        }
    }
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Cust: Record Customer;
    begin
        Cust.Na
    end;
}"#;
        let table_uri = Url::parse("file:///test/customer.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor after `Na` in `Cust.Na` (line 6, col 15)
        let params = make_completion_params(codeunit_uri, 6, 15);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Name"),
            "expected Name in completion, got: {labels:?}"
        );
        assert!(
            !labels.iter().any(|l| l == "Number"),
            "did not expect Number with Na prefix, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_record_includes_builtin_methods() {
        let source = r#"table 18 Customer
{
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        Cust: Record Customer;
    begin
        Cust.Fi
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let params = make_completion_params(uri, 10, 15);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "FindFirst"),
            "expected built-in Record method FindFirst, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "FindSet"),
            "expected built-in Record method FindSet, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_record_includes_tablename() {
        let source = r#"table 18 Customer
{
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        Cust: Record Customer;
    begin
        Cust.Tab
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "Cust.Tab");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "TableName"),
            "expected built-in Record method TableName, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_list_includes_builtin_methods() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Tags: List of [Text];
    begin
        Tags.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "Tags.");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Add"),
            "expected List built-in method Add, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Count"),
            "expected List built-in method Count, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_dictionary_includes_builtin_methods() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Tags: Dictionary of [Text, Text];
    begin
        Tags.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "Tags.");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "ContainsKey"),
            "expected Dictionary built-in method ContainsKey, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Values"),
            "expected Dictionary built-in method Values, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_procedure_return_interface() {
        let source = r#"interface "Demo IFunctions"
{
    procedure NumberPad(KeyText: Text; KeyValue: Integer);
}

codeunit 50100 Test
{
    procedure DoWork()
    begin
        HelperFunc.
    end;

    local procedure HelperFunc(): Interface "Demo IFunctions";
    begin
        exit(ServiceLocator.CoreFunctions());
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "HelperFunc.");
        let doc = state.documents.get(&uri).unwrap();
        let source_text = doc.source();
        assert_eq!(
            fallback_procedure_return_type_from_source(&source_text, "HelperFunc"),
            Some("Interface \"Demo IFunctions\"".to_string())
        );
        let byte_offset =
            crate::convert::lsp_position_to_byte_offset(&doc.rope, Position { line, character })
                .unwrap();
        assert_eq!(source_text.as_bytes()[byte_offset - 1], b'.');
        assert_eq!(
            parse_object_before_trailing_dot(&source_text, byte_offset),
            Some((
                source_text.find("HelperFunc.").unwrap(),
                "HelperFunc".to_string()
            ))
        );
        assert_eq!(
            resolve_object_type_from_symbol_name(
                &doc,
                &source_text,
                "HelperFunc",
                source_text.find("HelperFunc.").unwrap()
            ),
            Some(("interface".to_string(), "Demo IFunctions".to_string()))
        );
        let dot_target = dot_target_at_offset(&state, &doc, &source_text, byte_offset);
        assert_eq!(
            dot_target,
            Some(("interface".to_string(), "Demo IFunctions".to_string()))
        );
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "NumberPad"),
            "expected interface member NumberPad, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_procedure_call_return_interface() {
        let source = r#"interface "Demo IFunctions"
{
    procedure NumberPad(KeyText: Text; KeyValue: Integer);
}

codeunit 50100 Test
{
    procedure DoWork()
    begin
        HelperFunc().
    end;

    local procedure HelperFunc(): Interface "Demo IFunctions";
    begin
        exit(ServiceLocator.CoreFunctions());
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "HelperFunc().");
        let doc = state.documents.get(&uri).unwrap();
        let source_text = doc.source();
        assert_eq!(
            fallback_procedure_return_type_from_source(&source_text, "HelperFunc"),
            Some("Interface \"Demo IFunctions\"".to_string())
        );
        let byte_offset =
            crate::convert::lsp_position_to_byte_offset(&doc.rope, Position { line, character })
                .unwrap();
        assert_eq!(source_text.as_bytes()[byte_offset - 1], b'.');
        assert_eq!(
            parse_object_before_trailing_dot(&source_text, byte_offset),
            Some((
                source_text.find("HelperFunc().").unwrap(),
                "HelperFunc".to_string()
            ))
        );
        assert_eq!(
            resolve_object_type_from_symbol_name(
                &doc,
                &source_text,
                "HelperFunc",
                source_text.find("HelperFunc().").unwrap()
            ),
            Some(("interface".to_string(), "Demo IFunctions".to_string()))
        );
        let dot_target = dot_target_at_offset(&state, &doc, &source_text, byte_offset);
        assert_eq!(
            dot_target,
            Some(("interface".to_string(), "Demo IFunctions".to_string()))
        );
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "NumberPad"),
            "expected interface member NumberPad, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_codeunit_builtin_run() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        CODEUNIT.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let params = make_completion_params(uri, 4, 17);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Run"),
            "expected CODEUNIT built-in Run method, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_page_builtin_runmodal() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        PAGE.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "PAGE.");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "RunModal"),
            "expected PAGE built-in RunModal method, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_page_builtin_settableview_and_lookupmode() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        PAGE.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "PAGE.");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "SetTableView"),
            "expected PAGE built-in SetTableView method, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "LookupMode"),
            "expected PAGE built-in LookupMode method, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_enum_builtin_names_and_ordinals() {
        let source = r#"enum 50100 "Dummy State"
{
    value(0; Open) { }
    value(1; Closed) { }
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        CurrentState: Enum "Dummy State";
    begin
        CurrentState.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "CurrentState.");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Names"),
            "expected Enum built-in Names member, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Ordinals"),
            "expected Enum built-in Ordinals member, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_report_builtin_runmodal() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        REPORT.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let params = make_completion_params(uri, 4, 15);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "RunModal"),
            "expected REPORT built-in RunModal method, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_dot_query_variable_includes_open_read_close() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Q: Query "Customer Sales";
    begin
        Q.
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let params = make_completion_params(uri, 6, 10);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Open"),
            "expected Query built-in Open method, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Read"),
            "expected Query built-in Read method, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Close"),
            "expected Query built-in Close method, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_enum_values_after_double_colon() {
        let source = r#"enum 50100 MyEnum
{
    value(0; First)
    {
    }
    value(1; Second)
    {
    }
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := MyEnum::;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `MyEnum::` on line 16, col 21
        let params = make_completion_params(uri, 16, 21);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "First"),
            "expected First enum value in completion, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Second"),
            "expected Second enum value in completion, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_enum_values_prefix_filter() {
        let source = r#"enum 50100 MyEnum
{
    value(0; First)
    {
    }
    value(1; Second)
    {
    }
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := MyEnum::Fi;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `Fi` on line 16, col 23
        let params = make_completion_params(uri, 16, 23);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "First"),
            "expected First enum value in completion, got: {labels:?}"
        );
        assert!(
            !labels.iter().any(|l| l == "Second"),
            "did not expect Second with Fi prefix, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_enum_values_for_record_field_qualified_access() {
        let enum_source = r#"enum 50100 "Document Type"
{
    value(0; Order)
    {
    }
    value(1; Invoice)
    {
    }
}"#;
        let table_source = r#"table 50100 "Sales Header"
{
    fields
    {
        field(1; "Document Type"; Enum "Document Type")
        {
        }
    }
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Rec: Record "Sales Header";
    begin
        Rec."Document Type"::;
    end;
}"#;

        let enum_uri = Url::parse("file:///test/enum.al").unwrap();
        let table_uri = Url::parse("file:///test/table.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(enum_uri, DocumentState::new(enum_source).unwrap());
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor after `::` on line 6, col 30
        let params = make_completion_params(codeunit_uri, 6, 30);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Order"),
            "expected Order enum value in completion, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Invoice"),
            "expected Invoice enum value in completion, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_enum_values_for_record_field_with_slash_after_tablerelation_if_else() {
        let enum_source = r#"enum 50100 "Dummy KDS-Trigger Sends"
{
    value(0; "On Item Added")
    {
    }
    value(1; Manual)
    {
    }
}"#;
        let table_source = r#"table 50100 "Dummy Hospitality Type"
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
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        HospType: Record "Dummy Hospitality Type";
    begin
        HospType."KDS Display/Printing"::;
    end;
}"#;

        let enum_uri = Url::parse("file:///test/enum.al").unwrap();
        let table_uri = Url::parse("file:///test/table.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(enum_uri, DocumentState::new(enum_source).unwrap());
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        let (line, character) = cursor_after(codeunit_source, "\"KDS Display/Printing\"::");
        let params = make_completion_params(codeunit_uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "On Item Added"),
            "expected enum value completion for quoted field with slash, got: {labels:?}"
        );
        assert!(
            labels.iter().any(|l| l == "Manual"),
            "expected second enum value completion, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_event_subscriber_object_reference() {
        let publisher_source = r#"codeunit 50100 "My Publisher"
{
    [IntegrationEvent(false, false)]
    procedure OnAfterPost()
    begin
    end;
}"#;
        let subscriber_source = r#"codeunit 50101 "My Subscriber"
{
    [EventSubscriber(ObjectType::Codeunit, )]
    local procedure HandleOnAfterPost()
    begin
    end;
}"#;
        let publisher_uri = Url::parse("file:///test/publisher.al").unwrap();
        let subscriber_uri = Url::parse("file:///test/subscriber.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(publisher_uri, DocumentState::new(publisher_source).unwrap());
        state.documents.insert(
            subscriber_uri.clone(),
            DocumentState::new(subscriber_source).unwrap(),
        );

        let (line, character) = cursor_after(subscriber_source, "ObjectType::Codeunit, ");
        let params = make_completion_params(subscriber_uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Codeunit::\"My Publisher\""),
            "expected codeunit object reference completion, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_event_subscriber_event_name() {
        let source = r#"codeunit 50100 "My Publisher"
{
    [IntegrationEvent(false, false)]
    procedure OnAfterPost()
    begin
    end;
}

codeunit 50101 "My Subscriber"
{
    [EventSubscriber(ObjectType::Codeunit, Codeunit::"My Publisher", '', '', false, false)]
    local procedure HandleOnAfterPost()
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "Codeunit::\"My Publisher\", '");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "OnAfterPost"),
            "expected event publisher completion, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_table_property_names_context() {
        let source = r#"table 50100 MyTable
{
    Da
}"#;
        let uri = Url::parse("file:///test/table.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `Da` on line 2
        let params = make_completion_params(uri, 2, 6);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "DataClassification"),
            "expected DataClassification property, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_codeunit_property_names_context() {
        let source = r#"codeunit 50100 MyCodeunit
{
    Si
}"#;
        let uri = Url::parse("file:///test/codeunit.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `Si` on line 2
        let params = make_completion_params(uri, 2, 6);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "SingleInstance"),
            "expected SingleInstance property, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_property_values_dataclassification() {
        let source = r#"table 50100 MyTable
{
    DataClassification = 
}"#;
        let uri = Url::parse("file:///test/table.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `=` on line 2
        let params = make_completion_params(uri, 2, 25);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "ToBeClassified"),
            "expected DataClassification values, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_property_values_singleinstance() {
        let source = r#"codeunit 50100 MyCodeunit
{
    SingleInstance = 
}"#;
        let uri = Url::parse("file:///test/codeunit.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `=` on line 2
        let params = make_completion_params(uri, 2, 21);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "true") && labels.iter().any(|l| l == "false"),
            "expected boolean values for SingleInstance, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_field_property_names_include_calcformula() {
        let source = r#"table 50100 MyTable
{
    fields
    {
        field(1; "Amount"; Decimal)
        {
            Ca
        }
    }
}"#;
        let uri = Url::parse("file:///test/table.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `Ca` on line 6
        let params = make_completion_params(uri, 6, 14);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "CalcFormula"),
            "expected CalcFormula property, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_property_values_calcformula_methods() {
        let source = r#"table 50100 MyTable
{
    fields
    {
        field(1; "Amount"; Decimal)
        {
            CalcFormula = 
        }
    }
}"#;
        let uri = Url::parse("file:///test/table.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor after `=` on line 6
        let params = make_completion_params(uri, 6, 26);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Lookup") && labels.iter().any(|l| l == "Sum"),
            "expected CalcFormula method values, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_property_values_tablerelation_tables() {
        let table_source = r#"table 50000 "Lookup Source"
{
}"#;
        let owner_source = r#"table 50100 MyTable
{
    fields
    {
        field(1; "Field Link"; Integer)
        {
            TableRelation = 
        }
    }
}"#;
        let table_uri = Url::parse("file:///test/source.al").unwrap();
        let owner_uri = Url::parse("file:///test/owner.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state
            .documents
            .insert(owner_uri.clone(), DocumentState::new(owner_source).unwrap());

        // Cursor after `=` on line 6
        let params = make_completion_params(owner_uri, 6, 28);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "Lookup Source"),
            "expected table names for TableRelation, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_where_value_expression_in_tablerelation() {
        let source = r#"table 50100 MyTable
{
    fields
    {
        field(1; "Field Link"; Integer)
        {
            TableRelation = Field."No." WHERE(TableNo = );
        }
    }
}"#;
        let uri = Url::parse("file:///test/table.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "TableNo = ");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "FIELD(")
                && labels.iter().any(|l| l == "CONST(")
                && labels.iter().any(|l| l == "FILTER("),
            "expected WHERE value expressions, got: {labels:?}"
        );
    }

    #[test]
    fn test_completion_where_value_expression_prefix_in_calcformula() {
        let source = r#"table 50100 MyTable
{
    fields
    {
        field(1; "Codeunit Name"; Text[30])
        {
            CalcFormula = Lookup(AllObj."Object Name" WHERE("Object Type" = C));
        }
    }
}"#;
        let uri = Url::parse("file:///test/table.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "\"Object Type\" = C");
        let params = make_completion_params(uri, line, character);
        let result = handle_completion(&state, params);
        assert!(result.is_some(), "expected completion result");
        let labels: Vec<String> = items_from(result.unwrap())
            .into_iter()
            .map(|i| i.label)
            .collect();
        assert!(
            labels.iter().any(|l| l == "CONST("),
            "expected CONST completion for C prefix, got: {labels:?}"
        );
        assert!(
            !labels.iter().any(|l| l == "FIELD(") && !labels.iter().any(|l| l == "FILTER("),
            "did not expect FIELD/FILTER for C prefix, got: {labels:?}"
        );
    }
}
