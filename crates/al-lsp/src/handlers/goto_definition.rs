use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use tree_sitter::Point;

use al_syntax::ast::{extract_name, AlSymbol, AlSymbolKind};
use al_syntax::navigation::{
    extract_type_object_name, identifier_context_at_offset, node_at_offset, resolve_at_offset,
};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::handlers::completion::{
    enum_value_target_at_offset, enum_value_usage_at_offset, member_access_target_at_offset,
    resolve_option_members_from_context,
};
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

fn find_inline_option_member_declaration(
    uri: &lsp_types::Url,
    doc: &al_syntax::document::DocumentState,
    source: &str,
    qualifier_byte_offset: usize,
    value_name: &str,
) -> Option<Location> {
    let ctx = identifier_context_at_offset(&doc.tree, source, &doc.symbol_table, qualifier_byte_offset)?;
    let sym = ctx.symbol?;
    let decl_start = sym.start_byte.min(source.len());
    let decl_end = sym.end_byte.min(source.len());
    if decl_start >= decl_end {
        return None;
    }

    let decl_source = &source[decl_start..decl_end];
    let option_pos_rel = decl_source.to_ascii_lowercase().find("option")?;
    let search_start = decl_start + option_pos_rel + "option".len();
    let search_source = &source[search_start..decl_end];

    let quoted = format!("\"{}\"", value_name);
    if let Some(rel) = search_source.find(&quoted) {
        let start = search_start + rel;
        let end = start + quoted.len();
        return Some(Location {
            uri: uri.clone(),
            range: ts_range_to_lsp_range(
                point_at_offset(source, start)?,
                point_at_offset(source, end)?,
            ),
        });
    }

    if let Some(rel) = search_source.find(value_name) {
        let start = search_start + rel;
        let end = start + value_name.len();
        return Some(Location {
            uri: uri.clone(),
            range: ts_range_to_lsp_range(
                point_at_offset(source, start)?,
                point_at_offset(source, end)?,
            ),
        });
    }

    None
}

fn point_at_offset(source: &str, offset: usize) -> Option<Point> {
    if offset > source.len() {
        return None;
    }
    let prefix = &source[..offset];
    let row = prefix.bytes().filter(|&b| b == b'\n').count();
    let column = prefix.rfind('\n').map(|idx| offset - idx - 1).unwrap_or(offset);
    Some(Point { row, column })
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
        let uri = entry.key().clone();
        let doc = entry.value();
        let source = doc.source();
        for symbol in &doc.symbol_table.symbols {
            let AlSymbolKind::Object(kind) = symbol.kind else {
                continue;
            };
            if !kind.label().eq_ignore_ascii_case(object_kind)
                || !symbol.name.eq_ignore_ascii_case(object_name)
            {
                continue;
            }

            let mut found_symbol_member = false;
            for child in &symbol.children {
                if !child.name.eq_ignore_ascii_case(member_name) {
                    continue;
                }
                if is_method_call && !matches!(child.kind, AlSymbolKind::Procedure) {
                    continue;
                }
                found_symbol_member = true;
                locations.push(Location {
                    uri: uri.clone(),
                    range: ts_range_to_lsp_range(child.start_point, child.end_point),
                });
            }

            if !is_method_call && kind.label().eq_ignore_ascii_case("table") && !found_symbol_member
            {
                if let Some(location) = find_table_field_location(&uri, source, symbol, member_name)
                {
                    locations.push(location);
                }
            }
        }
    }
    locations
}

fn find_table_field_location(
    uri: &lsp_types::Url,
    source: &str,
    table_symbol: &AlSymbol,
    field_name: &str,
) -> Option<Location> {
    let object_start = table_symbol.start_byte.min(source.len());
    let object_end = table_symbol.end_byte.min(source.len());
    if object_start >= object_end {
        return None;
    }
    let object_text = &source[object_start..object_end];
    let (field_start, field_end) = find_field_name_span_in_object_text(object_text, field_name)?;
    let start_point = point_from_relative_byte(table_symbol.start_point, object_text, field_start);
    let end_point = point_from_relative_byte(table_symbol.start_point, object_text, field_end);
    Some(Location {
        uri: uri.clone(),
        range: ts_range_to_lsp_range(start_point, end_point),
    })
}

fn find_field_name_span_in_object_text(
    object_text: &str,
    field_name: &str,
) -> Option<(usize, usize)> {
    let object_lower = object_text.to_ascii_lowercase();
    let bytes = object_text.as_bytes();
    let mut cursor = 0usize;
    while cursor < object_lower.len() {
        let Some(rel) = object_lower[cursor..].find("field") else {
            break;
        };
        let field_kw_start = cursor + rel;
        let mut open_paren = field_kw_start + "field".len();
        while open_paren < bytes.len() && bytes[open_paren].is_ascii_whitespace() {
            open_paren += 1;
        }
        if open_paren >= bytes.len() || bytes[open_paren] != b'(' {
            cursor = field_kw_start + "field".len();
            continue;
        }

        let field_body_start = open_paren + 1;
        let Some(first_sep_rel) = object_text[field_body_start..].find(';') else {
            break;
        };
        let second_arg_start = field_body_start + first_sep_rel + 1;
        let Some(second_sep_rel) = object_text[second_arg_start..].find(';') else {
            cursor = field_kw_start + "field".len();
            continue;
        };
        let second_arg_end = second_arg_start + second_sep_rel;

        let mut name_start = second_arg_start;
        while name_start < second_arg_end && bytes[name_start].is_ascii_whitespace() {
            name_start += 1;
        }
        let mut name_end = second_arg_end;
        while name_end > name_start && bytes[name_end - 1].is_ascii_whitespace() {
            name_end -= 1;
        }
        if name_end > name_start + 1 && bytes[name_start] == b'"' && bytes[name_end - 1] == b'"' {
            name_start += 1;
            name_end -= 1;
        }

        if name_end > name_start
            && object_text[name_start..name_end].eq_ignore_ascii_case(field_name)
        {
            return Some((name_start, name_end));
        }
        cursor = field_kw_start + "field".len();
    }
    None
}

fn point_from_relative_byte(base: Point, text: &str, rel: usize) -> Point {
    let mut point = base;
    let rel = rel.min(text.len());
    for &byte in text.as_bytes().iter().take(rel) {
        if byte == b'\n' {
            point.row += 1;
            point.column = 0;
        } else {
            point.column += 1;
        }
    }
    point
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

fn interface_target_from_implements_identifier(
    node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<(&'static str, String)> {
    if !matches!(node.kind(), "identifier" | "quoted_identifier") {
        return None;
    }
    let parent = node.parent()?;
    if parent.kind() == "implements_clause" {
        return Some(("interface", extract_name(node, source)));
    }
    None
}

#[derive(Debug, Clone)]
enum TableRelationNavTarget {
    Table(String),
    TableField {
        table_name: String,
        field_name: String,
    },
}

fn table_relation_nav_target_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<TableRelationNavTarget> {
    let node = node_at_offset(tree, byte_offset)?;
    if !matches!(node.kind(), "identifier" | "quoted_identifier") {
        return None;
    }

    if let Some(target) = table_relation_nav_target_from_relation_expression(node, source) {
        return Some(target);
    }
    table_relation_nav_target_from_property_value(node, source)
}

fn table_relation_nav_target_from_relation_expression(
    node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<TableRelationNavTarget> {
    let relation_target_expr = find_ancestor_of_kind(node, "table_relation_target_expression")?;
    let relation_node = relation_target_expr.child_by_field_name("relation")?;
    table_relation_nav_target_from_relation_node(node, relation_node, source)
}

fn table_relation_nav_target_from_property_value(
    node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<TableRelationNavTarget> {
    let property = find_ancestor_of_kind(node, "property")?;
    let prop_name_node = property.child_by_field_name("name")?;
    if !extract_name(prop_name_node, source).eq_ignore_ascii_case("TableRelation") {
        return None;
    }

    let value_node = property.child_by_field_name("value")?;
    if value_node.id() == node.id()
        && matches!(value_node.kind(), "identifier" | "quoted_identifier")
    {
        return Some(TableRelationNavTarget::Table(extract_name(
            value_node, source,
        )));
    }

    if value_node.kind() == "member_access" {
        return table_relation_nav_target_from_relation_node(node, value_node, source);
    }

    None
}

fn table_relation_nav_target_from_relation_node(
    node: tree_sitter::Node<'_>,
    relation_node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<TableRelationNavTarget> {
    match relation_node.kind() {
        "identifier" | "quoted_identifier" => (relation_node.id() == node.id())
            .then(|| TableRelationNavTarget::Table(extract_name(relation_node, source))),
        "member_access" => {
            let member_node = relation_node.child_by_field_name("member")?;
            let object_node =
                unwrap_primary_expression(relation_node.child_by_field_name("object")?);
            let table_name = table_name_from_member_access_object(object_node, source)?;
            if member_node.id() == node.id() {
                return Some(TableRelationNavTarget::TableField {
                    table_name,
                    field_name: extract_name(member_node, source),
                });
            }

            if matches!(object_node.kind(), "identifier" | "quoted_identifier")
                && object_node.id() == node.id()
            {
                return Some(TableRelationNavTarget::Table(table_name));
            }
            None
        }
        _ => None,
    }
}

fn table_name_from_member_access_object(
    node: tree_sitter::Node<'_>,
    source: &str,
) -> Option<String> {
    let node = unwrap_primary_expression(node);
    match node.kind() {
        "identifier" | "quoted_identifier" => Some(extract_name(node, source)),
        "member_access" => {
            let object = node.child_by_field_name("object")?;
            table_name_from_member_access_object(object, source)
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

fn find_ancestor_of_kind<'a>(
    mut node: tree_sitter::Node<'a>,
    kind: &str,
) -> Option<tree_sitter::Node<'a>> {
    loop {
        if node.kind() == kind {
            return Some(node);
        }
        node = node.parent()?;
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
    if let Some(usage) = enum_value_usage_at_offset(&doc.tree, &source, byte_offset) {
        if let Some((_, members)) = resolve_option_members_from_context(state, &uri, &usage.context)
        {
            if members
                .iter()
                .any(|member| member.eq_ignore_ascii_case(&usage.value_name))
            {
                if let crate::handlers::completion::EnumContext::Direct {
                    qualifier_byte_offset,
                    ..
                } = usage.context
                {
                    if let Some(location) = find_inline_option_member_declaration(
                        &uri,
                        &doc,
                        &source,
                        qualifier_byte_offset,
                        &usage.value_name,
                    ) {
                        return Some(GotoDefinitionResponse::Scalar(location));
                    }
                }
            }
        }
    }
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

    if let Some(target) = table_relation_nav_target_at_offset(&doc.tree, &source, byte_offset) {
        drop(doc);
        match target {
            TableRelationNavTarget::Table(table_name) => {
                if let Some(resp) =
                    to_definition_response(find_object_declarations(state, "table", &table_name))
                {
                    return Some(resp);
                }
            }
            TableRelationNavTarget::TableField {
                table_name,
                field_name,
            } => {
                if let Some(resp) = to_definition_response(find_object_member_declarations(
                    state,
                    "table",
                    &table_name,
                    &field_name,
                    false,
                )) {
                    return Some(resp);
                }
            }
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
    let mut implements_type_target: Option<(String, String)> = None;
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

        // Cursor on an interface name inside an object `implements` clause.
        if let Some((object_kind, object_name)) =
            interface_target_from_implements_identifier(ctx.node, &source)
        {
            implements_type_target = Some((object_kind.to_string(), object_name));
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

    if let Some((object_kind, object_name)) = implements_type_target {
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
    fn test_goto_definition_option_value_navigates_to_inline_declaration_member() {
        let source = r#"codeunit 50100 Dummy
{
    procedure Run()
    var
        Choice: Option First, "Second Value";
    begin
        if Choice = Choice::"Second Value" then;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, mut character) = cursor_on(source, "Choice::\"Second Value\"");
        character += "Choice::".len() as u32;
        let result = handle_goto_definition(&state, make_goto_params(uri.clone(), line, character));
        assert!(result.is_some(), "expected goto-definition result");

        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == uri && l.range.start.line == 4 && l.range.start.character >= 23),
            "expected navigation to inline option member declaration on line 4, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_exact_option_parameter_and_local_variable_values() {
        let source = r#"codeunit 50100 Dummy
{
    procedure HelloWithOptions(OptionParameter : Option Alpha, "Bra-vo")
    var
        OptionVariable : Option C, "or D";
    begin
        Message('%1',OptionParameter::Alpha);
        Message('%1',OptionVariable::C);
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (param_line, mut param_char) = cursor_on(source, "OptionParameter::Alpha");
        param_char += "OptionParameter::".len() as u32;
        let param_result =
            handle_goto_definition(&state, make_goto_params(uri.clone(), param_line, param_char));
        assert!(param_result.is_some(), "expected goto-definition for option parameter");
        let param_locs = locations_from(param_result.unwrap());
        assert!(
            param_locs
                .iter()
                .any(|l| l.uri == uri && l.range.start.line == 2 && l.range.start.character >= 54),
            "expected navigation to Alpha in parameter declaration, got: {param_locs:?}"
        );

        let (var_line, mut var_char) = cursor_on(source, "OptionVariable::C");
        var_char += "OptionVariable::".len() as u32;
        let var_result =
            handle_goto_definition(&state, make_goto_params(uri.clone(), var_line, var_char));
        assert!(var_result.is_some(), "expected goto-definition for option variable");
        let var_locs = locations_from(var_result.unwrap());
        assert!(
            var_locs
                .iter()
                .any(|l| l.uri == uri && l.range.start.line == 4 && l.range.start.character >= 26),
            "expected navigation to C in local option declaration, got: {var_locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_global_inline_option_value() {
        let source = r#"codeunit 50100 Dummy
{
    var
        GlobalChoice: Option Alpha, "Bra-vo";

    procedure Run()
    begin
        Message('%1', GlobalChoice::Alpha);
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, mut character) = cursor_on(source, "GlobalChoice::Alpha");
        character += "GlobalChoice::".len() as u32;
        let result = handle_goto_definition(&state, make_goto_params(uri.clone(), line, character));
        assert!(result.is_some(), "expected goto-definition result");

        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == uri && l.range.start.line == 3 && l.range.start.character >= 29),
            "expected navigation to Alpha in global option declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_currpage_usercontrol_controladdin_method() {
        let source = r#"controladdin "Dummy AddIn"
{
    procedure Invoke(Value: Text);
}

page 50100 "Dummy Host"
{
    layout
    {
        area(content)
        {
            usercontrol(Host; "Dummy AddIn")
            {
                ApplicationArea = All;
            }
        }
    }

    procedure Run()
    begin
        CurrPage.Host.Invoke('x');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_on(source, "CurrPage.Host.Invoke");
        let character = character + "CurrPage.Host.".len() as u32;
        let result = handle_goto_definition(&state, make_goto_params(uri.clone(), line, character));
        assert!(result.is_some(), "expected goto-definition result");
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.uri == uri && l.range.start.line == 2),
            "expected navigation to control add-in procedure declaration on line 2, got: {locs:?}"
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

    #[test]
    fn test_goto_definition_interface_name_in_enum_implements_clause() {
        let iface_source = r#"interface "Dummy Device Action"
{
    procedure RunAction();
}"#;
        let enum_source = r#"enum 50100 "Dummy RFID Action" implements "Dummy Device Action"
{
    value(0; None)
    {
    }
}"#;
        let iface_uri = Url::parse("file:///test/interface.al").unwrap();
        let enum_uri = Url::parse("file:///test/enum.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(iface_uri.clone(), DocumentState::new(iface_source).unwrap());
        state
            .documents
            .insert(enum_uri.clone(), DocumentState::new(enum_source).unwrap());

        let (line, character) = cursor_on(enum_source, "\"Dummy Device Action\"");
        let params = make_goto_params(enum_uri, line, character);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to resolve interface in implements clause"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == iface_uri && l.range.start.line == 0),
            "expected navigation to interface declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_tablerelation_table_target_from_member_access() {
        let ref_table_source = r#"table 50100 "Dummy Ref"
{
    fields
    {
        field(1; "Target Code"; Code[20])
        {
            TableRelation = "Dummy Target"."No." WHERE("Blocked" = CONST(false));
        }
    }
}"#;
        let target_table_source = r#"table 50101 "Dummy Target"
{
    fields
    {
        field(1; "No."; Code[20]) { }
        field(2; "Blocked"; Boolean) { }
    }
}"#;
        let ref_uri = Url::parse("file:///test/ref-table.al").unwrap();
        let target_uri = Url::parse("file:///test/target-table.al").unwrap();
        let state = WorldState::new();
        state.documents.insert(
            ref_uri.clone(),
            DocumentState::new(ref_table_source).unwrap(),
        );
        state.documents.insert(
            target_uri.clone(),
            DocumentState::new(target_table_source).unwrap(),
        );

        let (line, character) = cursor_on(ref_table_source, "\"Dummy Target\".\"No.\" WHERE");
        let params = make_goto_params(ref_uri, line, character + 1);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return related table declaration"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == target_uri && l.range.start.line == 0),
            "expected navigation to related table declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_tablerelation_field_target_from_member_access() {
        let ref_table_source = r#"table 50100 "Dummy Ref"
{
    fields
    {
        field(1; "Target Code"; Code[20])
        {
            TableRelation = "Dummy Target"."No." WHERE("Blocked" = CONST(false));
        }
    }
}"#;
        let target_table_source = r#"table 50101 "Dummy Target"
{
    fields
    {
        field(1; "No."; Code[20]) { }
        field(2; "Blocked"; Boolean) { }
    }
}"#;
        let ref_uri = Url::parse("file:///test/ref-table.al").unwrap();
        let target_uri = Url::parse("file:///test/target-table.al").unwrap();
        let state = WorldState::new();
        state.documents.insert(
            ref_uri.clone(),
            DocumentState::new(ref_table_source).unwrap(),
        );
        state.documents.insert(
            target_uri.clone(),
            DocumentState::new(target_table_source).unwrap(),
        );

        let (line, character) = cursor_on(ref_table_source, "\"No.\" WHERE");
        let params = make_goto_params(ref_uri, line, character + 1);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return related field declaration"
        );
        let locs = locations_from(result.unwrap());
        let expected_line = target_table_source[..target_table_source.find("\"No.\"").unwrap()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count() as u32;
        assert!(
            locs.iter()
                .any(|l| l.uri == target_uri && l.range.start.line == expected_line),
            "expected navigation to related table field declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_tablerelation_simple_table_value() {
        let ref_table_source = r#"table 50100 "Dummy Ref"
{
    fields
    {
        field(1; "Config Code"; Code[20])
        {
            TableRelation = "Dummy Config";
        }
    }
}"#;
        let target_table_source = r#"table 50102 "Dummy Config"
{
}"#;
        let ref_uri = Url::parse("file:///test/ref-table.al").unwrap();
        let target_uri = Url::parse("file:///test/target-table.al").unwrap();
        let state = WorldState::new();
        state.documents.insert(
            ref_uri.clone(),
            DocumentState::new(ref_table_source).unwrap(),
        );
        state.documents.insert(
            target_uri.clone(),
            DocumentState::new(target_table_source).unwrap(),
        );

        let (line, character) = cursor_on(ref_table_source, "\"Dummy Config\"");
        let params = make_goto_params(ref_uri, line, character + 1);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return related table declaration"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == target_uri && l.range.start.line == 0),
            "expected navigation to related table declaration, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_record_slash_field_member_to_table_field_declaration() {
        let enum_source = r#"enum 50100 "Dummy Trigger Mode"
{
    value(0; "On Item Added")
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
        field(10; "KDS Display/Printing"; Enum "Dummy Trigger Mode") { }
        field(11; "Restaurant No."; Code[20]) { }
    }
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        HospType: Record "Dummy Hospitality Type";
    begin
        if HospType."KDS Display/Printing" = HospType."KDS Display/Printing"::"On Item Added" then;
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
            .insert(table_uri.clone(), DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        let (line, character) = cursor_on(codeunit_source, "\"KDS Display/Printing\" = HospType");
        let params = make_goto_params(codeunit_uri, line, character + 1);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return table field declaration"
        );

        let expected_line = table_source[..table_source.find("\"KDS Display/Printing\"").unwrap()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count() as u32;
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == table_uri && l.range.start.line == expected_line),
            "expected navigation to slash field declaration in table document, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_enum_value_from_record_slash_field_qualified_usage() {
        let enum_source = r#"enum 50100 "Dummy Trigger Mode"
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
        field(10; "KDS Display/Printing"; Enum "Dummy Trigger Mode") { }
        field(11; "Restaurant No."; Code[20]) { }
    }
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        HospType: Record "Dummy Hospitality Type";
    begin
        if HospType."KDS Display/Printing" = HospType."KDS Display/Printing"::"On Item Added" then;
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

        let (line, character) = cursor_on(codeunit_source, "\"On Item Added\"");
        let params = make_goto_params(codeunit_uri, line, character + 1);
        let result = handle_goto_definition(&state, params);
        assert!(
            result.is_some(),
            "expected goto-definition to return enum value declaration"
        );

        let expected_line = enum_source[..enum_source.find("\"On Item Added\"").unwrap()]
            .bytes()
            .filter(|&b| b == b'\n')
            .count() as u32;
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter()
                .any(|l| l.uri == enum_uri && l.range.start.line == expected_line),
            "expected navigation to enum value declaration in enum document, got: {locs:?}"
        );
    }
}
