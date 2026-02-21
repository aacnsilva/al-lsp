use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use al_syntax::navigation::resolve_at_offset;

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_goto_definition(
    state: &WorldState,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    // Check if cursor is on a procedure inside a codeunit that implements interfaces.
    // If so, navigate to the corresponding interface method definition.
    if let Some((iface_names, method_name)) =
        doc.symbol_table.implementation_procedure_at(byte_offset)
    {
        let iface_names: Vec<String> = iface_names.to_vec();
        let method_name = method_name.to_string();
        drop(doc); // Release the DashMap ref before iterating

        let mut locations = Vec::new();
        for entry in state.documents.iter() {
            let other_doc = entry.value();
            for iface_name in &iface_names {
                if let Some(method_sym) = other_doc
                    .symbol_table
                    .find_interface_method(iface_name, &method_name)
                {
                    locations.push(Location {
                        uri: entry.key().clone(),
                        range: ts_range_to_lsp_range(method_sym.start_point, method_sym.end_point),
                    });
                }
            }
        }

        if !locations.is_empty() {
            if locations.len() == 1 {
                return Some(GotoDefinitionResponse::Scalar(
                    locations.into_iter().next().unwrap(),
                ));
            }
            return Some(GotoDefinitionResponse::Array(locations));
        }

        // If no interface method found, fall through to normal resolution.
        let doc = state.documents.get(&uri)?;
        let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
        let source = doc.source();
        let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
        let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);
        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range,
        }));
    }

    let resolved = resolve_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    let range = ts_range_to_lsp_range(resolved.symbol.start_point, resolved.symbol.end_point);

    Some(GotoDefinitionResponse::Scalar(Location {
        uri: uri.clone(),
        range,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{
        GotoDefinitionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Url,
    };

    /// Helper: build GotoDefinitionParams for a given URI and position.
    fn make_goto_params(uri: Url, line: u32, character: u32) -> GotoDefinitionParams {
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    /// Extract all locations from a GotoDefinitionResponse.
    fn locations_from(resp: GotoDefinitionResponse) -> Vec<Location> {
        match resp {
            GotoDefinitionResponse::Scalar(loc) => vec![loc],
            GotoDefinitionResponse::Array(locs) => locs,
            GotoDefinitionResponse::Link(_) => vec![],
        }
    }

    #[test]
    fn test_goto_definition_impl_procedure_to_interface_method_cross_doc() {
        // Cursor on GetAddress in the CODEUNIT -> should go to GetAddress in the INTERFACE.
        let iface_source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}"#;
        let impl_source = r#"codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;
}"#;
        let iface_uri = Url::parse("file:///test/iface.al").unwrap();
        let impl_uri = Url::parse("file:///test/impl.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(iface_uri.clone(), DocumentState::new(iface_source).unwrap());
        state
            .documents
            .insert(impl_uri.clone(), DocumentState::new(impl_source).unwrap());

        // Cursor on "GetAddress" in the codeunit (line 2, col 14)
        let params = make_goto_params(impl_uri.clone(), 2, 14);
        let result = handle_goto_definition(&state, params);

        eprintln!("impl->iface result: {result:?}");
        assert!(
            result.is_some(),
            "expected goto-definition to return a result"
        );

        let locs = locations_from(result.unwrap());
        eprintln!("impl->iface locations: {locs:?}");

        // Should point to the interface document's GetAddress method
        assert!(
            locs.iter().any(|l| l.uri == iface_uri),
            "expected a location in the interface document, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_impl_procedure_to_interface_method_same_doc() {
        // Both in the same document.
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/address.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "GetAddress" in the codeunit (line 7, col 14)
        let params = make_goto_params(uri.clone(), 7, 14);
        let result = handle_goto_definition(&state, params);

        eprintln!("same-doc impl->iface result: {result:?}");
        assert!(
            result.is_some(),
            "expected goto-definition to return a result"
        );

        let locs = locations_from(result.unwrap());
        eprintln!("same-doc impl->iface locations: {locs:?}");

        // Should point to the interface method on line 2 (0-indexed)
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected a location pointing to line 2 (the interface method), got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_definition_non_interface_procedure_unchanged() {
        // Regular procedure call — should still resolve within the same document.
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;

    procedure DoWork()
    begin
        Hello()
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "Hello" in the call (line 8, col 8)
        let params = make_goto_params(uri.clone(), 8, 8);
        let result = handle_goto_definition(&state, params);

        eprintln!("non-iface result: {result:?}");
        assert!(result.is_some(), "expected goto-definition for Hello call");

        let locs = locations_from(result.unwrap());
        // Should point to the Hello definition on line 2
        assert!(
            locs.iter().any(|l| l.range.start.line == 2),
            "expected definition on line 2, got: {locs:?}"
        );
    }
}
