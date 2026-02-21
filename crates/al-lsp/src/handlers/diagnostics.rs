use lsp_types::Url;
use tower_lsp::Client;

use al_syntax::document::DocumentState;

pub async fn publish_diagnostics(client: &Client, uri: &Url, doc: &DocumentState) {
    client
        .publish_diagnostics(uri.clone(), doc.diagnostics.clone(), None)
        .await;
}
