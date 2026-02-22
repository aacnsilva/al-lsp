use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, Range, TextEdit,
    WorkspaceEdit,
};
use std::collections::HashMap;

use al_syntax::navigation::node_at_offset;

use crate::convert::lsp_position_to_byte_offset;
use crate::state::WorldState;

pub fn handle_code_action(
    state: &WorldState,
    params: CodeActionParams,
) -> Option<Vec<CodeActionOrCommand>> {
    let uri = params.text_document.uri;
    let range = params.range;

    let doc = state.documents.get(&uri)?;
    let source = doc.source();

    let mut actions = Vec::new();

    // Toggle procedure visibility
    if let Some(action) = toggle_visibility_action(&doc.tree, &source, &doc.rope, &uri, range) {
        actions.push(CodeActionOrCommand::CodeAction(action));
    }

    // Extract procedure (only when there's a non-empty selection)
    if range.start != range.end {
        if let Some(action) = extract_procedure_action(&doc.tree, &source, &doc.rope, &uri, range)
        {
            actions.push(CodeActionOrCommand::CodeAction(action));
        }
    }

    if actions.is_empty() {
        None
    } else {
        Some(actions)
    }
}

/// Walk up from a node to find the nearest ancestor of the given kind.
fn find_ancestor<'a>(
    node: tree_sitter::Node<'a>,
    kind: &str,
) -> Option<tree_sitter::Node<'a>> {
    let mut current = node;
    loop {
        if current.kind() == kind {
            return Some(current);
        }
        current = current.parent()?;
    }
}

/// Detect the access modifier (local/internal/protected) before "procedure" keyword
/// by examining the source text of the procedure node.
/// Returns the modifier text and its byte range (start, end) if found.
fn detect_access_modifier(source: &str, proc_node: tree_sitter::Node) -> Option<(String, usize, usize)> {
    let proc_text = &source[proc_node.start_byte()..proc_node.end_byte()];
    let proc_lower = proc_text.to_lowercase();

    // Find "procedure" keyword in the node text
    let proc_kw_offset = proc_lower.find("procedure")?;
    if proc_kw_offset == 0 {
        return None; // No modifier before "procedure"
    }

    let before = proc_text[..proc_kw_offset].trim_end();
    if before.is_empty() {
        return None;
    }

    let modifier_lower = before.to_lowercase();
    if matches!(modifier_lower.as_str(), "local" | "internal" | "protected") {
        // Find the modifier in the original source
        let modifier_start = proc_node.start_byte() + proc_text.find(before).unwrap_or(0);
        // End includes trailing whitespace up to "procedure"
        let modifier_end = proc_node.start_byte() + proc_kw_offset;
        Some((modifier_lower, modifier_start, modifier_end))
    } else {
        None
    }
}

/// Toggle procedure visibility code action.
/// When cursor is on a procedure_declaration:
/// - If it has local/internal/protected → offer "Remove '...' modifier"
/// - If it has no access modifier → offer "Add 'local' modifier"
fn toggle_visibility_action(
    tree: &tree_sitter::Tree,
    source: &str,
    rope: &ropey::Rope,
    uri: &lsp_types::Url,
    range: Range,
) -> Option<CodeAction> {
    let byte_offset = lsp_position_to_byte_offset(rope, range.start)?;
    let node = node_at_offset(tree, byte_offset)?;

    // Find enclosing procedure_declaration
    let proc_node = find_ancestor(node, "procedure_declaration")?;

    let (title, edit) = if let Some((modifier, mod_start, mod_end)) =
        detect_access_modifier(source, proc_node)
    {
        // Has an access modifier — offer to remove it
        let title = format!("Remove '{}' modifier", modifier);

        // Convert byte positions to tree_sitter::Point using rope
        let start_line = rope.byte_to_line(mod_start);
        let start_col = mod_start - rope.line_to_byte(start_line);
        let end_line = rope.byte_to_line(mod_end);
        let end_col = mod_end - rope.line_to_byte(end_line);

        let start_point = tree_sitter::Point { row: start_line, column: start_col };
        let end_point = tree_sitter::Point { row: end_line, column: end_col };

        let text_edit = TextEdit {
            range: crate::convert::ts_range_to_lsp_range(start_point, end_point),
            new_text: String::new(),
        };
        (title, text_edit)
    } else {
        // No access modifier — offer to add 'local'
        let title = "Add 'local' modifier".to_string();

        // Insert "local " before the procedure node start
        let proc_start = proc_node.start_position();
        let insert_pos = crate::convert::ts_range_to_lsp_range(proc_start, proc_start);

        let text_edit = TextEdit {
            range: insert_pos,
            new_text: "local ".to_string(),
        };
        (title, text_edit)
    };

    let mut changes = HashMap::new();
    changes.insert(uri.clone(), vec![edit]);

    Some(CodeAction {
        title,
        kind: Some(CodeActionKind::REFACTOR),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}

/// Extract procedure code action.
/// When the user has a non-empty selection covering complete statements within a begin...end block,
/// extract those statements into a new local procedure.
fn extract_procedure_action(
    tree: &tree_sitter::Tree,
    source: &str,
    rope: &ropey::Rope,
    uri: &lsp_types::Url,
    range: Range,
) -> Option<CodeAction> {
    let start_offset = lsp_position_to_byte_offset(rope, range.start)?;
    let end_offset = lsp_position_to_byte_offset(rope, range.end)?;

    if start_offset >= end_offset {
        return None;
    }

    // Find the node at the start of the selection
    let start_node = node_at_offset(tree, start_offset)?;

    // Find enclosing procedure_declaration
    let proc_node = find_ancestor(start_node, "procedure_declaration")?;

    // Find the body block
    let body = proc_node.child_by_field_name("body")?;
    if body.kind() != "block" {
        return None;
    }

    // Collect statement nodes within the selection
    let mut selected_statements = Vec::new();
    let mut cursor = body.walk();
    for child in body.named_children(&mut cursor) {
        // A statement is any named child of the block (except begin/end keywords)
        if child.kind().ends_with("_statement") || child.kind() == "function_call" || child.kind() == "method_call" {
            if child.start_byte() >= start_offset && child.end_byte() <= end_offset {
                selected_statements.push(child);
            }
        }
    }

    if selected_statements.is_empty() {
        return None;
    }

    // Get the selected source text range (from first statement start to last statement end)
    let first_stmt = selected_statements.first()?;
    let last_stmt = selected_statements.last()?;
    let extract_start = first_stmt.start_byte();
    let extract_end = last_stmt.end_byte();
    let extracted_text = &source[extract_start..extract_end];

    // Collect free variables: locals/params referenced in selection but declared outside it
    let free_vars = collect_free_variables(tree, source, proc_node, extract_start, extract_end);

    // Build parameter list for the new procedure
    let params_str = if free_vars.is_empty() {
        String::new()
    } else {
        free_vars
            .iter()
            .map(|(name, type_info)| format!("var {}: {}", name, type_info))
            .collect::<Vec<_>>()
            .join("; ")
    };

    // Build call argument list
    let args_str = if free_vars.is_empty() {
        String::new()
    } else {
        free_vars
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };

    // Determine the indentation of the procedure
    let proc_start_line = proc_node.start_position().row;
    let proc_line_start = rope.line_to_byte(proc_start_line);
    let proc_indent = &source[proc_line_start..proc_node.start_byte()];

    // Re-indent extracted text relative to the new procedure body
    let stmt_indent = &source[rope.line_to_byte(first_stmt.start_position().row)..first_stmt.start_byte()];
    let new_body_indent = format!("{}    ", proc_indent);
    let reindented = reindent(extracted_text, stmt_indent, &new_body_indent);

    // Build the new procedure text
    let new_proc = format!(
        "\n\n{}local procedure ExtractedProcedure({})\n{}begin\n{}\n{}end;",
        proc_indent,
        params_str,
        proc_indent,
        reindented,
        proc_indent,
    );

    // Build the call replacement
    let call_text = format!("ExtractedProcedure({});", args_str);

    // Two edits:
    // 1. Replace selected statements with the call
    let replace_range = crate::convert::ts_range_to_lsp_range(
        first_stmt.start_position(),
        last_stmt.end_position(),
    );
    let replace_edit = TextEdit {
        range: replace_range,
        new_text: call_text,
    };

    // 2. Insert new procedure after the enclosing procedure
    let insert_pos = proc_node.end_position();
    let insert_range = crate::convert::ts_range_to_lsp_range(insert_pos, insert_pos);
    let insert_edit = TextEdit {
        range: insert_range,
        new_text: new_proc,
    };

    let mut changes = HashMap::new();
    changes.insert(uri.clone(), vec![replace_edit, insert_edit]);

    Some(CodeAction {
        title: "Extract procedure".to_string(),
        kind: Some(CodeActionKind::REFACTOR_EXTRACT),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}

/// Collect free variables referenced in the byte range [start..end] that are
/// declared as parameters or local variables in the enclosing procedure.
fn collect_free_variables(
    tree: &tree_sitter::Tree,
    source: &str,
    proc_node: tree_sitter::Node,
    start: usize,
    end: usize,
) -> Vec<(String, String)> {
    use al_syntax::ast::{extract_name, node_text};
    use std::collections::HashSet;

    // Collect all identifiers used in the selection
    let mut used_names: HashSet<String> = HashSet::new();
    collect_identifiers_in_range(tree.root_node(), source, start, end, &mut used_names);

    // Collect parameter and variable declarations from the procedure
    let mut declared: Vec<(String, String)> = Vec::new();

    if let Some(params) = proc_node.child_by_field_name("parameters") {
        let mut cursor = params.walk();
        for child in params.named_children(&mut cursor) {
            if child.kind() == "parameter" {
                if let (Some(name_node), Some(type_node)) = (
                    child.child_by_field_name("name"),
                    child.child_by_field_name("type"),
                ) {
                    let name = extract_name(name_node, source);
                    let type_info = node_text(type_node, source).to_string();
                    declared.push((name, type_info));
                }
            }
        }
    }

    if let Some(vars) = proc_node.child_by_field_name("vars") {
        let mut cursor = vars.walk();
        for child in vars.named_children(&mut cursor) {
            if child.kind() == "variable_declaration" {
                if let (Some(name_node), Some(type_node)) = (
                    child.child_by_field_name("name"),
                    child.child_by_field_name("type"),
                ) {
                    let name = extract_name(name_node, source);
                    let type_info = node_text(type_node, source).to_string();
                    declared.push((name, type_info));
                }
            }
        }
    }

    // Filter to those that are used in the selection but declared outside it
    declared
        .into_iter()
        .filter(|(name, _)| {
            let lower = name.to_lowercase();
            used_names.contains(&lower)
        })
        .collect()
}

/// Recursively collect all identifier names used within a byte range.
fn collect_identifiers_in_range(
    node: tree_sitter::Node,
    source: &str,
    start: usize,
    end: usize,
    names: &mut std::collections::HashSet<String>,
) {
    if node.end_byte() < start || node.start_byte() > end {
        return;
    }

    if (node.kind() == "identifier" || node.kind() == "quoted_identifier")
        && node.start_byte() >= start
        && node.end_byte() <= end
    {
        let name = al_syntax::ast::extract_name(node, source);
        names.insert(name.to_lowercase());
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_identifiers_in_range(child, source, start, end, names);
    }
}

/// Re-indent text by replacing old_indent prefix with new_indent on each line.
fn reindent(text: &str, old_indent: &str, new_indent: &str) -> String {
    text.lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix(old_indent) {
                format!("{}{}", new_indent, rest)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use al_syntax::document::DocumentState;
    use crate::state::WorldState;
    use lsp_types::{
        CodeActionContext, CodeActionParams, Position, Range, TextDocumentIdentifier, Url,
    };

    fn make_code_action_params(uri: Url, start: (u32, u32), end: (u32, u32)) -> CodeActionParams {
        CodeActionParams {
            text_document: TextDocumentIdentifier { uri },
            range: Range {
                start: Position {
                    line: start.0,
                    character: start.1,
                },
                end: Position {
                    line: end.0,
                    character: end.1,
                },
            },
            context: CodeActionContext {
                diagnostics: vec![],
                only: None,
                trigger_kind: None,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    #[test]
    fn test_toggle_visibility_add_local() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on the procedure (line 2, col 14)
        let params = make_code_action_params(uri.clone(), (2, 14), (2, 14));
        let result = handle_code_action(&state, params);

        assert!(result.is_some(), "expected code actions");
        let actions = result.unwrap();
        let toggle = actions
            .iter()
            .find(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title.contains("local"),
                _ => false,
            });
        assert!(toggle.is_some(), "expected 'Add local modifier' action");

        if let CodeActionOrCommand::CodeAction(ca) = toggle.unwrap() {
            let changes = ca.edit.as_ref().unwrap().changes.as_ref().unwrap();
            let edits = changes.get(&uri).unwrap();
            assert_eq!(edits.len(), 1);
            assert_eq!(edits[0].new_text, "local ");
        }
    }

    #[test]
    fn test_toggle_visibility_remove_local() {
        let source = r#"codeunit 50100 Test
{
    local procedure DoWork()
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on the procedure name (line 2, col 20)
        let params = make_code_action_params(uri.clone(), (2, 20), (2, 20));
        let result = handle_code_action(&state, params);

        assert!(result.is_some(), "expected code actions");
        let actions = result.unwrap();
        let toggle = actions
            .iter()
            .find(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title.contains("Remove"),
                _ => false,
            });
        assert!(toggle.is_some(), "expected 'Remove local modifier' action, got: {:?}",
            actions.iter().map(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title.clone(),
                _ => "command".to_string(),
            }).collect::<Vec<_>>()
        );

        if let CodeActionOrCommand::CodeAction(ca) = toggle.unwrap() {
            let changes = ca.edit.as_ref().unwrap().changes.as_ref().unwrap();
            let edits = changes.get(&uri).unwrap();
            assert_eq!(edits.len(), 1);
            // Should remove "local " (including trailing space)
            assert_eq!(edits[0].new_text, "");
        }
    }

    #[test]
    fn test_toggle_visibility_remove_internal() {
        let source = r#"codeunit 50100 Test
{
    internal procedure DoWork()
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let params = make_code_action_params(uri.clone(), (2, 24), (2, 24));
        let result = handle_code_action(&state, params);

        assert!(result.is_some(), "expected code actions");
        let actions = result.unwrap();
        let toggle = actions
            .iter()
            .find(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title.contains("internal"),
                _ => false,
            });
        assert!(toggle.is_some(), "expected 'Remove internal modifier' action");
    }

    #[test]
    fn test_no_toggle_outside_procedure() {
        let source = r#"codeunit 50100 Test
{
    var
        X: Integer;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on the variable (line 3, col 8)
        let params = make_code_action_params(uri.clone(), (3, 8), (3, 8));
        let result = handle_code_action(&state, params);

        // Should not have a toggle visibility action
        if let Some(actions) = result {
            let has_toggle = actions.iter().any(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => {
                    ca.title.contains("local") || ca.title.contains("Remove")
                }
                _ => false,
            });
            assert!(!has_toggle, "should not offer toggle visibility outside a procedure");
        }
    }

    #[test]
    fn test_extract_procedure() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
        Y: Integer;
    begin
        X := 1;
        Y := X + 1;
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Select "X := 1;\n        Y := X + 1;" (lines 7-8)
        // Line 7: "        X := 1;"  (starts at col 8)
        // Line 8: "        Y := X + 1;" (ends at col 19)
        let params = make_code_action_params(uri.clone(), (7, 8), (8, 19));
        let result = handle_code_action(&state, params);

        assert!(result.is_some(), "expected code actions for selection");
        let actions = result.unwrap();
        let extract = actions
            .iter()
            .find(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title == "Extract procedure",
                _ => false,
            });
        assert!(extract.is_some(), "expected 'Extract procedure' action, got: {:?}",
            actions.iter().map(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title.clone(),
                _ => "command".to_string(),
            }).collect::<Vec<_>>()
        );

        if let CodeActionOrCommand::CodeAction(ca) = extract.unwrap() {
            let changes = ca.edit.as_ref().unwrap().changes.as_ref().unwrap();
            let edits = changes.get(&uri).unwrap();
            assert_eq!(edits.len(), 2, "expected 2 edits (replace + insert)");

            // First edit replaces the selected statements with a call
            assert!(edits[0].new_text.contains("ExtractedProcedure("));

            // Second edit inserts the new procedure
            assert!(edits[1].new_text.contains("local procedure ExtractedProcedure"));
            assert!(edits[1].new_text.contains("begin"));
            assert!(edits[1].new_text.contains("end;"));
        }
    }
}
