use lsp_types::{DocumentHighlight, DocumentHighlightKind, DocumentHighlightParams};

use al_syntax::navigation::find_all_references;

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_document_highlight(
    state: &WorldState,
    params: DocumentHighlightParams,
) -> Option<Vec<DocumentHighlight>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    // Always include declaration for highlight
    let refs = find_all_references(&doc.tree, &source, &doc.symbol_table, byte_offset, true);

    if refs.is_empty() {
        return None;
    }

    let highlights: Vec<DocumentHighlight> = refs
        .into_iter()
        .map(|(start, end)| {
            let range = ts_range_to_lsp_range(start, end);
            // Classify as Write if the node is on the left side of an assignment.
            // For simplicity, we mark all as Read — a more precise classification
            // would require checking the parent node type at each reference site.
            DocumentHighlight {
                range,
                kind: Some(DocumentHighlightKind::READ),
            }
        })
        .collect();

    Some(highlights)
}
