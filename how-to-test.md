  cargo install --path crates/al-lsp && \
    cd editors/vscode && npm install && npm run compile && cd ../.. && \
    code --extensionDevelopmentPath=$PWD/editors/vscode test-fixtures/