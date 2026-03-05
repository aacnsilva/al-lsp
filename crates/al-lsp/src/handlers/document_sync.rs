use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
};
use tower_lsp::Client;

use al_syntax::document::{DocumentState, IncrementalEdit};

use crate::state::WorldState;

use super::diagnostics::{publish_diagnostics, publish_syntax_diagnostics};

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
    let mut has_full_sync_change = false;
    if let Some(mut doc) = state.documents.get_mut(&uri) {
        for change in params.content_changes {
            changed = true;
            if change.range.is_some() {
                // Incremental edit path for low-latency typing diagnostics.
                if let Some(range) = change.range {
                    let start_offset = offset_from_position(&doc.rope, range.start);
                    let end_offset = offset_from_position(&doc.rope, range.end);
                    let start_point = point_from_position(&doc.rope, range.start);
                    let old_end_point = point_from_position(&doc.rope, range.end);
                    if let (Some(start), Some(end), Some(start_point), Some(old_end_point)) =
                        (start_offset, end_offset, start_point, old_end_point)
                    {
                        let new_end_point = advance_point_by_text(start_point, &change.text);
                        doc.apply_edit(IncrementalEdit {
                            start_byte: start,
                            old_end_byte: end,
                            start_point,
                            old_end_point,
                            new_end_point,
                            new_text: &change.text,
                        });
                    } else {
                        // Fallback: apply best-effort range replacement and do a full reparse.
                        let mut source = doc.source().to_string();
                        let start = start_offset
                            .unwrap_or_else(|| offset_from_position_clamped(&doc.rope, range.start))
                            .min(source.len());
                        let mut end = end_offset
                            .unwrap_or_else(|| offset_from_position_clamped(&doc.rope, range.end))
                            .min(source.len());
                        if end < start {
                            end = start;
                        }
                        source.replace_range(start..end, &change.text);
                        doc.reparse_full(&source);
                    }
                }
            } else {
                // Full document sync
                has_full_sync_change = true;
                doc.reparse_full(&change.text);
            }
        }
    }

    if changed {
        state.reindex_document(&uri);
        if let Some(doc_ref) = state.documents.get(&uri) {
            if has_full_sync_change {
                publish_diagnostics(client, state, &uri, &doc_ref).await;
            } else {
                publish_syntax_diagnostics(client, &uri, &doc_ref).await;
            }
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

fn offset_from_position_clamped(rope: &ropey::Rope, pos: lsp_types::Position) -> usize {
    if rope.len_lines() == 0 {
        return 0;
    }
    let mut clamped = pos;
    let max_line = rope.len_lines().saturating_sub(1) as u32;
    if clamped.line > max_line {
        clamped.line = max_line;
        let line_idx = clamped.line as usize;
        clamped.character = rope.line(line_idx).len_chars() as u32;
    }
    crate::convert::lsp_position_to_byte_offset(rope, clamped).unwrap_or(rope.len_bytes())
}

fn point_from_position(rope: &ropey::Rope, pos: lsp_types::Position) -> Option<tree_sitter::Point> {
    let line = pos.line as usize;
    if line >= rope.len_lines() {
        return None;
    }
    let byte_offset = offset_from_position(rope, pos)?;
    let line_start = rope.line_to_byte(line);
    Some(tree_sitter::Point {
        row: line,
        column: byte_offset.saturating_sub(line_start),
    })
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
