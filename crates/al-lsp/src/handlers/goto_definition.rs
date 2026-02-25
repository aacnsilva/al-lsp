use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use al_syntax::ast::{extract_name, AlSymbolKind};
use al_syntax::navigation::{
    extract_type_object_name, identifier_context_at_offset, resolve_at_offset,
};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::handlers::completion::{enum_value_target_at_offset, member_access_target_at_offset};
use crate::handlers::events::{
    event_invocation_target_at_offset, event_subscriber_context_at_offset, find_event_publishers,
};
use crate::state::WorldState;

fn to_definition_response(locations: Vec<Location>) -> Option<GotoDefinitionResponse> {
    if locations.is_empty() {
        return None;
    }
    if locations.len() == 1 {
        return Some(GotoDefinitionResponse::Scalar(
            locations.into_iter().next().unwrap(),
        ));
    }
    Some(GotoDefinitionResponse::Array(locations))
}

fn find_object_declarations(
    state: &WorldState,
    object_kind: &str,
    object_name: &str,
) -> Vec<Location> {
    let mut locations = Vec::new();
    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            if let AlSymbolKind::Object(kind) = symbol.kind {
                if kind.label().eq_ignore_ascii_case(object_kind)
                    && symbol.name.eq_ignore_ascii_case(object_name)
                {
                    locations.push(Location {
                        uri: entry.key().clone(),
                        range: ts_range_to_lsp_range(symbol.start_point, symbol.end_point),
                    });
                }
            }
        }
    }
    locations
}

fn find_enum_value_declarations(
    state: &WorldState,
    enum_name: &str,
    value_name: &str,
) -> Vec<Location> {
    let mut locations = Vec::new();
    for entry in state.documents.iter() {
        for symbol in &entry.value().symbol_table.symbols {
            let AlSymbolKind::Object(kind) = symbol.kind else {
                continue;
            };
            if kind.label() != "enum" || !symbol.name.eq_ignore_ascii_case(enum_name) {
                continue;
            }

            for child in &symbol.children {
                if matches!(child.kind, AlSymbolKind::EnumValue)
                    && child.name.eq_ignore_ascii_case(value_name)
                {
                    locations.push(Location {
                        uri: entry.key().clone(),
                        range: ts_range_to_lsp_range(child.start_point, child.end_point),
                    });
                }
            }
        }
    }
    locations
}

fn find_object_member_declarations(
    state: &WorldState,
    object_kind: &str,
    object_name: &str,
    member_name: &str,
    is_method_call: bool,
) -> Vec<Location> {
    let mut locations = Vec::new();
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
                if is_method_call && !matches!(child.kind, AlSymbolKind::Procedure) {
                    continue;
                }
                locations.push(Location {
                    uri: entry.key().clone(),
                    range: ts_range_to_lsp_range(child.start_point, child.end_point),
                });
            }
        }
    }
    locations
}

fn type_target_from_type_identifier(
    node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<(&'static str, String)> {
    if !matches!(node.kind(), "identifier" | "quoted_identifier") {
        return None;
    }
    let parent = node.parent()?;
    match parent.kind() {
        "record_type" => {
            let table = parent.child_by_field_name("table")?;
            if table.id() == node.id() {
                Some(("table", extract_name(node, source)))
            } else {
                None
            }
        }
        "codeunit_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("codeunit", extract_name(node, source)))
        }
        "page_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("page", extract_name(node, source)))
        }
        "report_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("report", extract_name(node, source)))
        }
        "query_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("query", extract_name(node, source)))
        }
        "xmlport_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("xmlport", extract_name(node, source)))
        }
        "enum_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("enum", extract_name(node, source)))
        }
        "interface_type" => {
            let name = parent.child_by_field_name("name")?;
            (name.id() == node.id()).then(|| ("interface", extract_name(node, source)))
        }
        _ => None,
    }
}

pub fn handle_goto_definition(
    state: &WorldState,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();
    let enum_target = enum_value_target_at_offset(state, &uri, &doc.tree, &source, byte_offset);

    if let Some((enum_name, value_name)) = enum_target {
        drop(doc);
        if let Some(resp) =
            to_definition_response(find_enum_value_declarations(state, &enum_name, &value_name))
        {
            return Some(resp);
        }
        let doc = state.documents.get(&uri)?;
        let source = doc.source();
        let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
        let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range,
        }));
    }

    if let Some(event_ctx) = event_subscriber_context_at_offset(&doc.tree, &source, byte_offset) {
        if event_ctx.arg_index == 1 {
            if let Some((object_kind, object_name)) = event_ctx.object_ref {
                drop(doc);
                if let Some(resp) = to_definition_response(find_object_declarations(
                    state,
                    &object_kind,
                    &object_name,
                )) {
                    return Some(resp);
                }
                let doc = state.documents.get(&uri)?;
                let source = doc.source();
                let resolved =
                    resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
                let range =
                    ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
                return Some(GotoDefinitionResponse::Scalar(Location {
                    uri: uri.clone(),
                    range,
                }));
            }
        }

        if event_ctx.arg_index >= 2 {
            if let Some(target) = event_ctx.target {
                drop(doc);
                let locations: Vec<Location> = find_event_publishers(state, &target)
                    .into_iter()
                    .map(|publisher| Location {
                        uri: publisher.uri,
                        range: ts_range_to_lsp_range(publisher.name_start, publisher.name_end),
                    })
                    .collect();
                if let Some(resp) = to_definition_response(locations) {
                    return Some(resp);
                }
                let doc = state.documents.get(&uri)?;
                let source = doc.source();
                let resolved =
                    resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
                let range =
                    ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
                return Some(GotoDefinitionResponse::Scalar(Location {
                    uri: uri.clone(),
                    range,
                }));
            }
        }
    }

    let invocation_target =
        event_invocation_target_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset);
    let doc = if let Some(target) = invocation_target {
        drop(doc);
        let locations: Vec<Location> = find_event_publishers(state, &target)
            .into_iter()
            .map(|publisher| Location {
                uri: publisher.uri,
                range: ts_range_to_lsp_range(publisher.name_start, publisher.name_end),
            })
            .collect();
        if let Some(resp) = to_definition_response(locations) {
            return Some(resp);
        }
        state.documents.get(&uri)?
    } else {
        doc
    };
    let source = doc.source();

    if let Some(target) = member_access_target_at_offset(state, &doc, &source, byte_offset) {
        drop(doc);
        if let Some(resp) = to_definition_response(find_object_member_declarations(
            state,
            &target.object_kind,
            &target.object_name,
            &target.member_name,
            target.is_method_call,
        )) {
            return Some(resp);
        }
        let doc = state.documents.get(&uri)?;
        let source = doc.source();
        let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
        let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range,
        }));
    }

    let id_ctx = identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset);

    // Only attempt interface→impl navigation when the cursor is on a Procedure identifier.
    let cursor_on_procedure = id_ctx
        .as_ref()
        .and_then(|ctx| ctx.symbol)
        .is_some_and(|sym| matches!(sym.kind, AlSymbolKind::Procedure));

    if cursor_on_procedure {
        // Check if cursor is on a procedure inside a codeunit that implements interfaces.
        // If so, navigate to the corresponding interface method definition.
        if let Some((iface_names, method_name)) =
            doc.symbol_table.implementation_procedure_at(byte_offset)
        {
            let iface_names: Vec<String> = iface_names.to_vec();
            let method_name = method_name.to_string();
            drop(doc); // Release the DashMap ref before iterating

            let mut locations = Vec::new();
            for entry in state.documents.iter() {
                let other_doc = entry.value();
                for iface_name in &iface_names {
                    if let Some(method_sym) = other_doc
                        .symbol_table
                        .find_interface_method(iface_name, &method_name)
                    {
                        locations.push(Location {
                            uri: entry.key().clone(),
                            range: ts_range_to_lsp_range(
                                method_sym.start_point,
                                method_sym.end_point,
                            ),
                        });
                    }
                }
            }

            if !locations.is_empty() {
                return to_definition_response(locations);
            }

            // If no interface method found, fall through to normal resolution.
            let doc = state.documents.get(&uri)?;
            let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
            let source = doc.source();
            let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
            let range =
                ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
            return Some(GotoDefinitionResponse::Scalar(Location {
                uri: uri.clone(),
                range,
            }));
        }
    }

    let mut symbol_type_target: Option<(String, String)> = None;
    let mut node_type_target: Option<(String, String)> = None;
    if let Some(ctx) = id_ctx {
        // Variable/parameter usage: jump to the declared object type (Record/Codeunit/etc).
        if let Some(sym) = ctx.symbol {
            if matches!(sym.kind, AlSymbolKind::Variable | AlSymbolKind::Parameter) {
                if let Some(type_info) = sym.type_info.as_deref() {
                    if let Some((object_kind, object_name)) = extract_type_object_name(type_info) {
                        symbol_type_target =
                            Some((object_kind.to_string(), object_name.to_string()));
                    }
                }
            }
        }

        // Cursor directly on a type name in a declaration: `Record "Customer"`, etc.
        if let Some((object_kind, object_name)) =
            type_target_from_type_identifier(ctx.node, &source)
        {
            node_type_target = Some((object_kind.to_string(), object_name));
        }
    }

    if let Some((object_kind, object_name)) = symbol_type_target {
        drop(doc);
        if let Some(resp) =
            to_definition_response(find_object_declarations(state, &object_kind, &object_name))
        {
            return Some(resp);
        }
        let doc = state.documents.get(&uri)?;
        let source = doc.source();
        let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
        let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range,
        }));
    }

    if let Some((object_kind, object_name)) = node_type_target {
        drop(doc);
        if let Some(resp) =
            to_definition_response(find_object_declarations(state, &object_kind, &object_name))
        {
            return Some(resp);
        }
        let doc = state.documents.get(&uri)?;
        let source = doc.source();
        let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
        let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range,
        }));
    }

    let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);

    Some(GotoDefinitionResponse::Scalar(Location {
        uri: uri.clone(),
        range,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Url};

    fn make_goto_params(uri: Url, line: u32, character: u32) -> GotoDefinitionParams {
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    fn locations_from(resp: GotoDefinitionResponse) -> Vec<Location> {
        match resp {
            GotoDefinitionResponse::Scalar(loc) => vec![loc],
            GotoDefinitionResponse::Array(locs) => locs,
            GotoDefinitionResponse::Link(_) => vec![],
        }
    }

    fn cursor_on(source: &str, marker: &str) -> (u32, u32) {
        let idx = source
            .find(marker)
            .unwrap_or_else(|| panic!("marker not found: {marker}"));
        let line = source[..idx].bytes().filter(|&b| b == b'\n').count() as u32;
        let line_start = source[..idx].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let character = (idx - line_start) as u32;
        (line, character)
    }

    #[test]
    fn test_goto_definition_impl_procedure_to_interface_method_cross_doc() {
        // Cursor on GetAddress in a codeunit that implements IAddressProvider.
        // Should navigate to the interface method definition in another document.
        let iface_source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}"#;
        let impl_source = r#"codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;
}"#;
        let iface_uri = Url::parse("file:///test/iface.al").unwrap();
        let impl_uri = Url::parse("file:///test/impl.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(iface_uri.clone(), DocumentState::new(iface_source).unwrap());
        state
            .documents
            .insert(impl_uri.clone(), DocumentState::new(impl_source).unwrap());

        // Cursor on "GetAddress" in the codeunit (line 2, col 14)
        let params = make_goto_params(impl_uri.clone(), 2, 14);
        let result = handle_goto_definition(&state, params);

        assert!(
            result.is_some(),
            "expected goto-definition to return a result"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.uri == iface_uri),
            "expected navigation to the interface document, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_impl_procedure_to_interface_method_same_doc() {
        // Interface and codeunit in the same document.
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "GetAddress" in the codeunit (line 7, col 14)
        let params = make_goto_params(uri.clone(), 7, 14);
        let result = handle_goto_definition(&state, params);

        assert!(
            result.is_some(),
            "expected goto-definition to return a result"
        );
        let locs = locations_from(result.unwrap());
        // Should navigate to the interface method on line 2
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected navigation to the interface method on line 2, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_non_interface_procedure_unchanged() {
        // Cursor on a regular procedure that is NOT an interface implementation.
        // Normal resolution should apply (no interface navigation).
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;

    procedure World()
    begin
        Hello();
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "Hello" in the call Hello() (line 8, col 8)
        let params = make_goto_params(uri.clone(), 8, 8);
        let result = handle_goto_definition(&state, params);

        assert!(
            result.is_some(),
            "expected goto-definition to resolve Hello"
        );
        let locs = locations_from(result.unwrap());
        // Should navigate to the Hello procedure declaration (line 2)
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected navigation to Hello declaration on line 2, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_record_variable_usage_to_table_cross_doc() {
        let table_source = r#"table 18 Customer
{
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Cust: Record Customer;
    begin
        Cust.FindFirst();
    end;
}"#;
        let table_uri = Url::parse("file:///test/customer.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(table_uri.clone(), DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor on `Cust` in `Cust.FindFirst()` (line 6, col 8)
        let params = make_goto_params(codeunit_uri.clone(), 6, 8);
        let result = handle_goto_definition(&state, params);
        assert!(result.is_some(), "expected goto-definition result");
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.uri == table_uri),
            "expected navigation to table declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_member_on_procedure_return_interface() {
        let source = r#"interface "Demo IFunctions"
{
    procedure NumberPad(KeyText: Text; KeyValue: Integer);
}

codeunit 50100 Test
{
    procedure DoWork()
    begin
        HelperFunc.NumberPad('9', 9);
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

        // Cursor on NumberPad in HelperFunc.NumberPad(...)
        let (line, character) = cursor_on(source, "NumberPad('9', 9)");
        let params = make_goto_params(uri.clone(), line, character);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to resolve interface method"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected navigation to interface method declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_member_on_procedure_call_return_interface() {
        let source = r#"interface "Demo IFunctions"
{
    procedure NumberPad(KeyText: Text; KeyValue: Integer);
}

codeunit 50100 Test
{
    procedure DoWork()
    begin
        HelperFunc().NumberPad('9', 9);
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

        let (line, character) = cursor_on(source, "NumberPad('9', 9)");
        let params = make_goto_params(uri, line, character);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to resolve interface method"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected navigation to interface method declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_on_type_name_in_record_declaration() {
        let table_source = r#"table 18 Customer
{
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Cust: Record Customer;
    begin
    end;
}"#;
        let table_uri = Url::parse("file:///test/customer.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(table_uri.clone(), DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor on `Customer` in `Cust: Record Customer;` (line 4, col 21)
        let params = make_goto_params(codeunit_uri.clone(), 4, 21);
        let result = handle_goto_definition(&state, params);
        assert!(result.is_some(), "expected goto-definition result");
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.uri == table_uri),
            "expected navigation to table declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_enum_value_from_qualified_usage() {
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
        X := MyEnum::First;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on `First` in `MyEnum::First` (line 16, col 21)
        let params = make_goto_params(uri, 16, 21);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return enum value declaration"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected navigation to enum value declaration on line 2, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_enum_value_from_record_field_qualified_usage() {
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
        if Rec."Document Type" = Rec."Document Type"::Order then;
    end;
}"#;
        let enum_uri = Url::parse("file:///test/enum.al").unwrap();
        let table_uri = Url::parse("file:///test/table.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(enum_uri.clone(), DocumentState::new(enum_source).unwrap());
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor on `Order` in `Rec."Document Type"::Order` (line 6, col 47)
        let params = make_goto_params(codeunit_uri, 6, 47);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return enum value declaration"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == enum_uri && l.range.start.line == 2),
            "expected navigation to enum value declaration in enum document, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_event_subscriber_event_name_to_publisher() {
        let publisher_source = r#"codeunit 50100 "My Publisher"
{
    [IntegrationEvent(false, false)]
    procedure OnAfterPost()
    begin
    end;
}"#;
        let subscriber_source = r#"codeunit 50101 "My Subscriber"
{
    [EventSubscriber(ObjectType::Codeunit, Codeunit::"My Publisher", 'OnAfterPost', '', false, false)]
    local procedure HandleOnAfterPost()
    begin
    end;
}"#;
        let publisher_uri = Url::parse("file:///test/publisher.al").unwrap();
        let subscriber_uri = Url::parse("file:///test/subscriber.al").unwrap();
        let state = WorldState::new();
        state.documents.insert(
            publisher_uri.clone(),
            DocumentState::new(publisher_source).unwrap(),
        );
        state.documents.insert(
            subscriber_uri.clone(),
            DocumentState::new(subscriber_source).unwrap(),
        );

        let (line, character) = cursor_on(subscriber_source, "OnAfterPost");
        let params = make_goto_params(subscriber_uri, line, character);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to resolve event publisher"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == publisher_uri && l.range.start.line == 3),
            "expected navigation to publisher procedure declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_event_invocation_to_publisher_declaration() {
        let source = r#"codeunit 50100 "My Publisher"
{
    [IntegrationEvent(false, false)]
    procedure OnAfterPost()
    begin
    end;

    procedure Raise()
    begin
        OnAfterPost();
    end;
}"#;
        let uri = Url::parse("file:///test/publisher.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_on(source, "OnAfterPost();");
        let params = make_goto_params(uri, line, character);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition from invocation to resolve event declaration"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.range.start.line == 3),
            "expected navigation to event declaration on line 3, got: {locs:?}"
        );
    }
}
