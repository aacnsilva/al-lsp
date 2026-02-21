use lsp_types::{Location, ReferenceParams};

use al_syntax::navigation::find_all_references;

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_references(state: &WorldState, params: ReferenceParams) -> Option<Vec<Location>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let include_declaration = params.context.include_declaration;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let refs = find_all_references(
        &doc.tree,
        &source,
        &doc.symbol_table,
        byte_offset,
        include_declaration,
    );

    let mut locations: Vec<Location> = refs
        .into_iter()
        .map(|(start, end)| Location {
            uri: uri.clone(),
            range: ts_range_to_lsp_range(start, end),
        })
        .collect();

    // If cursor is on a procedure inside a codeunit that implements interfaces,
    // also include the interface method definition as a reference.
    if let Some((iface_names, method_name)) =
        doc.symbol_table.implementation_procedure_at(byte_offset)
    {
        let iface_names: Vec<String> = iface_names.to_vec();
        let method_name = method_name.to_string();
        drop(doc); // Release the DashMap ref before iterating

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
    }

    if locations.is_empty() {
        return None;
    }

    Some(locations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{
        Position, ReferenceContext, ReferenceParams, TextDocumentIdentifier,
        TextDocumentPositionParams, Url,
    };

    /// Helper: build ReferenceParams for a given URI, position, and include_declaration flag.
    fn make_ref_params(
        uri: Url,
        line: u32,
        character: u32,
        include_declaration: bool,
    ) -> ReferenceParams {
        ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            context: ReferenceContext {
                include_declaration,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        }
    }

    #[test]
    fn test_references_impl_procedure_includes_interface_method() {
        // Cursor on GetAddress in the CODEUNIT -> references should include
        // GetAddress in the INTERFACE.
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

        // Cursor on "GetAddress" in the codeunit (line 2, col 14), include declaration
        let params = make_ref_params(impl_uri.clone(), 2, 14, true);
        let result = handle_references(&state, params);

        assert!(result.is_some(), "expected references to return results");
        let locs = result.unwrap();

        // Should include the interface method as a reference
        assert!(
            locs.iter().any(|l| l.uri == iface_uri),
            "expected a reference in the interface document, got: {locs:?}"
        );
    }

    #[test]
    fn test_references_interface_method_does_not_include_impls() {
        // Cursor on GetAddress in the INTERFACE -> references should NOT include
        // implementing procedures (use Go to Implementation for that).
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

        // Cursor on "GetAddress" in the interface (line 2, col 14), include declaration
        let params = make_ref_params(iface_uri.clone(), 2, 14, true);
        let result = handle_references(&state, params);

        // Should only have same-document references (the declaration itself), not impl procedures
        if let Some(locs) = &result {
            assert!(
                !locs.iter().any(|l| l.uri == impl_uri),
                "did NOT expect impl procedures in references, got: {locs:?}"
            );
        }
    }
}
