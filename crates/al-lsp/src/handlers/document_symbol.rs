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
        .filter_map(al_symbol_to_document_symbol)
        .collect();

    Some(DocumentSymbolResponse::Nested(symbols))
}

#[allow(deprecated)] // DocumentSymbol::deprecated field is deprecated in the type itself
fn al_symbol_to_document_symbol(sym: &AlSymbol) -> Option<DocumentSymbol> {
    if sym.name.trim().is_empty() {
        return None;
    }

    let range = ts_range_to_lsp_range(sym.start_point, sym.end_point);
    let selection_range = range;

    let children = if sym.children.is_empty() {
        None
    } else {
        let children: Vec<DocumentSymbol> = sym
            .children
            .iter()
            .filter_map(al_symbol_to_document_symbol)
            .collect();
        if children.is_empty() {
            None
        } else {
            Some(children)
        }
    };

    let detail = sym.type_info.clone();

    Some(DocumentSymbol {
        name: sym.name.clone(),
        detail,
        kind: sym.kind.lsp_kind(),
        tags: None,
        deprecated: None,
        range,
        selection_range,
        children,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use al_syntax::ast::{AlObjectKind, AlSymbolKind};

    fn make_symbol(name: &str) -> AlSymbol {
        AlSymbol {
            name: name.to_string(),
            kind: AlSymbolKind::Object(AlObjectKind::Codeunit),
            type_info: Some("codeunit".to_string()),
            implements: Vec::new(),
            start_byte: 0,
            end_byte: 0,
            start_point: tree_sitter::Point::new(0, 0),
            end_point: tree_sitter::Point::new(0, 0),
            name_start_point: tree_sitter::Point::new(0, 0),
            name_end_point: tree_sitter::Point::new(0, 0),
            children: Vec::new(),
        }
    }

    #[test]
    fn test_document_symbol_ignores_empty_name() {
        let sym = make_symbol("");
        assert!(al_symbol_to_document_symbol(&sym).is_none());
    }
}
