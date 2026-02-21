use tree_sitter::{Node, Tree};

/// The kind of AL object declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlObjectKind {
    Table,
    TableExtension,
    Page,
    PageExtension,
    Codeunit,
    Report,
    Enum,
    EnumExtension,
    Xmlport,
    Query,
    Interface,
    PermissionSet,
    ControlAddin,
}

impl AlObjectKind {
    pub fn from_node_kind(kind: &str) -> Option<Self> {
        match kind {
            "table_declaration" => Some(Self::Table),
            "table_extension_declaration" => Some(Self::TableExtension),
            "page_declaration" => Some(Self::Page),
            "page_extension_declaration" => Some(Self::PageExtension),
            "codeunit_declaration" => Some(Self::Codeunit),
            "report_declaration" => Some(Self::Report),
            "enum_declaration" => Some(Self::Enum),
            "enum_extension_declaration" => Some(Self::EnumExtension),
            "xmlport_declaration" => Some(Self::Xmlport),
            "query_declaration" => Some(Self::Query),
            "interface_declaration" => Some(Self::Interface),
            "permissionset_declaration" => Some(Self::PermissionSet),
            "controladdin_declaration" => Some(Self::ControlAddin),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::TableExtension => "tableextension",
            Self::Page => "page",
            Self::PageExtension => "pageextension",
            Self::Codeunit => "codeunit",
            Self::Report => "report",
            Self::Enum => "enum",
            Self::EnumExtension => "enumextension",
            Self::Xmlport => "xmlport",
            Self::Query => "query",
            Self::Interface => "interface",
            Self::PermissionSet => "permissionset",
            Self::ControlAddin => "controladdin",
        }
    }
}

/// The kind of symbol extracted from the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlSymbolKind {
    Object(AlObjectKind),
    Procedure,
    Trigger,
    Variable,
    Parameter,
    Field,
    Key,
    EnumValue,
}

impl AlSymbolKind {
    pub fn lsp_kind(&self) -> lsp_types::SymbolKind {
        match self {
            Self::Object(_) => lsp_types::SymbolKind::CLASS,
            Self::Procedure => lsp_types::SymbolKind::FUNCTION,
            Self::Trigger => lsp_types::SymbolKind::EVENT,
            Self::Variable => lsp_types::SymbolKind::VARIABLE,
            Self::Parameter => lsp_types::SymbolKind::VARIABLE,
            Self::Field => lsp_types::SymbolKind::FIELD,
            Self::Key => lsp_types::SymbolKind::KEY,
            Self::EnumValue => lsp_types::SymbolKind::ENUM_MEMBER,
        }
    }
}

/// A symbol extracted from the parse tree with its location.
#[derive(Debug, Clone)]
pub struct AlSymbol {
    pub name: String,
    pub kind: AlSymbolKind,
    pub type_info: Option<String>,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: tree_sitter::Point,
    pub end_point: tree_sitter::Point,
    pub children: Vec<AlSymbol>,
}

/// Extract the text of a node from the source.
pub fn node_text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
    node.utf8_text(source.as_bytes()).unwrap_or("")
}

/// Extract the name from an identifier or quoted_identifier node.
/// For quoted identifiers, strips the surrounding double quotes.
pub fn extract_name(node: Node, source: &str) -> String {
    let text = node_text(node, source);
    if node.kind() == "quoted_identifier" {
        text.trim_matches('"').to_string()
    } else {
        text.to_string()
    }
}

/// Find the name node of an object declaration.
/// Object declarations have the name as the first identifier or quoted_identifier child
/// (after the integer_literal for the object ID).
fn find_object_name(node: Node, source: &str) -> Option<String> {
    for i in 0..node.named_child_count() {
        if let Some(child) = node.named_child(i) {
            match child.kind() {
                "identifier" | "quoted_identifier" => {
                    return Some(extract_name(child, source));
                }
                _ => continue,
            }
        }
    }
    None
}

/// Extract type information from a type reference node.
fn extract_type_info(node: Node, source: &str) -> String {
    node_text(node, source).to_string()
}

/// Walk the tree and extract all symbols, building a nested structure.
pub fn extract_symbols(tree: &Tree, source: &str) -> Vec<AlSymbol> {
    let root = tree.root_node();
    let mut symbols = Vec::new();

    for i in 0..root.named_child_count() {
        if let Some(child) = root.named_child(i) {
            if let Some(obj_kind) = AlObjectKind::from_node_kind(child.kind()) {
                if let Some(sym) = extract_object_symbol(child, source, obj_kind) {
                    symbols.push(sym);
                }
            }
        }
    }

    symbols
}

fn extract_object_symbol(node: Node, source: &str, kind: AlObjectKind) -> Option<AlSymbol> {
    let name = find_object_name(node, source)?;
    let mut children = Vec::new();

    extract_children_symbols(node, source, &mut children);

    Some(AlSymbol {
        name,
        kind: AlSymbolKind::Object(kind),
        type_info: Some(kind.label().to_string()),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_point: node.start_position(),
        end_point: node.end_position(),
        children,
    })
}

fn extract_children_symbols(node: Node, source: &str, symbols: &mut Vec<AlSymbol>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "procedure_declaration" => {
                if let Some(sym) = extract_procedure_symbol(child, source) {
                    symbols.push(sym);
                }
            }
            "trigger_declaration" => {
                if let Some(sym) = extract_trigger_symbol(child, source) {
                    symbols.push(sym);
                }
            }
            "var_section" => {
                extract_var_symbols(child, source, symbols);
            }
            "fields_section" => {
                extract_field_symbols(child, source, symbols);
            }
            "keys_section" => {
                extract_key_symbols(child, source, symbols);
            }
            "enum_value_declaration" => {
                if let Some(sym) = extract_enum_value_symbol(child, source) {
                    symbols.push(sym);
                }
            }
            _ => {}
        }
    }
}

fn extract_procedure_symbol(node: Node, source: &str) -> Option<AlSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = extract_name(name_node, source);

    let type_info = node
        .child_by_field_name("return_type")
        .map(|rt| node_text(rt, source).trim_start_matches(':').trim().to_string());

    let mut children = Vec::new();

    // Extract parameters
    if let Some(params) = node.child_by_field_name("parameters") {
        extract_parameter_symbols(params, source, &mut children);
    }

    // Extract local variables
    if let Some(vars) = node.child_by_field_name("vars") {
        extract_var_symbols(vars, source, &mut children);
    }

    Some(AlSymbol {
        name,
        kind: AlSymbolKind::Procedure,
        type_info,
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_point: node.start_position(),
        end_point: node.end_position(),
        children,
    })
}

fn extract_trigger_symbol(node: Node, source: &str) -> Option<AlSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = extract_name(name_node, source);

    let mut children = Vec::new();
    if let Some(vars) = node.child_by_field_name("vars") {
        extract_var_symbols(vars, source, &mut children);
    }

    Some(AlSymbol {
        name,
        kind: AlSymbolKind::Trigger,
        type_info: None,
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_point: node.start_position(),
        end_point: node.end_position(),
        children,
    })
}

fn extract_var_symbols(node: Node, source: &str, symbols: &mut Vec<AlSymbol>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "variable_declaration" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = extract_name(name_node, source);
                let type_info = child
                    .child_by_field_name("type")
                    .map(|t| extract_type_info(t, source));

                symbols.push(AlSymbol {
                    name,
                    kind: AlSymbolKind::Variable,
                    type_info,
                    start_byte: child.start_byte(),
                    end_byte: child.end_byte(),
                    start_point: child.start_position(),
                    end_point: child.end_position(),
                    children: Vec::new(),
                });
            }
        }
    }
}

fn extract_parameter_symbols(node: Node, source: &str, symbols: &mut Vec<AlSymbol>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "parameter" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = extract_name(name_node, source);
                let type_info = child
                    .child_by_field_name("type")
                    .map(|t| extract_type_info(t, source));

                symbols.push(AlSymbol {
                    name,
                    kind: AlSymbolKind::Parameter,
                    type_info,
                    start_byte: child.start_byte(),
                    end_byte: child.end_byte(),
                    start_point: child.start_position(),
                    end_point: child.end_position(),
                    children: Vec::new(),
                });
            }
        }
    }
}

fn extract_field_symbols(node: Node, source: &str, symbols: &mut Vec<AlSymbol>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "field_declaration" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = extract_name(name_node, source);
                let type_info = child
                    .child_by_field_name("type")
                    .map(|t| extract_type_info(t, source));

                symbols.push(AlSymbol {
                    name,
                    kind: AlSymbolKind::Field,
                    type_info,
                    start_byte: child.start_byte(),
                    end_byte: child.end_byte(),
                    start_point: child.start_position(),
                    end_point: child.end_position(),
                    children: Vec::new(),
                });
            }
        }
    }
}

fn extract_key_symbols(node: Node, source: &str, symbols: &mut Vec<AlSymbol>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "key_declaration" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = extract_name(name_node, source);

                symbols.push(AlSymbol {
                    name,
                    kind: AlSymbolKind::Key,
                    type_info: None,
                    start_byte: child.start_byte(),
                    end_byte: child.end_byte(),
                    start_point: child.start_position(),
                    end_point: child.end_position(),
                    children: Vec::new(),
                });
            }
        }
    }
}

fn extract_enum_value_symbol(node: Node, source: &str) -> Option<AlSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = extract_name(name_node, source);

    Some(AlSymbol {
        name,
        kind: AlSymbolKind::EnumValue,
        type_info: None,
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_point: node.start_position(),
        end_point: node.end_position(),
        children: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_codeunit_symbols() {
        let source = r#"codeunit 50100 "My Codeunit"
{
    var
        GlobalVar: Integer;

    procedure Hello(Name: Text): Text
    var
        Greeting: Text;
    begin
    end;

    trigger OnRun()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);

        assert_eq!(symbols.len(), 1);
        let obj = &symbols[0];
        assert_eq!(obj.name, "My Codeunit");
        assert!(matches!(obj.kind, AlSymbolKind::Object(AlObjectKind::Codeunit)));

        // GlobalVar, Hello, OnRun
        assert_eq!(obj.children.len(), 3);

        let var = &obj.children[0];
        assert_eq!(var.name, "GlobalVar");
        assert!(matches!(var.kind, AlSymbolKind::Variable));
        assert_eq!(var.type_info.as_deref(), Some("Integer"));

        let proc = &obj.children[1];
        assert_eq!(proc.name, "Hello");
        assert!(matches!(proc.kind, AlSymbolKind::Procedure));
        assert_eq!(proc.type_info.as_deref(), Some("Text"));
        // Parameter + local var
        assert_eq!(proc.children.len(), 2);
        assert_eq!(proc.children[0].name, "Name");
        assert!(matches!(proc.children[0].kind, AlSymbolKind::Parameter));
        assert_eq!(proc.children[1].name, "Greeting");
        assert!(matches!(proc.children[1].kind, AlSymbolKind::Variable));

        let trigger = &obj.children[2];
        assert_eq!(trigger.name, "OnRun");
        assert!(matches!(trigger.kind, AlSymbolKind::Trigger));
    }

    #[test]
    fn test_extract_table_symbols() {
        let source = r#"table 50100 "Customer"
{
    fields
    {
        field(1; "No."; Code[20])
        {
        }
        field(2; Name; Text[100])
        {
        }
    }

    keys
    {
        key(PK; "No.")
        {
        }
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let symbols = extract_symbols(&tree, source);

        assert_eq!(symbols.len(), 1);
        let table = &symbols[0];
        assert_eq!(table.name, "Customer");

        // 2 fields + 1 key
        assert_eq!(table.children.len(), 3);
        assert_eq!(table.children[0].name, "No.");
        assert!(matches!(table.children[0].kind, AlSymbolKind::Field));
        assert_eq!(table.children[0].type_info.as_deref(), Some("Code[20]"));
        assert_eq!(table.children[1].name, "Name");
        assert_eq!(table.children[2].name, "PK");
        assert!(matches!(table.children[2].kind, AlSymbolKind::Key));
    }
}
