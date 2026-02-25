use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

use al_syntax::ast::AlSymbolKind;
use al_syntax::navigation::{identifier_at_offset, resolve_at_offset};
use al_syntax::symbols::format_hover;

use crate::convert::lsp_position_to_byte_offset;
use crate::handlers::completion::enum_value_target_at_offset;
use crate::handlers::events::{event_subscriber_context_at_offset, find_event_publishers};
use crate::state::WorldState;

pub fn handle_hover(state: &WorldState, params: HoverParams) -> Option<Hover> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();
    let enum_target = enum_value_target_at_offset(state, &uri, &doc.tree, &source, byte_offset);
    drop(doc);

    if let Some((enum_name, value_name)) = enum_target {
        for entry in state.documents.iter() {
            for object in &entry.value().symbol_table.symbols {
                if let AlSymbolKind::Object(kind) = object.kind {
                    if kind.label() != "enum" || !object.name.eq_ignore_ascii_case(&enum_name) {
                        continue;
                    }
                    for child in &object.children {
                        if matches!(child.kind, AlSymbolKind::EnumValue)
                            && child.name.eq_ignore_ascii_case(&value_name)
                        {
                            return Some(Hover {
                                contents: HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: format_hover(child),
                                }),
                                range: None,
                            });
                        }
                    }
                }
            }
        }
    }

    let doc = state.documents.get(&uri)?;
    let source = doc.source();
    if let Some(ctx) = event_subscriber_context_at_offset(&doc.tree, &source, byte_offset) {
        if ctx.arg_index >= 2 {
            if let Some(target) = ctx.target {
                drop(doc);
                let publishers = find_event_publishers(state, &target);
                if !publishers.is_empty() {
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: format!(
                                "```al\n{} {}::{}\n```",
                                target.object_kind, target.object_name, target.event_name
                            ),
                        }),
                        range: None,
                    });
                }
            }
        }
    }

    let doc = state.documents.get(&uri)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Url};

    fn make_hover_params(uri: Url, line: u32, character: u32) -> HoverParams {
        HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
        }
    }

    #[test]
    fn test_hover_on_qualified_enum_value() {
        let source = r#"enum 50100 MyEnum
{
    value(0; First)
    {
    }
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := MyEnum::First;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on `First` in `MyEnum::First` (line 13)
        let params = make_hover_params(uri, 13, 21);
        let hover = handle_hover(&state, params);
        assert!(hover.is_some(), "expected hover result");
        let hover = hover.unwrap();
        let HoverContents::Markup(content) = hover.contents else {
            panic!("expected markdown hover");
        };
        assert!(
            content.value.contains("enum value") && content.value.contains("First"),
            "expected enum value hover, got: {}",
            content.value
        );
    }

    #[test]
    fn test_hover_on_record_field_qualified_enum_value() {
        let enum_source = r#"enum 50100 "Document Type"
{
    value(0; Order)
    {
    }
    value(1; Invoice)
    {
    }
}"#;
        let table_source = r#"table 50100 "Sales Header"
{
    fields
    {
        field(1; "Document Type"; Enum "Document Type")
        {
        }
    }
}"#;
        let codeunit_source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Rec: Record "Sales Header";
    begin
        if Rec."Document Type" = Rec."Document Type"::Order then;
    end;
}"#;

        let enum_uri = Url::parse("file:///test/enum.al").unwrap();
        let table_uri = Url::parse("file:///test/table.al").unwrap();
        let codeunit_uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(enum_uri, DocumentState::new(enum_source).unwrap());
        state
            .documents
            .insert(table_uri, DocumentState::new(table_source).unwrap());
        state.documents.insert(
            codeunit_uri.clone(),
            DocumentState::new(codeunit_source).unwrap(),
        );

        // Cursor on `Order` in `Rec."Document Type"::Order` (line 6).
        let params = make_hover_params(codeunit_uri, 6, 47);
        let hover = handle_hover(&state, params);
        assert!(hover.is_some(), "expected hover result");
        let hover = hover.unwrap();
        let HoverContents::Markup(content) = hover.contents else {
            panic!("expected markdown hover");
        };
        assert!(
            content.value.contains("enum value") && content.value.contains("Order"),
            "expected enum value hover, got: {}",
            content.value
        );
    }
}
