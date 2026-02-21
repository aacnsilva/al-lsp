use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};

use al_syntax::navigation::{collect_folding_ranges, FoldingAreaKind};

use crate::state::WorldState;

pub fn handle_folding_range(
    state: &WorldState,
    params: FoldingRangeParams,
) -> Option<Vec<FoldingRange>> {
    let uri = params.text_document.uri;
    let doc = state.documents.get(&uri)?;

    let areas = collect_folding_ranges(&doc.tree);

    if areas.is_empty() {
        return None;
    }

    let ranges: Vec<FoldingRange> = areas
        .into_iter()
        .map(|area| FoldingRange {
            start_line: area.start_line as u32,
            start_character: None,
            end_line: area.end_line as u32,
            end_character: None,
            kind: Some(match area.kind {
                FoldingAreaKind::Comment => FoldingRangeKind::Comment,
                FoldingAreaKind::Region => FoldingRangeKind::Region,
            }),
            collapsed_text: None,
        })
        .collect();

    Some(ranges)
}
