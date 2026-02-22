use std::collections::HashMap;

use crate::ast::{AlObjectKind, AlSymbol, AlSymbolKind};

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
            .map(|refs| refs.iter().filter_map(|r| self.resolve_ref(r)).collect())
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

    /// Get all symbols reachable from a given byte offset.
    /// Returns locals (if inside a procedure) → object-level members → top-level objects.
    pub fn reachable_symbols(&self, byte_offset: usize) -> Vec<&AlSymbol> {
        let mut result = Vec::new();

        for sym in &self.symbols {
            if sym.start_byte <= byte_offset && byte_offset <= sym.end_byte {
                // Inside this object — check for procedure scope
                for child in &sym.children {
                    if child.start_byte <= byte_offset && byte_offset <= child.end_byte {
                        // Inside a procedure/trigger — add its locals first
                        for local in &child.children {
                            result.push(local);
                        }
                    }
                }
                // Add object-level children (procedures, variables, fields, etc.)
                for child in &sym.children {
                    result.push(child);
                }
            }
            // Add the object itself
            result.push(sym);
        }

        // If we didn't find ourselves inside any object, return all top-level symbols
        if result.is_empty() {
            for sym in &self.symbols {
                result.push(sym);
            }
        }

        result
    }

    /// Find a top-level object by name (case-insensitive).
    pub fn find_object_by_name(&self, name: &str) -> Option<&AlSymbol> {
        let lower = name.to_lowercase();
        self.symbols
            .iter()
            .find(|s| matches!(s.kind, AlSymbolKind::Object(_)) && s.name.to_lowercase() == lower)
    }

    /// If the symbol at `byte_offset` is a procedure that belongs to an interface object,
    /// returns `Some((interface_name, procedure_name))`.
    pub fn interface_method_at(&self, byte_offset: usize) -> Option<(&str, &str)> {
        for sym in &self.symbols {
            if !matches!(sym.kind, AlSymbolKind::Object(AlObjectKind::Interface)) {
                continue;
            }
            if sym.start_byte <= byte_offset && byte_offset <= sym.end_byte {
                for child in &sym.children {
                    if matches!(child.kind, AlSymbolKind::Procedure)
                        && child.start_byte <= byte_offset
                        && byte_offset <= child.end_byte
                    {
                        return Some((&sym.name, &child.name));
                    }
                }
            }
        }
        None
    }

    /// Find all codeunits (or other objects) that implement the given interface name.
    /// For each implementing object, find the child procedure matching `method_name`.
    /// Returns a list of matching procedure symbols.
    pub fn find_implementation_procedures(
        &self,
        interface_name: &str,
        method_name: &str,
    ) -> Vec<&AlSymbol> {
        let iface_lower = interface_name.to_lowercase();
        let method_lower = method_name.to_lowercase();
        let mut results = Vec::new();

        for sym in &self.symbols {
            if sym
                .implements
                .iter()
                .any(|i| i.to_lowercase() == iface_lower)
            {
                for child in &sym.children {
                    if matches!(child.kind, AlSymbolKind::Procedure)
                        && child.name.to_lowercase() == method_lower
                    {
                        results.push(child);
                    }
                }
            }
        }
        results
    }

    /// If the byte offset is on a procedure inside an object that has `implements`,
    /// returns `Some((interface_names, procedure_name))` — the list of interface names
    /// the parent object implements, and the procedure name.
    pub fn implementation_procedure_at(&self, byte_offset: usize) -> Option<(&[String], &str)> {
        for sym in &self.symbols {
            if sym.implements.is_empty() {
                continue;
            }
            if sym.start_byte <= byte_offset && byte_offset <= sym.end_byte {
                for child in &sym.children {
                    if matches!(child.kind, AlSymbolKind::Procedure)
                        && child.start_byte <= byte_offset
                        && byte_offset <= child.end_byte
                    {
                        return Some((&sym.implements, &child.name));
                    }
                }
            }
        }
        None
    }

    /// If the byte offset is on a procedure inside a codeunit (or other non-interface object),
    /// returns `Some((object_name, procedure_name))`.
    pub fn codeunit_procedure_at(&self, byte_offset: usize) -> Option<(&str, &str)> {
        for sym in &self.symbols {
            // Skip interfaces — they have their own handler
            if matches!(sym.kind, AlSymbolKind::Object(AlObjectKind::Interface)) {
                continue;
            }
            if sym.start_byte <= byte_offset && byte_offset <= sym.end_byte {
                for child in &sym.children {
                    if matches!(child.kind, AlSymbolKind::Procedure)
                        && child.start_byte <= byte_offset
                        && byte_offset <= child.end_byte
                    {
                        return Some((&sym.name, &child.name));
                    }
                }
            }
        }
        None
    }

    /// Find a procedure by name inside a named object (case-insensitive).
    pub fn find_object_procedure(
        &self,
        object_name: &str,
        procedure_name: &str,
    ) -> Option<&AlSymbol> {
        let obj_lower = object_name.to_lowercase();
        let proc_lower = procedure_name.to_lowercase();

        for sym in &self.symbols {
            if !matches!(sym.kind, AlSymbolKind::Object(_)) {
                continue;
            }
            if sym.name.to_lowercase() != obj_lower {
                continue;
            }
            for child in &sym.children {
                if matches!(child.kind, AlSymbolKind::Procedure)
                    && child.name.to_lowercase() == proc_lower
                {
                    return Some(child);
                }
            }
        }
        None
    }

    /// Find a method with the given name inside the given interface object.
    /// Returns the procedure symbol if found.
    pub fn find_interface_method(
        &self,
        interface_name: &str,
        method_name: &str,
    ) -> Option<&AlSymbol> {
        let iface_lower = interface_name.to_lowercase();
        let method_lower = method_name.to_lowercase();

        for sym in &self.symbols {
            if !matches!(sym.kind, AlSymbolKind::Object(AlObjectKind::Interface)) {
                continue;
            }
            if sym.name.to_lowercase() != iface_lower {
                continue;
            }
            for child in &sym.children {
                if matches!(child.kind, AlSymbolKind::Procedure)
                    && child.name.to_lowercase() == method_lower
                {
                    return Some(child);
                }
            }
        }
        None
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

/// Return a list of AL language keywords for completion.
pub fn al_keywords() -> &'static [&'static str] {
    &[
        "begin",
        "end",
        "if",
        "then",
        "else",
        "for",
        "to",
        "downto",
        "do",
        "while",
        "repeat",
        "until",
        "case",
        "of",
        "with",
        "exit",
        "var",
        "procedure",
        "trigger",
        "local",
        "internal",
        "protected",
        "true",
        "false",
        "not",
        "and",
        "or",
        "xor",
        "mod",
        "div",
        "in",
        "array",
        "of",
        "temporary",
        "record",
        "codeunit",
        "table",
        "page",
        "report",
        "query",
        "xmlport",
        "enum",
        "tableextension",
        "pageextension",
        "enumextension",
        "field",
        "key",
        "fieldgroup",
    ]
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
    fn test_reachable_symbols() {
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

        let proc_sym = &table.symbols[0].children[1]; // DoWork
        let mid = (proc_sym.start_byte + proc_sym.end_byte) / 2;

        let reachable = table.reachable_symbols(mid);
        let names: Vec<&str> = reachable.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"LocalVar"), "should contain LocalVar");
        assert!(names.contains(&"GlobalVar"), "should contain GlobalVar");
        assert!(names.contains(&"DoWork"), "should contain DoWork");
        assert!(names.contains(&"Test"), "should contain Test object");
    }

    #[test]
    fn test_find_object_by_name() {
        let source = r#"codeunit 50100 "My Codeunit"
{
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        assert!(table.find_object_by_name("My Codeunit").is_some());
        assert!(table.find_object_by_name("my codeunit").is_some());
        assert!(table.find_object_by_name("Nonexistent").is_none());
    }

    #[test]
    fn test_al_keywords() {
        let keywords = al_keywords();
        assert!(keywords.contains(&"begin"));
        assert!(keywords.contains(&"procedure"));
        assert!(keywords.len() > 20);
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

    #[test]
    fn test_interface_method_at() {
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
    procedure SetAddress(NewAddr: Text);
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // Cursor on "GetAddress" method name
        let get_addr_offset = source.find("GetAddress").unwrap();
        let result = table.interface_method_at(get_addr_offset);
        assert!(result.is_some());
        let (iface_name, method_name) = result.unwrap();
        assert_eq!(iface_name, "IAddressProvider");
        assert_eq!(method_name, "GetAddress");

        // Cursor on "SetAddress" method name
        let set_addr_offset = source.find("SetAddress").unwrap();
        let result = table.interface_method_at(set_addr_offset);
        assert!(result.is_some());
        let (iface_name, method_name) = result.unwrap();
        assert_eq!(iface_name, "IAddressProvider");
        assert_eq!(method_name, "SetAddress");

        // Cursor on interface name itself — not a method
        let iface_offset = source.find("IAddressProvider").unwrap();
        let result = table.interface_method_at(iface_offset);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_implementation_procedures() {
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;

    procedure HelperMethod()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        // Should find GetAddress in the implementing codeunit
        let impls = table.find_implementation_procedures("IAddressProvider", "GetAddress");
        assert_eq!(impls.len(), 1);
        assert_eq!(impls[0].name, "GetAddress");

        // Case-insensitive
        let impls = table.find_implementation_procedures("iaddressprovider", "getaddress");
        assert_eq!(impls.len(), 1);

        // HelperMethod exists in the implementing codeunit but is not named in the interface.
        // However, find_implementation_procedures just matches by name in implementing objects,
        // so it WILL find it. The caller controls which method names to search for.
        let impls = table.find_implementation_procedures("IAddressProvider", "HelperMethod");
        assert_eq!(impls.len(), 1);

        // Non-existent interface
        let impls = table.find_implementation_procedures("INonExistent", "GetAddress");
        assert_eq!(impls.len(), 0);
    }

    #[test]
    fn test_implements_clause_extraction() {
        let source = r#"codeunit 50200 MyCodeunit implements IFoo, IBar
{
    procedure DoSomething()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);
        let table = DocumentSymbolTable::new(symbols);

        assert_eq!(table.symbols.len(), 1);
        let cu = &table.symbols[0];
        assert_eq!(cu.implements.len(), 2);
        assert_eq!(cu.implements[0], "IFoo");
        assert_eq!(cu.implements[1], "IBar");
    }
}
