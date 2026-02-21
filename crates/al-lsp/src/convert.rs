use lsp_types::Position;

/// Convert a tree-sitter `Point` to an LSP `Position`.
/// tree-sitter uses 0-based row/column (byte offsets), LSP uses 0-based line/character (UTF-16).
/// For ASCII-dominated AL code, byte offset ≈ UTF-16 offset.
pub fn ts_point_to_lsp_position(point: tree_sitter::Point) -> Position {
    Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}

/// Convert an LSP `Position` to a tree-sitter `Point`.
pub fn lsp_position_to_ts_point(pos: Position) -> tree_sitter::Point {
    tree_sitter::Point {
        row: pos.line as usize,
        column: pos.character as usize,
    }
}

/// Convert an LSP `Position` to a byte offset in a Rope.
pub fn lsp_position_to_byte_offset(rope: &ropey::Rope, pos: Position) -> Option<usize> {
    let line = pos.line as usize;
    if line >= rope.len_lines() {
        return None;
    }
    let line_start = rope.line_to_byte(line);
    let col = pos.character as usize;
    Some(line_start + col)
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
