use lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};

use al_syntax::formatting::{self, FormatOptions};

use crate::state::WorldState;

pub fn handle_formatting(
    state: &WorldState,
    params: DocumentFormattingParams,
) -> Option<Vec<TextEdit>> {
    let uri = params.text_document.uri;
    let doc = state.documents.get(&uri)?;

    let source = doc.rope.to_string();
    let options = FormatOptions {
        tab_size: params.options.tab_size as usize,
        insert_spaces: params.options.insert_spaces,
    };

    let formatted = formatting::format_document(&doc.tree, &source, &options);

    if formatted == source {
        return None;
    }

    // Return a single edit replacing the entire document.
    let line_count = doc.rope.len_lines();
    let last_line = if line_count > 0 { line_count - 1 } else { 0 };
    let last_col = doc.rope.line(last_line).len_chars();

    Some(vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: last_line as u32,
                character: last_col as u32,
            },
        },
        new_text: formatted,
    }])
}
