use lsp_types::Diagnostic;
use ropey::Rope;
use std::sync::Arc;
use tree_sitter::Tree;

use crate::ast::{extract_symbols, AlSymbol};
use crate::diagnostics::extract_diagnostics;
use crate::symbols::DocumentSymbolTable;

/// A document's state: source text (as Rope), parse tree, symbols, and diagnostics.
pub struct DocumentState {
    pub rope: Rope,
    source_text: Arc<str>,
    pub tree: Tree,
    pub symbol_table: DocumentSymbolTable,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct IncrementalEdit<'a> {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub start_point: tree_sitter::Point,
    pub old_end_point: tree_sitter::Point,
    pub new_end_point: tree_sitter::Point,
    pub new_text: &'a str,
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
            source_text: Arc::<str>::from(source),
            tree,
            symbol_table,
            diagnostics,
        })
    }

    /// Apply an incremental edit and re-parse.
    pub fn apply_edit(&mut self, edit: IncrementalEdit<'_>) {
        let IncrementalEdit {
            start_byte,
            old_end_byte,
            start_point,
            old_end_point,
            new_end_point,
            new_text,
        } = edit;
        let new_end_byte = start_byte + new_text.len();

        // Update rope
        let start_char = self.rope.byte_to_char(start_byte);
        let old_end_char = self
            .rope
            .byte_to_char(old_end_byte.min(self.rope.len_bytes()));
        self.rope.remove(start_char..old_end_char);
        if !new_text.is_empty() {
            self.rope.insert(start_char, new_text);
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
        self.source_text = Arc::<str>::from(full_source.clone());
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
        self.source_text = Arc::<str>::from(source);
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
    pub fn source(&self) -> &str {
        &self.source_text
    }

    /// Get current source as shared immutable text.
    pub fn source_arc(&self) -> Arc<str> {
        self.source_text.clone()
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
