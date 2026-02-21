use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

use al_syntax::navigation::{identifier_at_offset, resolve_at_offset};
use al_syntax::symbols::format_hover;

use crate::convert::lsp_position_to_byte_offset;
use crate::state::WorldState;

pub fn handle_hover(state: &WorldState, params: HoverParams) -> Option<Hover> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    // First try to resolve to a definition
    if let Some(resolved) = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format_hover(resolved.symbol),
            }),
            range: None,
        });
    }

    // If we're on a definition itself, show its hover
    let name = identifier_at_offset(&doc.tree, &source, byte_offset)?;
    let symbols = doc.symbol_table.lookup(&name);
    let sym = symbols.into_iter().next()?;

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format_hover(sym),
        }),
        range: None,
    })
}
