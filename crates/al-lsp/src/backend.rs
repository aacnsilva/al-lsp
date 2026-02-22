use std::path::PathBuf;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::handlers::{
    code_action, completion, document_highlight, document_symbol, document_sync, folding_range,
    formatting, goto_definition, goto_implementation, goto_type_definition, hover, references,
    rename, signature_help, workspace_symbol,
};
use crate::state::WorldState;

pub struct AlBackend {
    pub client: Client,
    pub state: WorldState,
}

#[tower_lsp::async_trait]
impl LanguageServer for AlBackend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Capture workspace roots for file scanning.
        // Try workspace_folders first, then root_uri, then root_path.
        let mut roots: Vec<PathBuf> = Vec::new();

        if let Some(folders) = &params.workspace_folders {
            tracing::info!("initialize: received {} workspace folders", folders.len());
            for folder in folders {
                tracing::info!("  workspace folder: {}", folder.uri);
                if let Ok(path) = folder.uri.to_file_path() {
                    roots.push(path);
                }
            }
        }

        if roots.is_empty() {
            if let Some(root_uri) = &params.root_uri {
                tracing::info!("initialize: using root_uri: {}", root_uri);
                if let Ok(path) = root_uri.to_file_path() {
                    roots.push(path);
                }
            }
        }

        #[allow(deprecated)]
        if roots.is_empty() {
            if let Some(root_path) = &params.root_path {
                tracing::info!("initialize: using root_path: {}", root_path);
                let path = PathBuf::from(root_path);
                if path.is_dir() {
                    roots.push(path);
                }
            }
        }

        if roots.is_empty() {
            tracing::warn!("initialize: no workspace root detected from params");
        } else {
            tracing::info!("initialize: resolved {} workspace root(s)", roots.len());
        }

        *self.state.workspace_roots.lock().unwrap() = roots;

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
                implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    ..Default::default()
                }),
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

        let roots = self.state.workspace_roots.lock().unwrap().clone();
        if roots.is_empty() {
            self.client
                .log_message(MessageType::WARNING, "al-lsp: no workspace root detected")
                .await;
        } else {
            for root in &roots {
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("al-lsp: scanning workspace root: {}", root.display()),
                    )
                    .await;
            }
        }

        let count = self.state.load_workspace_files();
        let total = self.state.documents.len();
        let msg = format!("al-lsp: loaded {} .al files from workspace ({} total)", count, total);
        tracing::info!("{}", msg);
        self.client.log_message(MessageType::INFO, &msg).await;
        self.client.show_message(MessageType::INFO, &msg).await;
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

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in &params.changes {
            let uri = &change.uri;
            match change.typ {
                FileChangeType::CREATED | FileChangeType::CHANGED => {
                    // Reload from disk (updates existing or loads new file)
                    self.state.reload_file_from_disk(uri);
                }
                FileChangeType::DELETED => {
                    self.state.documents.remove(uri);
                }
                _ => {}
            }
        }
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        {
            let mut roots = self.state.workspace_roots.lock().unwrap();

            // Remove old folders
            for removed in &params.event.removed {
                if let Ok(path) = removed.uri.to_file_path() {
                    roots.retain(|r| r != &path);
                }
            }

            // Add new folders
            for added in &params.event.added {
                if let Ok(path) = added.uri.to_file_path() {
                    if !roots.contains(&path) {
                        roots.push(path);
                    }
                }
            }
        }

        // Rescan to pick up new files
        let count = self.state.load_workspace_files();
        let msg = format!("al-lsp: workspace folders changed, loaded {} new .al files", count);
        tracing::info!("{}", msg);
        self.client.log_message(MessageType::INFO, &msg).await;
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

    async fn goto_implementation(
        &self,
        params: request::GotoImplementationParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(goto_implementation::handle_goto_implementation(&self.state, params))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        Ok(folding_range::handle_folding_range(&self.state, params))
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        Ok(formatting::handle_formatting(&self.state, params))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<Vec<CodeActionOrCommand>>> {
        Ok(code_action::handle_code_action(&self.state, params))
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        Ok(workspace_symbol::handle_workspace_symbol(&self.state, params))
    }
}
