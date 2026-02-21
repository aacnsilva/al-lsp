use lsp_types::{Location, SymbolInformation, WorkspaceSymbolParams};

use al_syntax::ast::AlSymbol;

use crate::convert::ts_range_to_lsp_range;
use crate::state::WorldState;

#[allow(deprecated)] // SymbolInformation::deprecated field is deprecated in the type itself
pub fn handle_workspace_symbol(
    state: &WorldState,
    params: WorkspaceSymbolParams,
) -> Option<Vec<SymbolInformation>> {
    let query = params.query.to_lowercase();

    let mut symbols = Vec::new();

    for entry in state.documents.iter() {
        let uri = entry.key().clone();
        let doc = entry.value();

        for obj in doc.symbols() {
            collect_symbols_flat(obj, &uri, &query, &mut symbols);
        }
    }

    if symbols.is_empty() {
        return None;
    }

    Some(symbols)
}

#[allow(deprecated)]
fn collect_symbols_flat(
    sym: &AlSymbol,
    uri: &lsp_types::Url,
    query: &str,
    result: &mut Vec<SymbolInformation>,
) {
    if query.is_empty() || sym.name.to_lowercase().contains(query) {
        let range = ts_range_to_lsp_range(sym.start_point, sym.end_point);
        result.push(SymbolInformation {
            name: sym.name.clone(),
            kind: sym.kind.lsp_kind(),
            tags: None,
            deprecated: None,
            location: Location {
                uri: uri.clone(),
                range,
            },
            container_name: None,
        });
    }

    for child in &sym.children {
        collect_symbols_flat(child, uri, query, result);
    }
}
