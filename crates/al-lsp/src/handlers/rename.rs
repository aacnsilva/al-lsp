use lsp_types::{PrepareRenameResponse, RenameParams, TextDocumentPositionParams, TextEdit, WorkspaceEdit};
use std::collections::HashMap;

use al_syntax::navigation::{find_all_references, identifier_context_at_offset};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_prepare_rename(
    state: &WorldState,
    params: TextDocumentPositionParams,
) -> Option<PrepareRenameResponse> {
    let uri = params.text_document.uri;
    let position = params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let ctx = identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    // Don't allow renaming triggers or object names (too complex for simple rename)
    if let Some(sym) = ctx.symbol {
        match sym.kind {
            al_syntax::ast::AlSymbolKind::Trigger | al_syntax::ast::AlSymbolKind::Object(_) => {
                return None;
            }
            _ => {}
        }
    }

    let range = ts_range_to_lsp_range(ctx.node.start_position(), ctx.node.end_position());

    Some(PrepareRenameResponse::Range(range))
}

pub fn handle_rename(state: &WorldState, params: RenameParams) -> Option<WorkspaceEdit> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let new_name = params.new_name;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let ctx = identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    // Don't allow renaming triggers or object names
    if let Some(sym) = ctx.symbol {
        match sym.kind {
            al_syntax::ast::AlSymbolKind::Trigger | al_syntax::ast::AlSymbolKind::Object(_) => {
                return None;
            }
            _ => {}
        }
    }

    // Find all references including the declaration
    let refs = find_all_references(&doc.tree, &source, &doc.symbol_table, byte_offset, true);

    if refs.is_empty() {
        return None;
    }

    // Determine if we need quotes: if new_name contains spaces, it needs quoting
    let needs_quotes = new_name.contains(' ');

    let edits: Vec<TextEdit> = refs
        .into_iter()
        .map(|(start, end)| {
            let range = ts_range_to_lsp_range(start, end);
            // Check if original was quoted by checking column diff vs name length
            let replacement = if needs_quotes {
                format!("\"{}\"", new_name)
            } else {
                new_name.clone()
            };
            TextEdit {
                range,
                new_text: replacement,
            }
        })
        .collect();

    let mut changes = HashMap::new();
    changes.insert(uri.clone(), edits);

    Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    })
}
