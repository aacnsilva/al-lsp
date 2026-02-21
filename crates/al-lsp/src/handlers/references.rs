use lsp_types::{Location, ReferenceParams};

use al_syntax::navigation::find_all_references;

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_references(state: &WorldState, params: ReferenceParams) -> Option<Vec<Location>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let include_declaration = params.context.include_declaration;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let refs = find_all_references(
        &doc.tree,
        &source,
        &doc.symbol_table,
        byte_offset,
        include_declaration,
    );

    if refs.is_empty() {
        return None;
    }

    let locations: Vec<Location> = refs
        .into_iter()
        .map(|(start, end)| Location {
            uri: uri.clone(),
            range: ts_range_to_lsp_range(start, end),
        })
        .collect();

    Some(locations)
}
