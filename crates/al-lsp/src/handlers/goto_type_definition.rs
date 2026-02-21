use lsp_types::{GotoDefinitionResponse, Location, request::GotoTypeDefinitionParams};

use al_syntax::navigation::{extract_type_object_name, identifier_context_at_offset};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_goto_type_definition(
    state: &WorldState,
    params: GotoTypeDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let ctx = identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
    let sym = ctx.symbol?;

    let type_info = sym.type_info.as_deref()?;
    let object_name = extract_type_object_name(type_info)?;

    // Search in same document first
    if let Some(obj) = doc.symbol_table.find_object_by_name(object_name) {
        let range = ts_range_to_lsp_range(obj.start_point, obj.end_point);
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range,
        }));
    }

    // Search in other open documents
    for entry in state.documents.iter() {
        if *entry.key() == uri {
            continue;
        }
        if let Some(obj) = entry.value().symbol_table.find_object_by_name(object_name) {
            let range = ts_range_to_lsp_range(obj.start_point, obj.end_point);
            return Some(GotoDefinitionResponse::Scalar(Location {
                uri: entry.key().clone(),
                range,
            }));
        }
    }

    None
}
