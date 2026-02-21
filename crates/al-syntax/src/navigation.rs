use tree_sitter::{Node, Tree};

use crate::ast::{extract_name, AlSymbol};
use crate::symbols::DocumentSymbolTable;

/// Result of resolving a symbol at a position.
#[derive(Debug)]
pub struct ResolvedSymbol<'a> {
    pub symbol: &'a AlSymbol,
    pub name: String,
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
}
