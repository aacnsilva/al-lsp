use lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse};

use al_syntax::ast::AlSymbol;

use crate::convert::ts_range_to_lsp_range;
use crate::state::WorldState;

pub fn handle_document_symbol(
    state: &WorldState,
    params: DocumentSymbolParams,
) -> Option<DocumentSymbolResponse> {
    let uri = params.text_document.uri;
    let doc = state.documents.get(&uri)?;

    let symbols: Vec<DocumentSymbol> = doc
        .symbols()
        .iter()
        .map(al_symbol_to_document_symbol)
        .collect();

    Some(DocumentSymbolResponse::Nested(symbols))
}

#[allow(deprecated)] // DocumentSymbol::deprecated field is deprecated in the type itself
fn al_symbol_to_document_symbol(sym: &AlSymbol) -> DocumentSymbol {
    let range = ts_range_to_lsp_range(sym.start_point, sym.end_point);
    let selection_range = range;

    let children = if sym.children.is_empty() {
        None
    } else {
        Some(
            sym.children
                .iter()
                .map(al_symbol_to_document_symbol)
                .collect(),
        )
    };

    let detail = sym.type_info.clone();

    DocumentSymbol {
        name: sym.name.clone(),
        detail,
        kind: sym.kind.lsp_kind(),
        tags: None,
        deprecated: None,
        range,
        selection_range,
        children,
    }
}
