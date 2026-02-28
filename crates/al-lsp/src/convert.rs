use lsp_types::Position;

fn utf16_col_for_line_byte(line_text: &str, byte_col: usize) -> usize {
    let mut seen_bytes = 0usize;
    let mut utf16_col = 0usize;
    for ch in line_text.chars() {
        let ch_bytes = ch.len_utf8();
        if seen_bytes + ch_bytes > byte_col {
            break;
        }
        seen_bytes += ch_bytes;
        utf16_col += ch.len_utf16();
    }
    utf16_col
}

fn line_byte_for_utf16_col(line_text: &str, utf16_col: usize) -> usize {
    let mut seen_utf16 = 0usize;
    let mut seen_bytes = 0usize;
    for ch in line_text.chars() {
        let ch_utf16 = ch.len_utf16();
        if seen_utf16 + ch_utf16 > utf16_col {
            break;
        }
        seen_utf16 += ch_utf16;
        seen_bytes += ch.len_utf8();
    }
    seen_bytes
}

/// Convert a tree-sitter `Point` to an LSP `Position`.
///
/// tree-sitter columns are byte-based; LSP columns are UTF-16 code units.
/// This legacy helper keeps byte-based behavior for callers that do not have
/// source text available.
pub fn ts_point_to_lsp_position(point: tree_sitter::Point) -> Position {
    Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}

/// Convert a tree-sitter `Point` to an LSP `Position` (UTF-16 columns).
pub fn ts_point_to_lsp_position_utf16(rope: &ropey::Rope, point: tree_sitter::Point) -> Position {
    let line = point.row.min(rope.len_lines().saturating_sub(1));
    let line_slice = rope.line(line);
    let line_text = line_slice.to_string();
    let byte_col = point.column.min(line_text.len());
    Position {
        line: line as u32,
        character: utf16_col_for_line_byte(&line_text, byte_col) as u32,
    }
}

/// Convert an LSP `Position` to a byte offset in a Rope.
pub fn lsp_position_to_byte_offset(rope: &ropey::Rope, pos: Position) -> Option<usize> {
    let line = pos.line as usize;
    if line >= rope.len_lines() {
        return None;
    }
    let line_start = rope.line_to_byte(line);
    let line_slice = rope.line(line);
    let line_text = line_slice.to_string();
    let utf16_col = pos.character as usize;
    let line_byte = line_byte_for_utf16_col(&line_text, utf16_col);
    Some(line_start + line_byte)
}

/// Convert a tree-sitter `Point` pair to an LSP `Range`.
pub fn ts_range_to_lsp_range(
    start: tree_sitter::Point,
    end: tree_sitter::Point,
) -> lsp_types::Range {
    lsp_types::Range {
        start: ts_point_to_lsp_position(start),
        end: ts_point_to_lsp_position(end),
    }
}

/// Convert a tree-sitter `Point` pair to an LSP `Range` using UTF-16 columns.
pub fn ts_range_to_lsp_range_utf16(
    rope: &ropey::Rope,
    start: tree_sitter::Point,
    end: tree_sitter::Point,
) -> lsp_types::Range {
    lsp_types::Range {
        start: ts_point_to_lsp_position_utf16(rope, start),
        end: ts_point_to_lsp_position_utf16(rope, end),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_position_to_byte_offset_utf16() {
        let text = "codeunit 50100 Test\n{\n    // 😊\n}\n";
        let rope = ropey::Rope::from_str(text);
        // Line 2: "    // 😊"
        // Emoji is 2 UTF-16 code units and 4 bytes.
        let pos_before_emoji = Position {
            line: 2,
            character: 7,
        };
        let pos_after_emoji = Position {
            line: 2,
            character: 9,
        };
        let b1 = lsp_position_to_byte_offset(&rope, pos_before_emoji).unwrap();
        let b2 = lsp_position_to_byte_offset(&rope, pos_after_emoji).unwrap();
        assert_eq!(&text[b1..b2], "😊");
    }

    #[test]
    fn test_ts_point_to_lsp_position_utf16() {
        let text = "a😊b\n";
        let rope = ropey::Rope::from_str(text);
        let point_on_b = tree_sitter::Point { row: 0, column: 5 }; // "a"(1) + "😊"(4)
        let pos = ts_point_to_lsp_position_utf16(&rope, point_on_b);
        // UTF-16 columns: "a"(1) + "😊"(2) => 3
        assert_eq!(pos.line, 0);
        assert_eq!(pos.character, 3);
    }
}
