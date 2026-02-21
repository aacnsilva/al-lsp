use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use tree_sitter::Tree;

/// Walk the tree and extract diagnostics from ERROR and MISSING nodes.
pub fn extract_diagnostics(tree: &Tree, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    walk_for_errors(tree.root_node(), source, &mut diagnostics);
    diagnostics
}

fn walk_for_errors(node: tree_sitter::Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
    if node.is_error() {
        let start = node.start_position();
        let end = node.end_position();
        let text = node
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .chars()
            .take(50)
            .collect::<String>();

        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: start.row as u32,
                    character: start.column as u32,
                },
                end: Position {
                    line: end.row as u32,
                    character: end.column as u32,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("al-lsp".to_string()),
            message: format!("Syntax error: unexpected `{text}`"),
            ..Default::default()
        });
    } else if node.is_missing() {
        let start = node.start_position();
        let kind = node.kind();

        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: start.row as u32,
                    character: start.column as u32,
                },
                end: Position {
                    line: start.row as u32,
                    character: start.column as u32 + 1,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("al-lsp".to_string()),
            message: format!("Expected `{kind}`"),
            ..Default::default()
        });
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_for_errors(child, source, diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_errors_for_valid_code() {
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(diags.is_empty(), "expected no errors, got: {:?}", diags);
    }

    #[test]
    fn test_errors_for_invalid_code() {
        let source = r#"codeunit 50100 Test
{
    procedure
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(!diags.is_empty(), "expected errors for invalid code");
    }
}
