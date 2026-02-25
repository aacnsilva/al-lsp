use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
};
use tower_lsp::Client;

use al_syntax::document::DocumentState;

use crate::state::WorldState;

use super::diagnostics::publish_diagnostics;

pub async fn handle_did_open(
    client: &Client,
    state: &WorldState,
    params: DidOpenTextDocumentParams,
) {
    let uri = params.text_document.uri;
    let text = params.text_document.text;

    if let Some(doc) = DocumentState::new(&text) {
        state.upsert_document(uri.clone(), doc);
        if let Some(doc_ref) = state.documents.get(&uri) {
            publish_diagnostics(client, state, &uri, &doc_ref).await;
        }
    }
}

pub async fn handle_did_change(
    client: &Client,
    state: &WorldState,
    params: DidChangeTextDocumentParams,
) {
    let uri = params.text_document.uri;

    let mut changed = false;
    if let Some(mut doc) = state.documents.get_mut(&uri) {
        for change in params.content_changes {
            changed = true;
            if change.range.is_some() {
                // Incremental edit path for low-latency typing diagnostics.
                if let Some(range) = change.range {
                    let start_offset = offset_from_position(&doc.rope, range.start);
                    let end_offset = offset_from_position(&doc.rope, range.end);
                    if let (Some(start), Some(end)) = (start_offset, end_offset) {
                        let start_point = tree_sitter::Point {
                            row: range.start.line as usize,
                            column: range.start.character as usize,
                        };
                        let old_end_point = tree_sitter::Point {
                            row: range.end.line as usize,
                            column: range.end.character as usize,
                        };
                        let new_end_point = advance_point_by_text(start_point, &change.text);
                        let new_end_byte = start + change.text.len();

                        doc.apply_edit(
                            start,
                            end,
                            new_end_byte,
                            start_point,
                            old_end_point,
                            new_end_point,
                            &change.text,
                        );
                    } else {
                        // Fallback: if offsets cannot be computed, do a full reparse.
                        let mut source = doc.source();
                        if let (Some(start), Some(end)) = (start_offset, end_offset) {
                            source.replace_range(start..end, &change.text);
                        }
                        doc.reparse_full(&source);
                    }
                }
            } else {
                // Full document sync
                doc.reparse_full(&change.text);
            }
        }
    }

    if changed {
        state.reindex_document(&uri);
        if let Some(doc_ref) = state.documents.get(&uri) {
            publish_diagnostics(client, state, &uri, &doc_ref).await;
        }
    }
}

pub async fn handle_did_close(_state: &WorldState, params: DidCloseTextDocumentParams) {
    // Don't remove the document — it may be needed for cross-document features
    // (rename, references, go-to-definition). The workspace scanner loaded it from
    // disk, and closing a tab shouldn't discard that knowledge.
    // The file watcher (didChangeWatchedFiles) handles actual deletions.
    let _ = &params.text_document.uri;
}

fn offset_from_position(rope: &ropey::Rope, pos: lsp_types::Position) -> Option<usize> {
    crate::convert::lsp_position_to_byte_offset(rope, pos)
}

fn advance_point_by_text(start: tree_sitter::Point, text: &str) -> tree_sitter::Point {
    let mut row = start.row;
    let mut col = start.column;
    for ch in text.chars() {
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += ch.len_utf8();
        }
    }
    tree_sitter::Point { row, column: col }
}
