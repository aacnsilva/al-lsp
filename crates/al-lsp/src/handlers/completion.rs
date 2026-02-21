use lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse};

use al_syntax::ast::AlSymbolKind;
use al_syntax::symbols::al_keywords;

use crate::convert::lsp_position_to_byte_offset;
use crate::state::WorldState;

pub fn handle_completion(
    state: &WorldState,
    params: CompletionParams,
) -> Option<CompletionResponse> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;

    // Get the prefix: walk backwards from cursor to find word start
    let source = doc.source();
    let prefix = extract_prefix(&source, byte_offset);
    let prefix_lower = prefix.to_lowercase();

    let mut items = Vec::new();

    // Add reachable symbols
    let reachable = doc.symbol_table.reachable_symbols(byte_offset);
    for sym in reachable {
        if !prefix_lower.is_empty() && !sym.name.to_lowercase().starts_with(&prefix_lower) {
            continue;
        }

        let kind = match sym.kind {
            AlSymbolKind::Object(_) => CompletionItemKind::CLASS,
            AlSymbolKind::Procedure => CompletionItemKind::FUNCTION,
            AlSymbolKind::Trigger => CompletionItemKind::EVENT,
            AlSymbolKind::Variable => CompletionItemKind::VARIABLE,
            AlSymbolKind::Parameter => CompletionItemKind::VARIABLE,
            AlSymbolKind::Field => CompletionItemKind::FIELD,
            AlSymbolKind::Key => CompletionItemKind::KEYWORD,
            AlSymbolKind::EnumValue => CompletionItemKind::ENUM_MEMBER,
        };

        items.push(CompletionItem {
            label: sym.name.clone(),
            kind: Some(kind),
            detail: sym.type_info.clone(),
            ..Default::default()
        });
    }

    // Add keywords
    for &kw in al_keywords() {
        if !prefix_lower.is_empty() && !kw.to_lowercase().starts_with(&prefix_lower) {
            continue;
        }

        items.push(CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        });
    }

    if items.is_empty() {
        return None;
    }

    Some(CompletionResponse::Array(items))
}

/// Extract the word prefix before the cursor position.
fn extract_prefix(source: &str, byte_offset: usize) -> &str {
    let before = &source[..byte_offset.min(source.len())];
    let start = before
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    &before[start..]
}
