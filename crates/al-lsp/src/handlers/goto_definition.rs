use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use al_syntax::navigation::resolve_at_offset;

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_goto_definition(
    state: &WorldState,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);

    Some(GotoDefinitionResponse::Scalar(Location {
        uri: uri.clone(),
        range,
    }))
}
