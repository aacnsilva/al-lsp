use tree_sitter::Language;

unsafe extern "C" {
    fn tree_sitter_al() -> *const std::ffi::c_void;
}

/// Returns the tree-sitter [`Language`] for AL.
pub fn language() -> Language {
    unsafe { Language::from_raw(tree_sitter_al() as *const tree_sitter::ffi::TSLanguage) }
}

/// Parse AL source code, returning the tree-sitter [`Tree`].
pub fn parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language()).expect("failed to set AL language");
    parser.parse(source, None)
}

/// Parse AL source with an old tree for incremental reparsing.
pub fn parse_with(source: &str, old_tree: Option<&tree_sitter::Tree>) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language()).expect("failed to set AL language");
    parser.parse(source, old_tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_codeunit() {
        let source = r#"codeunit 50100 "My Codeunit"
{
    procedure Hello()
    begin
        Message('Hello, World!')
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_table() {
        let source = r#"table 50100 "My Table"
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
            Clustered = true;
        }
    }
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }
}
