mod backend;
mod convert;
mod handlers;
mod state;

use tower_lsp::{LspService, Server};

use backend::AlBackend;
use state::WorldState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| AlBackend {
        client,
        state: WorldState::new(),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
