use std::collections::HashMap;

use crate::ast::{AlSymbol, AlSymbolKind};

/// A case-insensitive index of symbols within a document.
#[derive(Debug, Default)]
pub struct DocumentSymbolTable {
    /// Top-level symbols (objects).
    pub symbols: Vec<AlSymbol>,
    /// Flat index: lowercase name → list of symbol references.
    index: HashMap<String, Vec<SymbolRef>>,
}

/// A reference into the symbol tree.
#[derive(Debug, Clone)]
pub struct SymbolRef {
    /// Index into the top-level symbols vec.
    pub object_idx: usize,
    /// Path of child indices to reach this symbol, empty for top-level.
    pub child_path: Vec<usize>,
}

impl DocumentSymbolTable {
    /// Build a symbol table from extracted symbols.
    pub fn new(symbols: Vec<AlSymbol>) -> Self {
        let mut index = HashMap::new();
        for (obj_idx, sym) in symbols.iter().enumerate() {
            insert_into_index(&mut index, sym, obj_idx, &[]);
        }
        DocumentSymbolTable { symbols, index }
    }

    /// Lookup symbols by name (case-insensitive).
    pub fn lookup(&self, name: &str) -> Vec<&AlSymbol> {
        let key = name.to_lowercase();
        self.index
            .get(&key)
            .map(|refs| {
                refs.iter()
                    .filter_map(|r| self.resolve_ref(r))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Lookup symbols by name within a specific scope (object or procedure).
    /// Searches the children of the scope, then falls back to the object level.
    pub fn lookup_in_scope(&self, name: &str, scope_byte: usize) -> Vec<&AlSymbol> {
        let key = name.to_lowercase();
        let mut results = Vec::new();

        // Find the innermost scope containing the byte offset
        for sym in &self.symbols {
            if sym.start_byte <= scope_byte && scope_byte <= sym.end_byte {
                // Check children (procedures, triggers) for local scope
                for child in &sym.children {
                    if child.start_byte <= scope_byte && scope_byte <= child.end_byte {
                        // Inside a procedure/trigger — check its locals first
                        for local in &child.children {
                            if local.name.to_lowercase() == key {
                                results.push(local);
                            }
                        }
                        if !results.is_empty() {
                            return results;
                        }
                    }
                }
                // Check object-level children
                for child in &sym.children {
                    if child.name.to_lowercase() == key {
                        results.push(child);
                    }
                }
                if !results.is_empty() {
                    return results;
                }
            }
        }

        // Fall back to global lookup
        self.lookup(name)
    }

    fn resolve_ref(&self, sym_ref: &SymbolRef) -> Option<&AlSymbol> {
        let mut current = self.symbols.get(sym_ref.object_idx)?;
        for &idx in &sym_ref.child_path {
            current = current.children.get(idx)?;
        }
        Some(current)
    }
}

fn insert_into_index(
    index: &mut HashMap<String, Vec<SymbolRef>>,
    sym: &AlSymbol,
    object_idx: usize,
    parent_path: &[usize],
) {
    let key = sym.name.to_lowercase();
    let sym_ref = SymbolRef {
        object_idx,
        child_path: parent_path.to_vec(),
    };
    index.entry(key).or_default().push(sym_ref);

    for (child_idx, child) in sym.children.iter().enumerate() {
        let mut child_path = parent_path.to_vec();
        child_path.push(child_idx);
        insert_into_index(index, child, object_idx, &child_path);
    }
}

/// Format symbol info as Markdown for hover display.
pub fn format_hover(sym: &AlSymbol) -> String {
    let kind_label = match &sym.kind {
        AlSymbolKind::Object(ok) => ok.label().to_string(),
        AlSymbolKind::Procedure => "procedure".to_string(),
        AlSymbolKind::Trigger => "trigger".to_string(),
        AlSymbolKind::Variable => "variable".to_string(),
        AlSymbolKind::Parameter => "parameter".to_string(),
        AlSymbolKind::Field => "field".to_string(),
        AlSymbolKind::Key => "key".to_string(),
        AlSymbolKind::EnumValue => "enum value".to_string(),
    };

    let type_str = sym
        .type_info
        .as_deref()
        .map(|t| format!(": {t}"))
        .unwrap_or_default();

    format!("```al\n({kind_label}) {}{type_str}\n```", sym.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::extract_symbols;

    #[test]
    fn test_case_insensitive_lookup() {
        let source = r#"codeunit 50100 Test
{
    procedure HelloWorld()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        assert_eq!(table.lookup("helloworld").len(), 1);
        assert_eq!(table.lookup("HELLOWORLD").len(), 1);
        assert_eq!(table.lookup("HelloWorld").len(), 1);
        assert_eq!(table.lookup("nonexistent").len(), 0);
    }

    #[test]
    fn test_scoped_lookup() {
        let source = r#"codeunit 50100 Test
{
    var
        GlobalVar: Integer;

    procedure DoWork()
    var
        LocalVar: Text;
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // Inside the procedure, LocalVar should be found
        // The procedure body is roughly around byte offset 100
        let proc_sym = &table.symbols[0].children[1]; // DoWork
        let mid = (proc_sym.start_byte + proc_sym.end_byte) / 2;

        let results = table.lookup_in_scope("localvar", mid);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "LocalVar");

        // GlobalVar should also be accessible
        let results = table.lookup_in_scope("globalvar", mid);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "GlobalVar");
    }
}
