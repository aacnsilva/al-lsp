use std::collections::HashSet;

use lsp_types::{CompletionItem, CompletionItemKind, Url};

use al_syntax::ast::{extract_name, AlObjectKind, AlSymbolKind};
use al_syntax::navigation::{extract_type_object_name, node_at_offset};
use al_syntax::symbols::DocumentSymbolTable;

use crate::state::WorldState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EventTarget {
    pub object_kind: String,
    pub object_name: String,
    pub event_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct EventSubscriberContext {
    pub arg_index: usize,
    pub object_ref: Option<(String, String)>,
    pub target: Option<EventTarget>,
}

#[derive(Debug, Clone)]
pub(crate) struct EventPublisherLocation {
    pub uri: Url,
    pub name_start: tree_sitter::Point,
    pub name_end: tree_sitter::Point,
}

#[derive(Debug, Clone)]
pub(crate) struct EventSubscriberUsage {
    pub uri: Url,
    pub start: tree_sitter::Point,
    pub end: tree_sitter::Point,
}

#[derive(Debug, Clone)]
pub(crate) struct EventInvocationUsage {
    pub uri: Url,
    pub start: tree_sitter::Point,
    pub end: tree_sitter::Point,
}

#[derive(Debug, Clone)]
struct EventPublisherInDoc {
    target: EventTarget,
    name_start: tree_sitter::Point,
    name_end: tree_sitter::Point,
}

#[derive(Debug, Clone)]
struct EventSubscriberInDoc {
    target: EventTarget,
    event_arg_start: tree_sitter::Point,
    event_arg_end: tree_sitter::Point,
}

#[derive(Debug, Clone)]
struct EventSubscriberCompletionContext {
    arg_index: usize,
    object_kind: Option<String>,
    object_name: Option<String>,
}

pub(crate) fn event_subscriber_context_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<EventSubscriberContext> {
    let node = node_at_offset(tree, byte_offset)?;
    let attr = find_ancestor_event_subscriber_attribute(node, source)?;
    let args = attribute_argument_nodes(attr);
    let arg_index = argument_index_at_offset(&args, byte_offset);

    let object_kind = args
        .first()
        .and_then(|n| parse_object_type_expr(node_text(*n, source)));
    let object_ref = args
        .get(1)
        .and_then(|n| parse_object_ref_expr(node_text(*n, source), object_kind.as_deref()));

    let target =
        if let (Some((kind, object_name)), Some(event_arg)) = (object_ref.clone(), args.get(2)) {
            let event_name = parse_event_name_expr(node_text(*event_arg, source));
            match event_name {
                Some(event_name) if !object_name.is_empty() => Some(EventTarget {
                    object_kind: kind,
                    object_name,
                    event_name,
                }),
                _ => None,
            }
        } else {
            None
        };

    Some(EventSubscriberContext {
        arg_index,
        object_ref,
        target,
    })
}

pub(crate) fn event_publisher_target_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    byte_offset: usize,
) -> Option<EventTarget> {
    let node = node_at_offset(tree, byte_offset)?;
    let proc_node = find_ancestor_of_kind(node, "procedure_declaration")?;
    let proc_name_node = proc_node.child_by_field_name("name")?;

    // Restrict to cursor on procedure name.
    if byte_offset < proc_name_node.start_byte() || byte_offset > proc_name_node.end_byte() {
        return None;
    }

    if !procedure_has_event_publisher_attribute(proc_node, source) {
        return None;
    }

    let (object_kind, object_name) = enclosing_object_info(proc_node, source)?;
    let event_name = extract_name(proc_name_node, source);

    Some(EventTarget {
        object_kind,
        object_name,
        event_name,
    })
}

pub(crate) fn find_event_publishers(
    state: &WorldState,
    target: &EventTarget,
) -> Vec<EventPublisherLocation> {
    let mut locations = Vec::new();

    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();
        for publisher in collect_event_publishers_in_tree(&doc.tree, &source) {
            if !event_target_matches(&publisher.target, target) {
                continue;
            }
            locations.push(EventPublisherLocation {
                uri: entry.key().clone(),
                name_start: publisher.name_start,
                name_end: publisher.name_end,
            });
        }
    }

    locations
}

pub(crate) fn find_event_subscriber_usages(
    state: &WorldState,
    target: &EventTarget,
) -> Vec<EventSubscriberUsage> {
    let mut locations = Vec::new();

    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();
        for subscriber in collect_event_subscribers_in_tree(&doc.tree, &source) {
            if !event_target_matches(&subscriber.target, target) {
                continue;
            }
            locations.push(EventSubscriberUsage {
                uri: entry.key().clone(),
                start: subscriber.event_arg_start,
                end: subscriber.event_arg_end,
            });
        }
    }

    locations
}

pub(crate) fn find_event_invocation_usages(
    state: &WorldState,
    target: &EventTarget,
) -> Vec<EventInvocationUsage> {
    let mut locations = Vec::new();

    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();
        for (start, end) in
            collect_event_invocations_in_tree(&doc.tree, &source, &doc.symbol_table, target)
        {
            locations.push(EventInvocationUsage {
                uri: entry.key().clone(),
                start,
                end,
            });
        }
    }

    locations
}

pub(crate) fn event_invocation_target_at_offset(
    tree: &tree_sitter::Tree,
    source: &str,
    symbol_table: &DocumentSymbolTable,
    byte_offset: usize,
) -> Option<EventTarget> {
    let node = node_at_offset(tree, byte_offset)?;
    if !matches!(node.kind(), "identifier" | "quoted_identifier") {
        return None;
    }

    let mut current = Some(node);
    while let Some(n) = current {
        if n.kind() == "function_call" {
            let Some(name_node) = function_call_name_node(n) else {
                current = n.parent();
                continue;
            };
            if name_node.id() != node.id() {
                current = n.parent();
                continue;
            }

            let (object_kind, object_name) = enclosing_object_info(n, source)?;
            return Some(EventTarget {
                object_kind,
                object_name,
                event_name: extract_name(name_node, source),
            });
        }

        if n.kind() == "method_call" {
            let Some(method_node) = n.child_by_field_name("method") else {
                current = n.parent();
                continue;
            };
            if method_node.id() != node.id() {
                current = n.parent();
                continue;
            }

            let event_name = extract_name(method_node, source);
            let Some(object_node) = n.child_by_field_name("object") else {
                current = n.parent();
                continue;
            };
            let object_ident = extract_name(object_node, source);
            let candidates = symbol_table.lookup_in_scope(&object_ident, n.start_byte());
            for sym in candidates {
                if !matches!(sym.kind, AlSymbolKind::Variable | AlSymbolKind::Parameter) {
                    continue;
                }
                let Some(type_info) = sym.type_info.as_deref() else {
                    continue;
                };
                let Some((object_kind, object_name)) = extract_type_object_name(type_info) else {
                    continue;
                };
                return Some(EventTarget {
                    object_kind: object_kind.to_string(),
                    object_name: object_name.to_string(),
                    event_name: event_name.clone(),
                });
            }
        }

        current = n.parent();
    }

    None
}

pub(crate) fn event_subscriber_completion_items(
    state: &WorldState,
    source: &str,
    byte_offset: usize,
    prefix_lower: &str,
) -> Option<Vec<CompletionItem>> {
    let ctx = event_subscriber_completion_context_from_source(source, byte_offset)?;

    match ctx.arg_index {
        0 => {
            let mut items = Vec::new();
            for label in [
                "ObjectType::Codeunit",
                "ObjectType::Table",
                "ObjectType::Page",
                "ObjectType::Report",
                "ObjectType::XmlPort",
                "ObjectType::Query",
            ] {
                let lower = label.to_ascii_lowercase();
                let suffix = label.split("::").nth(1).unwrap_or("").to_ascii_lowercase();
                if !prefix_lower.is_empty()
                    && !lower.starts_with(prefix_lower)
                    && !suffix.starts_with(prefix_lower)
                {
                    continue;
                }
                items.push(CompletionItem {
                    label: label.to_string(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some("EventSubscriber object type".to_string()),
                    ..Default::default()
                });
            }
            if items.is_empty() {
                None
            } else {
                Some(items)
            }
        }
        1 => {
            let Some(object_kind) = ctx.object_kind.as_deref() else {
                return None;
            };
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

                    let prefix = object_ref_prefix_for_kind(object_kind);
                    let escaped_name = symbol.name.replace('"', "\"\"");
                    let label = format!("{prefix}::\"{escaped_name}\"");
                    let label_lower = label.to_ascii_lowercase();
                    let name_lower = symbol.name.to_ascii_lowercase();
                    if !prefix_lower.is_empty()
                        && !label_lower.starts_with(prefix_lower)
                        && !name_lower.starts_with(prefix_lower)
                    {
                        continue;
                    }

                    let key = format!("{}::{}", object_kind.to_ascii_lowercase(), name_lower);
                    if !seen.insert(key) {
                        continue;
                    }

                    items.push(CompletionItem {
                        label,
                        kind: Some(CompletionItemKind::CLASS),
                        detail: Some(format!("{object_kind} object")),
                        ..Default::default()
                    });
                }
            }

            if items.is_empty() {
                None
            } else {
                Some(items)
            }
        }
        2 => {
            let (Some(object_kind), Some(object_name)) =
                (ctx.object_kind.as_deref(), ctx.object_name.as_deref())
            else {
                return None;
            };

            let mut items = Vec::new();
            let mut seen = HashSet::new();
            for event_name in
                collect_event_publisher_names_for_object(state, object_kind, object_name)
            {
                let event_lower = event_name.to_ascii_lowercase();
                if !prefix_lower.is_empty() && !event_lower.starts_with(prefix_lower) {
                    continue;
                }
                if !seen.insert(event_lower) {
                    continue;
                }
                items.push(CompletionItem {
                    label: event_name,
                    kind: Some(CompletionItemKind::EVENT),
                    detail: Some("Published event".to_string()),
                    ..Default::default()
                });
            }

            if items.is_empty() {
                None
            } else {
                Some(items)
            }
        }
        _ => None,
    }
}

fn collect_event_publisher_names_for_object(
    state: &WorldState,
    object_kind: &str,
    object_name: &str,
) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();

    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();
        for publisher in collect_event_publishers_in_tree(&doc.tree, &source) {
            if !publisher
                .target
                .object_kind
                .eq_ignore_ascii_case(object_kind)
                || !publisher
                    .target
                    .object_name
                    .eq_ignore_ascii_case(object_name)
            {
                continue;
            }
            let key = publisher.target.event_name.to_ascii_lowercase();
            if !seen.insert(key) {
                continue;
            }
            names.push(publisher.target.event_name);
        }
    }

    names
}

fn collect_event_publishers_in_tree(
    tree: &tree_sitter::Tree,
    source: &str,
) -> Vec<EventPublisherInDoc> {
    let mut publishers = Vec::new();
    let mut sink = Vec::new();
    collect_event_items(tree, source, &mut publishers, &mut sink);
    publishers
}

fn collect_event_subscribers_in_tree(
    tree: &tree_sitter::Tree,
    source: &str,
) -> Vec<EventSubscriberInDoc> {
    let mut sink = Vec::new();
    let mut subscribers = Vec::new();
    collect_event_items(tree, source, &mut sink, &mut subscribers);
    subscribers
}

fn collect_event_invocations_in_tree(
    tree: &tree_sitter::Tree,
    source: &str,
    symbol_table: &DocumentSymbolTable,
    target: &EventTarget,
) -> Vec<(tree_sitter::Point, tree_sitter::Point)> {
    let mut uses = Vec::new();
    collect_event_invocations_recursive(tree.root_node(), source, symbol_table, target, &mut uses);
    uses
}

fn collect_event_items(
    tree: &tree_sitter::Tree,
    source: &str,
    publishers: &mut Vec<EventPublisherInDoc>,
    subscribers: &mut Vec<EventSubscriberInDoc>,
) {
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        let Some(kind) = AlObjectKind::from_node_kind(child.kind()) else {
            continue;
        };
        let Some(object_name) = object_name_from_declaration(child, source) else {
            continue;
        };
        collect_event_items_in_object(
            child,
            source,
            kind.label(),
            &object_name,
            publishers,
            subscribers,
        );
    }
}

fn collect_event_items_in_object(
    node: tree_sitter::Node<'_>,
    source: &str,
    object_kind: &str,
    object_name: &str,
    publishers: &mut Vec<EventPublisherInDoc>,
    subscribers: &mut Vec<EventSubscriberInDoc>,
) {
    if node.kind() == "procedure_declaration" {
        let Some(name_node) = node.child_by_field_name("name") else {
            return;
        };
        let procedure_name = extract_name(name_node, source);
        let mut is_publisher = false;

        for attr in procedure_attribute_nodes(node) {
            let Some(name) = attribute_name(attr, source) else {
                continue;
            };
            if is_event_publisher_attribute(&name) {
                is_publisher = true;
            }

            if !name.eq_ignore_ascii_case("EventSubscriber") {
                continue;
            }

            if let Some(subscriber) =
                parse_event_subscriber_attribute(attr, source, object_kind, object_name)
            {
                subscribers.push(subscriber);
            }
        }

        if is_publisher {
            publishers.push(EventPublisherInDoc {
                target: EventTarget {
                    object_kind: object_kind.to_string(),
                    object_name: object_name.to_string(),
                    event_name: procedure_name,
                },
                name_start: name_node.start_position(),
                name_end: name_node.end_position(),
            });
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_event_items_in_object(
            child,
            source,
            object_kind,
            object_name,
            publishers,
            subscribers,
        );
    }
}

fn collect_event_invocations_recursive(
    node: tree_sitter::Node<'_>,
    source: &str,
    symbol_table: &DocumentSymbolTable,
    target: &EventTarget,
    uses: &mut Vec<(tree_sitter::Point, tree_sitter::Point)>,
) {
    if node.kind() == "function_call" {
        if let Some(name_node) = function_call_name_node(node) {
            let call_name = extract_name(name_node, source);
            if call_name.eq_ignore_ascii_case(&target.event_name) {
                if let Some((object_kind, object_name)) = enclosing_object_info(node, source) {
                    if object_kind.eq_ignore_ascii_case(&target.object_kind)
                        && object_name.eq_ignore_ascii_case(&target.object_name)
                    {
                        uses.push((name_node.start_position(), name_node.end_position()));
                    }
                }
            }
        }
    }

    if node.kind() == "method_call" {
        if let (Some(method_node), Some(object_node)) = (
            node.child_by_field_name("method"),
            node.child_by_field_name("object"),
        ) {
            let method_name = extract_name(method_node, source);
            if method_name.eq_ignore_ascii_case(&target.event_name) {
                let object_ident = extract_name(object_node, source);
                let candidates = symbol_table.lookup_in_scope(&object_ident, node.start_byte());
                for sym in candidates {
                    if !matches!(sym.kind, AlSymbolKind::Variable | AlSymbolKind::Parameter) {
                        continue;
                    }
                    let Some(type_info) = sym.type_info.as_deref() else {
                        continue;
                    };
                    if type_info_matches_event_target(type_info, target) {
                        uses.push((method_node.start_position(), method_node.end_position()));
                        break;
                    }
                }
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_event_invocations_recursive(child, source, symbol_table, target, uses);
    }
}

fn parse_event_subscriber_attribute(
    attr: tree_sitter::Node<'_>,
    source: &str,
    _owner_object_kind: &str,
    _owner_object_name: &str,
) -> Option<EventSubscriberInDoc> {
    let args = attribute_argument_nodes(attr);
    if args.len() < 3 {
        return None;
    }

    let object_kind_hint = parse_object_type_expr(node_text(args[0], source));
    let (object_kind, object_name) =
        parse_object_ref_expr(node_text(args[1], source), object_kind_hint.as_deref())?;
    if object_name.is_empty() {
        return None;
    }

    let event_name = parse_event_name_expr(node_text(args[2], source))?;
    if event_name.is_empty() {
        return None;
    }

    Some(EventSubscriberInDoc {
        target: EventTarget {
            object_kind,
            object_name,
            event_name,
        },
        event_arg_start: args[2].start_position(),
        event_arg_end: args[2].end_position(),
    })
}

fn find_ancestor_event_subscriber_attribute<'a>(
    node: tree_sitter::Node<'a>,
    source: &str,
) -> Option<tree_sitter::Node<'a>> {
    let mut current = Some(node);
    while let Some(n) = current {
        if n.kind() == "attribute"
            && attribute_name(n, source)
                .as_ref()
                .is_some_and(|name| name.eq_ignore_ascii_case("EventSubscriber"))
        {
            return Some(n);
        }
        current = n.parent();
    }
    None
}

fn procedure_has_event_publisher_attribute(node: tree_sitter::Node<'_>, source: &str) -> bool {
    procedure_attribute_nodes(node).into_iter().any(|attr| {
        attribute_name(attr, source)
            .as_ref()
            .is_some_and(|name| is_event_publisher_attribute(name))
    })
}

fn enclosing_object_info(node: tree_sitter::Node<'_>, source: &str) -> Option<(String, String)> {
    let mut current = Some(node);
    while let Some(n) = current {
        if let Some(kind) = AlObjectKind::from_node_kind(n.kind()) {
            let name = object_name_from_declaration(n, source)?;
            return Some((kind.label().to_string(), name));
        }
        current = n.parent();
    }
    None
}

fn object_name_from_declaration(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if matches!(child.kind(), "identifier" | "quoted_identifier") {
            return Some(extract_name(child, source));
        }
    }
    None
}

fn procedure_attribute_nodes(node: tree_sitter::Node<'_>) -> Vec<tree_sitter::Node<'_>> {
    let mut attrs = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "attribute" {
            attrs.push(child);
        }
    }
    attrs
}

fn attribute_name(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if matches!(child.kind(), "identifier" | "quoted_identifier") {
            return Some(extract_name(child, source));
        }
    }
    None
}

fn attribute_argument_nodes(node: tree_sitter::Node<'_>) -> Vec<tree_sitter::Node<'_>> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() != "attribute_arguments" {
            continue;
        }
        let mut args = Vec::new();
        let mut args_cursor = child.walk();
        for arg in child.named_children(&mut args_cursor) {
            args.push(arg);
        }
        return args;
    }
    Vec::new()
}

fn argument_index_at_offset(args: &[tree_sitter::Node<'_>], byte_offset: usize) -> usize {
    if args.is_empty() {
        return 0;
    }

    for (idx, arg) in args.iter().enumerate() {
        if byte_offset >= arg.start_byte() && byte_offset <= arg.end_byte() {
            return idx;
        }
    }

    if byte_offset < args[0].start_byte() {
        return 0;
    }

    for (idx, pair) in args.windows(2).enumerate() {
        if byte_offset > pair[0].end_byte() && byte_offset < pair[1].start_byte() {
            return idx + 1;
        }
    }

    if byte_offset > args[args.len() - 1].end_byte() {
        return args.len();
    }

    args.len().saturating_sub(1)
}

fn event_subscriber_completion_context_from_source(
    source: &str,
    byte_offset: usize,
) -> Option<EventSubscriberCompletionContext> {
    if byte_offset == 0 || source.is_empty() {
        return None;
    }

    let cursor = byte_offset.min(source.len());
    let before = &source[..cursor];
    let before_lower = before.to_ascii_lowercase();
    let marker = before_lower.rfind("[eventsubscriber")?;
    let after_marker = &source[marker..cursor];
    let open_rel = after_marker.find('(')?;
    let open = marker + open_rel;

    // Not inside this attribute anymore.
    if source[marker..cursor].contains(']') {
        return None;
    }

    if cursor <= open + 1 {
        return Some(EventSubscriberCompletionContext {
            arg_index: 0,
            object_kind: None,
            object_name: None,
        });
    }

    let args_prefix = &source[open + 1..cursor];
    let parts = split_top_level_args(args_prefix);
    if parts.is_empty() {
        return Some(EventSubscriberCompletionContext {
            arg_index: 0,
            object_kind: None,
            object_name: None,
        });
    }

    let arg_index = parts.len().saturating_sub(1);
    let object_kind = parts.first().and_then(|p| parse_object_type_expr(p));
    let object_ref = parts
        .get(1)
        .and_then(|p| parse_object_ref_expr(p, object_kind.as_deref()));

    let object_name =
        object_ref.and_then(|(_, name)| if name.is_empty() { None } else { Some(name) });

    Some(EventSubscriberCompletionContext {
        arg_index,
        object_kind,
        object_name,
    })
}

fn split_top_level_args(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    let bytes = text.as_bytes();
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    let mut in_single = false;
    let mut in_double = false;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    while i < bytes.len() {
        let b = bytes[i];

        if in_single {
            if b == b'\'' {
                if i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                    i += 2;
                    continue;
                }
                in_single = false;
            }
            i += 1;
            continue;
        }

        if in_double {
            if b == b'"' {
                if i + 1 < bytes.len() && bytes[i + 1] == b'"' {
                    i += 2;
                    continue;
                }
                in_double = false;
            }
            i += 1;
            continue;
        }

        match b {
            b'\'' => in_single = true,
            b'"' => in_double = true,
            b'(' => paren_depth += 1,
            b')' => paren_depth = paren_depth.saturating_sub(1),
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b',' if paren_depth == 0 && bracket_depth == 0 => {
                parts.push(text[start..i].trim().to_string());
                start = i + 1;
            }
            _ => {}
        }

        i += 1;
    }

    parts.push(text[start..].trim().to_string());
    parts
}

fn parse_object_type_expr(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some((left, right)) = split_double_colon(trimmed) {
        if left.eq_ignore_ascii_case("ObjectType") {
            return canonical_object_kind(&right).map(str::to_string);
        }

        if left.eq_ignore_ascii_case("Database") {
            return Some("table".to_string());
        }

        if let Some(kind) = canonical_object_kind(&right) {
            return Some(kind.to_string());
        }
        if let Some(kind) = canonical_object_kind(&left) {
            return Some(kind.to_string());
        }
    }

    canonical_object_kind(trimmed).map(str::to_string)
}

fn parse_object_ref_expr(text: &str, kind_hint: Option<&str>) -> Option<(String, String)> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some((left, right)) = split_double_colon(trimmed) {
        let kind = if left.eq_ignore_ascii_case("Database") {
            Some("table")
        } else {
            canonical_object_kind(&left)
                .or_else(|| {
                    if left.eq_ignore_ascii_case("ObjectType") {
                        canonical_object_kind(&right)
                    } else {
                        None
                    }
                })
                .or(kind_hint)
        };

        if let Some(kind) = kind {
            // `ObjectType::Codeunit` is not an object reference name.
            if left.eq_ignore_ascii_case("ObjectType") {
                return Some((kind.to_string(), String::new()));
            }
            return Some((kind.to_string(), right));
        }
    }

    let kind = kind_hint?;
    Some((kind.to_string(), clean_component(trimmed)))
}

fn parse_event_name_expr(text: &str) -> Option<String> {
    let name = clean_component(text);
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn type_info_matches_event_target(type_info: &str, target: &EventTarget) -> bool {
    let Some((object_kind, object_name)) = extract_type_object_name(type_info) else {
        return false;
    };
    object_kind.eq_ignore_ascii_case(&target.object_kind)
        && object_name.eq_ignore_ascii_case(&target.object_name)
}

fn split_double_colon(text: &str) -> Option<(String, String)> {
    let idx = text.rfind("::")?;
    let left = text[..idx].trim();
    let right = text[idx + 2..].trim();
    if left.is_empty() {
        return None;
    }
    Some((clean_component(left), clean_component(right)))
}

fn clean_component(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        return trimmed[1..trimmed.len() - 1].replace("\"\"", "\"");
    }
    if trimmed.len() >= 2 && trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        return trimmed[1..trimmed.len() - 1].replace("''", "'");
    }

    trimmed.to_string()
}

fn canonical_object_kind(text: &str) -> Option<&'static str> {
    match text.trim_matches('"').trim().to_ascii_lowercase().as_str() {
        "table" | "database" => Some("table"),
        "codeunit" => Some("codeunit"),
        "page" => Some("page"),
        "report" => Some("report"),
        "xmlport" => Some("xmlport"),
        "query" => Some("query"),
        "interface" => Some("interface"),
        "enum" => Some("enum"),
        _ => None,
    }
}

fn object_ref_prefix_for_kind(kind: &str) -> &'static str {
    match kind.to_ascii_lowercase().as_str() {
        "table" => "Database",
        "codeunit" => "Codeunit",
        "page" => "Page",
        "report" => "Report",
        "xmlport" => "XmlPort",
        "query" => "Query",
        "interface" => "Interface",
        "enum" => "Enum",
        _ => "Codeunit",
    }
}

fn is_event_publisher_attribute(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "integrationevent"
            | "businessevent"
            | "internalevent"
            | "externalbusinessevent"
            | "externalevent"
    )
}

fn event_target_matches(left: &EventTarget, right: &EventTarget) -> bool {
    left.object_kind.eq_ignore_ascii_case(&right.object_kind)
        && left.object_name.eq_ignore_ascii_case(&right.object_name)
        && left.event_name.eq_ignore_ascii_case(&right.event_name)
}

fn function_call_name_node(node: tree_sitter::Node<'_>) -> Option<tree_sitter::Node<'_>> {
    node.child_by_field_name("function")
        .or_else(|| node.child_by_field_name("name"))
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

fn node_text<'a>(node: tree_sitter::Node<'a>, source: &'a str) -> &'a str {
    node.utf8_text(source.as_bytes()).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_top_level_args_keeps_commas_in_strings() {
        let parts = split_top_level_args(
            "ObjectType::Codeunit, Codeunit::\"My Publisher\", 'On,After', '', false, false",
        );
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[2], "'On,After'");
    }

    #[test]
    fn test_parse_object_ref_expr_database_table() {
        let parsed = parse_object_ref_expr("Database::\"Sales Header\"", Some("table"));
        assert_eq!(
            parsed,
            Some(("table".to_string(), "Sales Header".to_string()))
        );
    }
}
