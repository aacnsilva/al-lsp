use dashmap::DashMap;
use lsp_types::Url;

use al_syntax::document::DocumentState;

/// Global server state holding all open documents.
pub struct WorldState {
    pub documents: DashMap<Url, DocumentState>,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            documents: DashMap::new(),
        }
    }
}
