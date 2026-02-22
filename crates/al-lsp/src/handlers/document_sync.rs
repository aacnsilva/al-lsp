use lsp_types::{DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams};
use tower_lsp::Client;

use al_syntax::document::DocumentState;

use crate::state::WorldState;

use super::diagnostics::publish_diagnostics;

pub async fn handle_did_open(client: &Client, state: &WorldState, params: DidOpenTextDocumentParams) {
    let uri = params.text_document.uri;
    let text = params.text_document.text;

    if let Some(doc) = DocumentState::new(&text) {
        publish_diagnostics(client, &uri, &doc).await;
        state.documents.insert(uri, doc);
    }
}

pub async fn handle_did_change(
    client: &Client,
    state: &WorldState,
    params: DidChangeTextDocumentParams,
) {
    let uri = params.text_document.uri;

    if let Some(mut doc) = state.documents.get_mut(&uri) {
        for change in params.content_changes {
            if change.range.is_some() {
                // Incremental change — for simplicity, do a full reparse.
                // A production implementation would compute InputEdit from the range.
                let mut source = doc.source();
                if let Some(range) = change.range {
                    let start_offset = offset_from_position(&doc.rope, range.start);
                    let end_offset = offset_from_position(&doc.rope, range.end);
                    if let (Some(start), Some(end)) = (start_offset, end_offset) {
                        source.replace_range(start..end, &change.text);
                    }
                }
                doc.reparse_full(&source);
            } else {
                // Full document sync
                doc.reparse_full(&change.text);
            }
        }
        publish_diagnostics(client, &uri, &doc).await;
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
