use lsp_types::{request::GotoImplementationParams, GotoDefinitionResponse, Location};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

/// Handle `textDocument/implementation`.
///
/// When the cursor is on a procedure inside an interface, returns all
/// implementing procedures across open documents.
pub fn handle_goto_implementation(
    state: &WorldState,
    params: GotoImplementationParams,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;

    // Only meaningful when cursor is on an interface method.
    let (interface_name, method_name) = doc.symbol_table.interface_method_at(byte_offset)?;
    let interface_name = interface_name.to_string();
    let method_name = method_name.to_string();
    drop(doc); // Release the DashMap ref before iterating

    let mut locations = Vec::new();
    for entry in state.documents.iter() {
        let other_doc = entry.value();
        let impls = other_doc
            .symbol_table
            .find_implementation_procedures(&interface_name, &method_name);
        for proc_sym in impls {
            locations.push(Location {
                uri: entry.key().clone(),
                range: ts_range_to_lsp_range(proc_sym.start_point, proc_sym.end_point),
            });
        }
    }

    if locations.is_empty() {
        return None;
    }

    if locations.len() == 1 {
        return Some(GotoDefinitionResponse::Scalar(
            locations.into_iter().next().unwrap(),
        ));
    }
    Some(GotoDefinitionResponse::Array(locations))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Url};

    fn make_impl_params(uri: Url, line: u32, character: u32) -> GotoImplementationParams {
        GotoImplementationParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    fn locations_from(resp: GotoDefinitionResponse) -> Vec<Location> {
        match resp {
            GotoDefinitionResponse::Scalar(loc) => vec![loc],
            GotoDefinitionResponse::Array(locs) => locs,
            GotoDefinitionResponse::Link(_) => vec![],
        }
    }

    #[test]
    fn test_goto_implementation_interface_to_impl_cross_doc() {
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

        // Cursor on "GetAddress" in the interface (line 2, col 14)
        let params = make_impl_params(iface_uri.clone(), 2, 14);
        let result = handle_goto_implementation(&state, params);

        assert!(
            result.is_some(),
            "expected go-to-implementation to return a result"
        );
        let locs = locations_from(result.unwrap());
        assert!(
            locs.iter().any(|l| l.uri == impl_uri),
            "expected a location in the impl document, got: {locs:?}"
        );
    }

    #[test]
    fn test_goto_implementation_multiple_impls() {
        let iface_source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}"#;
        let impl1_source = r#"codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;
}"#;
        let impl2_source = r#"codeunit 50201 PersonAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;
}"#;
        let iface_uri = Url::parse("file:///test/iface.al").unwrap();
        let impl1_uri = Url::parse("file:///test/impl1.al").unwrap();
        let impl2_uri = Url::parse("file:///test/impl2.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(iface_uri.clone(), DocumentState::new(iface_source).unwrap());
        state
            .documents
            .insert(impl1_uri.clone(), DocumentState::new(impl1_source).unwrap());
        state
            .documents
            .insert(impl2_uri.clone(), DocumentState::new(impl2_source).unwrap());

        let params = make_impl_params(iface_uri.clone(), 2, 14);
        let result = handle_goto_implementation(&state, params);

        assert!(
            result.is_some(),
            "expected go-to-implementation to return results"
        );
        let locs = locations_from(result.unwrap());
        assert_eq!(
            locs.len(),
            2,
            "expected 2 implementation locations, got: {locs:?}"
        );
        assert!(locs.iter().any(|l| l.uri == impl1_uri));
        assert!(locs.iter().any(|l| l.uri == impl2_uri));
    }

    #[test]
    fn test_goto_implementation_not_on_interface_returns_none() {
        // Cursor on a regular procedure, not an interface method.
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let params = make_impl_params(uri.clone(), 2, 14);
        let result = handle_goto_implementation(&state, params);

        assert!(
            result.is_none(),
            "expected None for non-interface procedure"
        );
    }
}
