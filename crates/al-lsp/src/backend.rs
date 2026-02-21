use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::handlers::{
    completion, document_highlight, document_symbol, document_sync, folding_range,
    goto_definition, goto_type_definition, hover, references, rename, signature_help,
    workspace_symbol,
};
use crate::state::WorldState;

pub struct AlBackend {
    pub client: Client,
    pub state: WorldState,
}

#[tower_lsp::async_trait]
impl LanguageServer for AlBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                })),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "al-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("AL Language Server initialized");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        tracing::info!("opened: {}", params.text_document.uri);
        document_sync::handle_did_open(&self.client, &self.state, params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        document_sync::handle_did_change(&self.client, &self.state, params).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        tracing::info!("closed: {}", params.text_document.uri);
        document_sync::handle_did_close(&self.state, params).await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(goto_definition::handle_goto_definition(&self.state, params))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        Ok(hover::handle_hover(&self.state, params))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        Ok(document_symbol::handle_document_symbol(&self.state, params))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(references::handle_references(&self.state, params))
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        Ok(document_highlight::handle_document_highlight(&self.state, params))
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        Ok(rename::handle_prepare_rename(&self.state, params))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(rename::handle_rename(&self.state, params))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(completion::handle_completion(&self.state, params))
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        Ok(signature_help::handle_signature_help(&self.state, params))
    }

    async fn goto_type_definition(
        &self,
        params: request::GotoTypeDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(goto_type_definition::handle_goto_type_definition(&self.state, params))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        Ok(folding_range::handle_folding_range(&self.state, params))
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        Ok(workspace_symbol::handle_workspace_symbol(&self.state, params))
    }
}
