use tree_sitter::{Node, Tree};

use crate::ast::{extract_name, AlSymbol, AlSymbolKind};
use crate::symbols::DocumentSymbolTable;

/// Result of resolving a symbol at a position.
#[derive(Debug)]
pub struct ResolvedSymbol<'a> {
    pub symbol: &'a AlSymbol,
    pub name: String,
}

/// Context about an identifier at a position (works for both definitions and references).
#[derive(Debug)]
pub struct IdentifierContext<'a> {
    pub node: tree_sitter::Node<'a>,
    pub name: String,
    pub symbol: Option<&'a AlSymbol>,
    pub is_definition: bool,
}

/// Context about a function/method call surrounding the cursor.
#[derive(Debug)]
pub struct CallContext<'a> {
    pub function_name: String,
    pub symbol: Option<&'a AlSymbol>,
    pub active_parameter: usize,
    _marker: std::marker::PhantomData<&'a ()>,
}

/// A folding range expressed as tree-sitter points.
#[derive(Debug)]
pub struct FoldingArea {
    pub start_line: usize,
    pub end_line: usize,
    pub kind: FoldingAreaKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldingAreaKind {
    Region,
    Comment,
}

/// Find the named node at a given byte offset.
pub fn node_at_offset(tree: &Tree, byte_offset: usize) -> Option<tree_sitter::Node<'_>> {
    let root = tree.root_node();
    find_deepest_named_node(root, byte_offset)
}

fn find_deepest_named_node(node: Node, byte_offset: usize) -> Option<Node> {
    if byte_offset < node.start_byte() || byte_offset > node.end_byte() {
        return None;
    }

    // Try to find a deeper named child
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.start_byte() <= byte_offset && byte_offset <= child.end_byte() {
            if let Some(deeper) = find_deepest_named_node(child, byte_offset) {
                return Some(deeper);
            }
        }
    }

    // If no deeper named child, also check unnamed children for named descendants
    let mut cursor2 = node.walk();
    for child in node.children(&mut cursor2) {
        if !child.is_named() && child.start_byte() <= byte_offset && byte_offset <= child.end_byte()
        {
            // Check if there's a named node inside
            if let Some(deeper) = find_deepest_named_node(child, byte_offset) {
                return Some(deeper);
            }
        }
    }

    if node.is_named() {
        Some(node)
    } else {
        None
    }
}

/// Resolve the identifier at a byte offset to its definition symbol.
pub fn resolve_at_offset<'a>(
    tree: &Tree,
    source: &str,
    symbol_table: &'a DocumentSymbolTable,
    byte_offset: usize,
) -> Option<ResolvedSymbol<'a>> {
    let node = node_at_offset(tree, byte_offset)?;

    // Only resolve identifiers and quoted identifiers
    match node.kind() {
        "identifier" | "quoted_identifier" => {}
        _ => return None,
    }

    let name = extract_name(node, source);

    // Use scoped lookup to find the definition
    let results = symbol_table.lookup_in_scope(&name, byte_offset);
    let symbol = results.into_iter().next()?;

    // Don't resolve to self (if we're already on the definition)
    if symbol.start_byte == node.start_byte() && symbol.end_byte == node.end_byte() {
        return None;
    }

    Some(ResolvedSymbol { symbol, name })
}

/// Get the identifier name at a byte offset (for hover).
pub fn identifier_at_offset(tree: &Tree, source: &str, byte_offset: usize) -> Option<String> {
    let node = node_at_offset(tree, byte_offset)?;
    match node.kind() {
        "identifier" | "quoted_identifier" => Some(extract_name(node, source)),
        _ => None,
    }
}

/// Like `resolve_at_offset` but also works when the cursor is on a definition itself.
/// Returns context including whether the identifier is a definition or a reference.
pub fn identifier_context_at_offset<'a>(
    tree: &'a Tree,
    source: &str,
    symbol_table: &'a DocumentSymbolTable,
    byte_offset: usize,
) -> Option<IdentifierContext<'a>> {
    let node = node_at_offset(tree, byte_offset)?;

    match node.kind() {
        "identifier" | "quoted_identifier" => {}
        _ => return None,
    }

    let name = extract_name(node, source);

    // Try scoped lookup
    let results = symbol_table.lookup_in_scope(&name, byte_offset);
    let symbol = results.into_iter().next();

    // Check if we're on a definition: the identifier is the "name" field of a declaration node
    let is_definition = is_definition_node(node);

    Some(IdentifierContext {
        node,
        name,
        symbol,
        is_definition,
    })
}

/// Check if an identifier node is the defining name of a declaration.
fn is_definition_node(node: Node) -> bool {
    if let Some(parent) = node.parent() {
        let parent_kind = parent.kind();
        let is_decl = matches!(
            parent_kind,
            "variable_declaration"
                | "parameter"
                | "procedure_declaration"
                | "trigger_declaration"
                | "field_declaration"
                | "key_declaration"
                | "enum_value_declaration"
                | "interface_method"
        ) || parent_kind.ends_with("_declaration");

        if is_decl {
            // Verify this node is the "name" field of the parent
            if let Some(name_node) = parent.child_by_field_name("name") {
                return name_node.id() == node.id();
            }
            // For objects, the name is the first identifier/quoted_identifier child (after integer)
            if parent_kind.ends_with("_declaration") {
                let mut cursor = parent.walk();
                for child in parent.named_children(&mut cursor) {
                    if child.kind() == "identifier" || child.kind() == "quoted_identifier" {
                        return child.id() == node.id();
                    }
                }
            }
        }
    }
    false
}

/// Find all references to the same symbol as the identifier at `byte_offset`.
/// Walks the entire tree, collecting identifier nodes that match the name and resolve
/// to the same definition (via `lookup_in_scope` per candidate to respect shadowing).
/// Returns start/end points of each reference (including the definition if `include_declaration` is true).
pub fn find_all_references(
    tree: &Tree,
    source: &str,
    symbol_table: &DocumentSymbolTable,
    byte_offset: usize,
    include_declaration: bool,
) -> Vec<(tree_sitter::Point, tree_sitter::Point)> {
    let ctx = match identifier_context_at_offset(tree, source, symbol_table, byte_offset) {
        Some(c) => c,
        None => return Vec::new(),
    };

    let target_symbol = match ctx.symbol {
        Some(s) => s,
        None => return Vec::new(),
    };

    let target_name = ctx.name.to_lowercase();
    let target_start = target_symbol.start_byte;
    let target_end = target_symbol.end_byte;

    let mut refs = Vec::new();
    collect_identifier_nodes(tree.root_node(), source, &target_name, &mut |node| {
        let node_name = extract_name(node, source);
        if node_name.to_lowercase() != target_name {
            return;
        }

        // Check if this node IS the definition name
        let is_def = is_definition_node(node);
        if is_def {
            // Verify it's the SAME definition by checking parent range
            let matches_target = if let Some(parent) = node.parent() {
                parent.start_byte() == target_start && parent.end_byte() == target_end
            } else {
                false
            };
            if matches_target && include_declaration {
                refs.push((node.start_position(), node.end_position()));
            }
            return;
        }

        // Resolve this candidate to see if it points to the same definition
        let candidates = symbol_table.lookup_in_scope(&node_name, node.start_byte());
        if let Some(resolved) = candidates.into_iter().next() {
            if resolved.start_byte == target_start && resolved.end_byte == target_end {
                refs.push((node.start_position(), node.end_position()));
            }
        }
    });

    refs
}

/// Walk tree and call `callback` for each identifier/quoted_identifier node matching `target_name_lower`.
fn collect_identifier_nodes<'a, F>(node: Node<'a>, source: &str, target_name_lower: &str, callback: &mut F)
where
    F: FnMut(Node<'a>),
{
    match node.kind() {
        "identifier" | "quoted_identifier" => {
            let name = extract_name(node, source);
            if name.to_lowercase() == *target_name_lower {
                callback(node);
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_identifier_nodes(child, source, target_name_lower, callback);
    }
}

/// Walk up ancestors from the cursor position to find an enclosing function/method call.
/// Count commas before the cursor to determine the active parameter index.
pub fn find_call_context<'a>(
    tree: &'a Tree,
    source: &str,
    symbol_table: &'a DocumentSymbolTable,
    byte_offset: usize,
) -> Option<CallContext<'a>> {
    let root = tree.root_node();
    let mut node = find_deepest_node(root, byte_offset)?;

    // Walk up to find function_call or method_call
    loop {
        let kind = node.kind();
        if kind == "function_call" || kind == "method_call" {
            break;
        }
        node = match node.parent() {
            Some(p) => p,
            None => return None,
        };
    }

    // Find the function name
    let name_node = node.child_by_field_name("name")
        .or_else(|| node.child_by_field_name("method"))?;
    let function_name = extract_name(name_node, source);

    // Find argument_list and count commas before byte_offset
    let mut active_parameter = 0;
    if let Some(args) = node.child_by_field_name("arguments") {
        let mut cursor = args.walk();
        for child in args.children(&mut cursor) {
            if child.kind() == "," && child.start_byte() < byte_offset {
                active_parameter += 1;
            }
        }
    }

    // Try to resolve the function name to a symbol
    let symbol = symbol_table
        .lookup_in_scope(&function_name, node.start_byte())
        .into_iter()
        .find(|s| matches!(s.kind, AlSymbolKind::Procedure | AlSymbolKind::Trigger));

    Some(CallContext {
        function_name,
        symbol,
        active_parameter,
        _marker: std::marker::PhantomData,
    })
}

/// Find the deepest node (named or unnamed) at a byte offset.
fn find_deepest_node(node: Node, byte_offset: usize) -> Option<Node> {
    if byte_offset < node.start_byte() || byte_offset > node.end_byte() {
        return None;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.start_byte() <= byte_offset && byte_offset <= child.end_byte() {
            if let Some(deeper) = find_deepest_node(child, byte_offset) {
                return Some(deeper);
            }
        }
    }

    Some(node)
}

/// Extract the object name from a type string like `Record "Customer"` → `"Customer"`.
pub fn extract_type_object_name(type_info: &str) -> Option<&str> {
    let type_info = type_info.trim();

    // Handle: Record "Customer", Record Customer, etc.
    // Common AL type patterns: Record X, Codeunit X, Page X, ...
    let prefixes = [
        "Record", "Codeunit", "Page", "Report", "Query", "Xmlport", "Enum", "Interface",
    ];

    for prefix in &prefixes {
        if let Some(rest) = type_info.strip_prefix(prefix) {
            let rest = rest.trim();
            if rest.is_empty() {
                return None;
            }
            // Strip quotes if present
            let name = rest.trim_matches('"');
            if name.is_empty() {
                return None;
            }
            return Some(name);
        }
    }

    None
}

/// Collect folding ranges from structural nodes in the parse tree.
/// Returns ranges for objects, procedures, blocks, sections, and multi-line comments.
pub fn collect_folding_ranges(tree: &Tree) -> Vec<FoldingArea> {
    let mut ranges = Vec::new();
    collect_folding_ranges_recursive(tree.root_node(), &mut ranges);
    ranges
}

fn collect_folding_ranges_recursive(node: Node, ranges: &mut Vec<FoldingArea>) {
    let kind = node.kind();

    let is_foldable = matches!(
        kind,
        "codeunit_declaration"
            | "table_declaration"
            | "table_extension_declaration"
            | "page_declaration"
            | "page_extension_declaration"
            | "report_declaration"
            | "enum_declaration"
            | "enum_extension_declaration"
            | "xmlport_declaration"
            | "query_declaration"
            | "interface_declaration"
            | "permissionset_declaration"
            | "controladdin_declaration"
            | "procedure_declaration"
            | "trigger_declaration"
            | "var_section"
            | "fields_section"
            | "keys_section"
            | "if_statement"
            | "for_statement"
            | "while_statement"
            | "repeat_statement"
            | "case_statement"
            | "with_statement"
    );

    let is_comment = kind == "comment" || kind == "block_comment";

    if is_foldable || is_comment {
        let start_line = node.start_position().row;
        let end_line = node.end_position().row;
        if end_line > start_line {
            ranges.push(FoldingArea {
                start_line,
                end_line,
                kind: if is_comment {
                    FoldingAreaKind::Comment
                } else {
                    FoldingAreaKind::Region
                },
            });
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_folding_ranges_recursive(child, ranges);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::extract_symbols;
    use crate::symbols::DocumentSymbolTable;

    #[test]
    fn test_resolve_variable() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        MyVar: Integer;
    begin
        MyVar := 42
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // Find "MyVar" in the assignment (the usage, not the declaration)
        let usage_offset = source.rfind("MyVar").unwrap();
        let resolved = resolve_at_offset(&tree, source, &table, usage_offset);
        assert!(resolved.is_some(), "should resolve MyVar");
        let r = resolved.unwrap();
        assert_eq!(r.name, "MyVar");
        assert_eq!(r.symbol.type_info.as_deref(), Some("Integer"));
    }

    #[test]
    fn test_resolve_procedure() {
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;

    procedure DoWork()
    begin
        Hello()
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // Find "Hello" in the call
        let call_offset = source.rfind("Hello").unwrap();
        let resolved = resolve_at_offset(&tree, source, &table, call_offset);
        assert!(resolved.is_some(), "should resolve Hello");
        let r = resolved.unwrap();
        assert_eq!(r.name, "Hello");
    }

    #[test]
    fn test_no_resolve_on_keyword() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // "begin" keyword offset
        let begin_offset = source.find("begin").unwrap();
        let resolved = resolve_at_offset(&tree, source, &table, begin_offset);
        // begin is a keyword, not an identifier, so no resolution
        // (depending on parse tree, it may or may not be a named node)
        assert!(
            resolved.is_none(),
            "should not resolve keyword 'begin'"
        );
    }

    #[test]
    fn test_identifier_context_on_definition() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        MyVar: Integer;
    begin
        MyVar := 42
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // On the definition of MyVar
        let def_offset = source.find("MyVar").unwrap();
        let ctx = identifier_context_at_offset(&tree, source, &table, def_offset);
        assert!(ctx.is_some());
        let ctx = ctx.unwrap();
        assert_eq!(ctx.name, "MyVar");
        assert!(ctx.is_definition);
        assert!(ctx.symbol.is_some());
    }

    #[test]
    fn test_identifier_context_on_reference() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        MyVar: Integer;
    begin
        MyVar := 42
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // On the usage of MyVar
        let usage_offset = source.rfind("MyVar").unwrap();
        let ctx = identifier_context_at_offset(&tree, source, &table, usage_offset);
        assert!(ctx.is_some());
        let ctx = ctx.unwrap();
        assert_eq!(ctx.name, "MyVar");
        assert!(!ctx.is_definition);
        assert!(ctx.symbol.is_some());
    }

    #[test]
    fn test_find_all_references() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        MyVar: Integer;
    begin
        MyVar := 42;
        MyVar := MyVar + 1
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // From the first usage of MyVar
        let usage_offset = source[source.find("begin").unwrap()..].find("MyVar").unwrap()
            + source.find("begin").unwrap();

        // With declaration
        let refs = find_all_references(&tree, source, &table, usage_offset, true);
        // declaration + 3 usages = 4
        assert!(refs.len() >= 3, "expected at least 3 refs, got {}", refs.len());

        // Without declaration
        let refs_no_decl = find_all_references(&tree, source, &table, usage_offset, false);
        assert_eq!(refs_no_decl.len(), refs.len() - 1);
    }

    #[test]
    fn test_extract_type_object_name() {
        assert_eq!(extract_type_object_name("Record \"Customer\""), Some("Customer"));
        assert_eq!(extract_type_object_name("Record Customer"), Some("Customer"));
        assert_eq!(extract_type_object_name("Codeunit \"Sales Mgt.\""), Some("Sales Mgt."));
        assert_eq!(extract_type_object_name("Integer"), None);
        assert_eq!(extract_type_object_name("Text[100]"), None);
    }

    #[test]
    fn test_collect_folding_ranges() {
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;

    procedure World()
    var
        X: Integer;
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let ranges = collect_folding_ranges(&tree);
        // At minimum: the codeunit and the two procedures
        assert!(ranges.len() >= 3, "expected at least 3 folding ranges, got {}", ranges.len());
        assert!(ranges.iter().all(|r| r.end_line > r.start_line));
    }
}
