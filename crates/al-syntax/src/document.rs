use lsp_types::Diagnostic;
use ropey::Rope;
use tree_sitter::Tree;

use crate::ast::{extract_symbols, AlSymbol};
use crate::diagnostics::extract_diagnostics;
use crate::symbols::DocumentSymbolTable;

/// A document's state: source text (as Rope), parse tree, symbols, and diagnostics.
pub struct DocumentState {
    pub rope: Rope,
    pub tree: Tree,
    pub symbol_table: DocumentSymbolTable,
    pub diagnostics: Vec<Diagnostic>,
}

impl DocumentState {
    /// Create a new document state from full source text.
    pub fn new(source: &str) -> Option<Self> {
        let tree = al_parser::parse(source)?;
        let symbols = extract_symbols(&tree, source);
        let diagnostics = extract_diagnostics(&tree, source);
        let symbol_table = DocumentSymbolTable::new(symbols);
        let rope = Rope::from_str(source);

        Some(DocumentState {
            rope,
            tree,
            symbol_table,
            diagnostics,
        })
    }

    /// Apply an incremental edit and re-parse.
    pub fn apply_edit(
        &mut self,
        start_byte: usize,
        old_end_byte: usize,
        new_end_byte: usize,
        start_point: tree_sitter::Point,
        old_end_point: tree_sitter::Point,
        new_end_point: tree_sitter::Point,
        new_source: &str,
    ) {
        // Update rope
        let start_char = self.rope.byte_to_char(start_byte);
        let old_end_char = self.rope.byte_to_char(old_end_byte.min(self.rope.len_bytes()));
        self.rope.remove(start_char..old_end_char);
        let new_text_len = new_end_byte - start_byte;
        if new_text_len > 0 {
            // Extract the new text from the full new source
            let insert_text = &new_source[..new_text_len.min(new_source.len())];
            self.rope.insert(start_char, insert_text);
        }

        // Apply edit to tree for incremental parsing
        let edit = tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: start_point,
            old_end_position: old_end_point,
            new_end_position: new_end_point,
        };
        self.tree.edit(&edit);

        // Re-parse incrementally
        let full_source = self.rope.to_string();
        if let Some(new_tree) = al_parser::parse_with(&full_source, Some(&self.tree)) {
            let symbols = extract_symbols(&new_tree, &full_source);
            self.diagnostics = extract_diagnostics(&new_tree, &full_source);
            self.symbol_table = DocumentSymbolTable::new(symbols);
            self.tree = new_tree;
        }
    }

    /// Re-parse the document from the current rope contents.
    /// Used when applying full-document changes.
    pub fn reparse_full(&mut self, source: &str) {
        self.rope = Rope::from_str(source);
        if let Some(new_tree) = al_parser::parse(source) {
            let symbols = extract_symbols(&new_tree, source);
            self.diagnostics = extract_diagnostics(&new_tree, source);
            self.symbol_table = DocumentSymbolTable::new(symbols);
            self.tree = new_tree;
        }
    }

    /// Get all top-level symbols.
    pub fn symbols(&self) -> &[AlSymbol] {
        &self.symbol_table.symbols
    }

    /// Get current source text.
    pub fn source(&self) -> String {
        self.rope.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_state_creation() {
        let source = r#"codeunit 50100 Test
{
    var
        X: Integer;

    procedure Hello()
    begin
    end;
}"#;
        let doc = DocumentState::new(source).unwrap();
        assert!(doc.diagnostics.is_empty());
        assert_eq!(doc.symbols().len(), 1);
        assert_eq!(doc.symbols()[0].name, "Test");
        // X + Hello
        assert_eq!(doc.symbols()[0].children.len(), 2);
    }

    #[test]
    fn test_full_reparse() {
        let source1 = r#"codeunit 50100 Test
{
}"#;
        let mut doc = DocumentState::new(source1).unwrap();
        assert_eq!(doc.symbols()[0].children.len(), 0);

        let source2 = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;
}"#;
        doc.reparse_full(source2);
        assert_eq!(doc.symbols()[0].children.len(), 1);
        assert_eq!(doc.symbols()[0].children[0].name, "Hello");
    }
}
