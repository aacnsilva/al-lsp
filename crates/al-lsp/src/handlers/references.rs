use lsp_types::{Location, ReferenceParams};

use al_syntax::ast::AlSymbolKind;
use al_syntax::navigation::{
    codeunit_method_call_at_offset, find_all_references, find_codeunit_method_calls,
    find_interface_method_calls, identifier_context_at_offset, interface_method_call_at_offset,
};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_references(state: &WorldState, params: ReferenceParams) -> Option<Vec<Location>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let include_declaration = params.context.include_declaration;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    // First, check if the cursor is on the method part of an interface-typed method call
    // (e.g. `AddressProvider.GetAddress()` where AddressProvider is `Interface IAddressProvider`).
    // If so, treat this as a reference query on the interface method itself.
    if let Some((interface_name, method_name)) =
        interface_method_call_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
    {
        drop(doc); // Release the DashMap ref before iterating

        let mut locations = Vec::new();
        for entry in state.documents.iter() {
            let other_doc = entry.value();
            let other_source = other_doc.source();

            // Include the interface method definition if requested
            if include_declaration {
                if let Some(method_sym) = other_doc
                    .symbol_table
                    .find_interface_method(&interface_name, &method_name)
                {
                    locations.push(Location {
                        uri: entry.key().clone(),
                        range: ts_range_to_lsp_range(method_sym.start_point, method_sym.end_point),
                    });
                }
            }

            // Find all call sites on interface-typed variables
            let calls = find_interface_method_calls(
                &other_doc.tree,
                &other_source,
                &other_doc.symbol_table,
                &interface_name,
                &method_name,
            );
            for (start, end) in calls {
                locations.push(Location {
                    uri: entry.key().clone(),
                    range: ts_range_to_lsp_range(start, end),
                });
            }
        }

        if locations.is_empty() {
            return None;
        }
        return Some(locations);
    }

    // Check if cursor is on the method part of a codeunit-typed method call
    // (e.g. `CompanyAddressProvider2.HelloWorld2()` where CompanyAddressProvider2 is `Codeunit CompanyAddressProvider2`).
    if let Some((codeunit_name, method_name)) =
        codeunit_method_call_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
    {
        drop(doc);

        let mut locations = Vec::new();
        for entry in state.documents.iter() {
            let other_doc = entry.value();
            let other_source = other_doc.source();

            // Include the procedure definition in the target codeunit
            if include_declaration {
                if let Some(proc_sym) = other_doc
                    .symbol_table
                    .find_object_procedure(&codeunit_name, &method_name)
                {
                    locations.push(Location {
                        uri: entry.key().clone(),
                        range: ts_range_to_lsp_range(
                            proc_sym.name_start_point,
                            proc_sym.name_end_point,
                        ),
                    });
                }
            }

            // Find all call sites on codeunit-typed variables
            let calls = find_codeunit_method_calls(
                &other_doc.tree,
                &other_source,
                &other_doc.symbol_table,
                &codeunit_name,
                &method_name,
            );
            for (start, end) in calls {
                locations.push(Location {
                    uri: entry.key().clone(),
                    range: ts_range_to_lsp_range(start, end),
                });
            }
        }

        if locations.is_empty() {
            return None;
        }
        return Some(locations);
    }

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

    // Determine what the cursor is actually on. Only enter interface/impl paths
    // when the cursor is on a Procedure identifier.
    let cursor_on_procedure =
        identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
            .and_then(|ctx| ctx.symbol)
            .is_some_and(|sym| matches!(sym.kind, AlSymbolKind::Procedure));

    if !cursor_on_procedure {
        // Not on a procedure identifier — return standard same-document references only.
        if locations.is_empty() {
            return None;
        }
        return Some(locations);
    }

    // Gather cross-document reference info before dropping the doc borrow.
    let iface_method = doc.symbol_table.interface_method_at(byte_offset)
        .map(|(i, m)| (i.to_string(), m.to_string()));
    let impl_proc = doc.symbol_table.implementation_procedure_at(byte_offset)
        .map(|(ifaces, m)| (ifaces.to_vec(), m.to_string()));
    let cu_proc = doc.symbol_table.codeunit_procedure_at(byte_offset)
        .map(|(obj, m)| (obj.to_string(), m.to_string()));

    drop(doc);

    // If cursor is on an interface method, search all documents for call sites
    // where a variable typed as this interface invokes this method.
    if let Some((interface_name, method_name)) = iface_method {
        for entry in state.documents.iter() {
            let other_doc = entry.value();
            let other_source = other_doc.source();
            let calls = find_interface_method_calls(
                &other_doc.tree,
                &other_source,
                &other_doc.symbol_table,
                &interface_name,
                &method_name,
            );
            for (start, end) in calls {
                locations.push(Location {
                    uri: entry.key().clone(),
                    range: ts_range_to_lsp_range(start, end),
                });
            }
        }
    }

    // If cursor is on a procedure inside a codeunit that implements interfaces,
    // also include the interface method definition as a reference.
    if let Some((iface_names, method_name)) = impl_proc {
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

    // If cursor is on a procedure inside any codeunit, find cross-document call sites
    // where a variable typed as this codeunit calls this method.
    if let Some((object_name, method_name)) = cu_proc {
        for entry in state.documents.iter() {
            let other_doc = entry.value();
            let other_source = other_doc.source();
            let calls = find_codeunit_method_calls(
                &other_doc.tree,
                &other_source,
                &other_doc.symbol_table,
                &object_name,
                &method_name,
            );
            for (start, end) in calls {
                locations.push(Location {
                    uri: entry.key().clone(),
                    range: ts_range_to_lsp_range(start, end),
                });
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

    fn full_example_source() -> &'static str {
        r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddressLbl: Label 'Company address \ Denmark 2800';
    begin
        exit(ExampleAddressLbl);
    end;

    procedure HelloWorld()
    var
        AddressProvider: Interface IAddressProvider;
    begin
        AddressProvider.GetAddress();
    end;
}

codeunit 50201 CompanyAddressProvider2 implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddressLbl: Label 'Company address \ Denmark 2800';
    begin
        exit(ExampleAddressLbl);
    end;

    procedure HelloWorld()
    var
        IAddressProvider: Interface IAddressProvider;
    begin
        IAddressProvider.GetAddress();
    end;
}"#
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
    fn test_references_interface_method_finds_call_sites() {
        // Cursor on GetAddress in the INTERFACE -> references should include
        // the call sites AddressProvider.GetAddress() in the codeunits.
        let source = full_example_source();
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "GetAddress" in the interface (line 2, col 14), include declaration
        let params = make_ref_params(uri.clone(), 2, 14, true);
        let result = handle_references(&state, params);

        assert!(result.is_some(), "expected references to return results");
        let locs = result.unwrap();

        let call_site_lines: Vec<u32> = locs.iter().map(|l| l.range.start.line).collect();
        eprintln!("interface method reference lines: {call_site_lines:?}");

        // The two calls: AddressProvider.GetAddress() and IAddressProvider.GetAddress()
        assert!(
            call_site_lines.contains(&18),
            "expected call site on line 18, got: {call_site_lines:?}"
        );
        assert!(
            call_site_lines.contains(&35),
            "expected call site on line 35, got: {call_site_lines:?}"
        );
    }

    #[test]
    fn test_references_interface_method_cross_doc() {
        // Interface and codeunits in separate documents.
        let iface_source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}"#;
        let impl_source = r#"codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    begin
    end;

    procedure HelloWorld()
    var
        AddressProvider: Interface IAddressProvider;
    begin
        AddressProvider.GetAddress();
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
        let params = make_ref_params(iface_uri.clone(), 2, 14, true);
        let result = handle_references(&state, params);

        assert!(result.is_some(), "expected references to return results");
        let locs = result.unwrap();

        // Should find the call site in the impl document
        assert!(
            locs.iter().any(|l| l.uri == impl_uri),
            "expected a call-site reference in the impl document, got: {locs:?}"
        );
    }

    #[test]
    fn test_references_variable_in_impl_codeunit_no_interface_leak() {
        // Cursor on ExampleAddressLbl inside a codeunit that implements an interface.
        // References should only show same-document usages of ExampleAddressLbl,
        // NOT the interface method.
        let source = full_example_source();
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "ExampleAddressLbl" in the var declaration (line 9, col 8)
        let params = make_ref_params(uri.clone(), 9, 8, true);
        let result = handle_references(&state, params);

        assert!(
            result.is_some(),
            "expected references for ExampleAddressLbl"
        );
        let locs = result.unwrap();
        let lines: Vec<u32> = locs.iter().map(|l| l.range.start.line).collect();
        eprintln!("ExampleAddressLbl reference lines: {lines:?}");

        // Should only contain the declaration (line 9) and usage (line 11)
        // Should NOT contain the interface method line (line 2)
        assert!(
            !lines.contains(&2),
            "ExampleAddressLbl references should NOT include the interface method, got: {lines:?}"
        );
    }

    #[test]
    fn test_references_impl_getaddress_no_interface_call_sites() {
        // Cursor on GetAddress in the implementing codeunit.
        // References should include the interface method definition,
        // but NOT the call site AddressProvider.GetAddress() (which calls via the interface).
        let source = full_example_source();
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "GetAddress" in the first codeunit's procedure (line 7, col 14)
        let params = make_ref_params(uri.clone(), 7, 14, true);
        let result = handle_references(&state, params);

        assert!(result.is_some(), "expected references for impl GetAddress");
        let locs = result.unwrap();
        let lines: Vec<u32> = locs.iter().map(|l| l.range.start.line).collect();
        eprintln!("impl GetAddress reference lines: {lines:?}");

        // Should include the interface method (line 2)
        assert!(
            lines.contains(&2),
            "expected interface method (line 2) in references, got: {lines:?}"
        );

        // Should NOT include AddressProvider.GetAddress() call site (line 18)
        // — that's a call via the interface, not a direct call to this implementation
        assert!(
            !lines.contains(&18),
            "impl GetAddress references should NOT include interface call site (line 18), got: {lines:?}"
        );
    }

    #[test]
    fn test_references_on_interface_method_call_resolves_to_interface() {
        // Cursor on GetAddress in `IAddressProvider.GetAddress()` — a call through an
        // interface-typed variable. References should resolve to the INTERFACE method,
        // showing the interface definition and all interface call sites, NOT the
        // codeunit implementation procedure.
        let source = full_example_source();
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "GetAddress" in `IAddressProvider.GetAddress()` (line 35, col 25)
        // Line 35 (0-indexed) = `        IAddressProvider.GetAddress();`
        // 8 spaces + "IAddressProvider." = 25 chars, so GetAddress starts at col 25
        let params = make_ref_params(uri.clone(), 35, 25, true);
        let result = handle_references(&state, params);

        assert!(
            result.is_some(),
            "expected references for GetAddress on interface call"
        );
        let locs = result.unwrap();
        let lines: Vec<u32> = locs.iter().map(|l| l.range.start.line).collect();
        eprintln!("interface method call reference lines: {lines:?}");

        // Should include the interface method definition (line 2)
        assert!(
            lines.contains(&2),
            "expected interface method definition (line 2), got: {lines:?}"
        );

        // Should include both interface call sites (lines 18 and 35)
        assert!(
            lines.contains(&18),
            "expected call site on line 18, got: {lines:?}"
        );
        assert!(
            lines.contains(&35),
            "expected call site on line 35, got: {lines:?}"
        );

        // Should NOT include the implementation procedure declarations (lines 7 and 24)
        assert!(
            !lines.contains(&7),
            "should NOT include impl procedure on line 7, got: {lines:?}"
        );
        assert!(
            !lines.contains(&24),
            "should NOT include impl procedure on line 24, got: {lines:?}"
        );
    }

    #[test]
    fn test_references_codeunit_method_call() {
        // Cursor on HelloWorld2 in the procedure declaration of CompanyAddressProvider2.
        // References should include the call site CompanyAddressProvider2.HelloWorld2().
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddressLbl: Label 'Company address \ Denmark 2800';
    begin
        exit(ExampleAddressLbl);
    end;

    procedure HelloWorld()
    var
        AddressProvider: Interface IAddressProvider;
        CompanyAddressProvider2: codeunit CompanyAddressProvider2;
    begin
        AddressProvider.GetAddress();
        CompanyAddressProvider2.HelloWorld2();
        GetAddress();
    end;
}

codeunit 50201 CompanyAddressProvider2 implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddressLbl: Label 'Company address \ Denmark 2800';
    begin
        exit(ExampleAddressLbl);
    end;

    procedure HelloWorld2()
    var
        Counter: Integer;
        IAddressProvider: Interface IAddressProvider;
    begin
        IAddressProvider.GetAddress();
        repeat
            Counter += 1;
        until Counter = 0;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "HelloWorld2" in the procedure declaration (line 34)
        // Line 34: "    procedure HelloWorld2()"
        // "    procedure " = 14 chars, so HelloWorld2 starts at col 14
        let params = make_ref_params(uri.clone(), 34, 14, true);
        let result = handle_references(&state, params);

        assert!(
            result.is_some(),
            "expected references for HelloWorld2 procedure"
        );
        let locs = result.unwrap();
        let lines: Vec<u32> = locs.iter().map(|l| l.range.start.line).collect();
        eprintln!("HelloWorld2 reference lines: {lines:?}");

        // Should include the call site: CompanyAddressProvider2.HelloWorld2() (line 20)
        assert!(
            lines.contains(&20),
            "expected call site on line 20, got: {lines:?}"
        );
    }

    #[test]
    fn test_references_from_codeunit_method_call_site() {
        // Cursor on HelloWorld2 in the call site CompanyAddressProvider2.HelloWorld2().
        // References should include the procedure definition.
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddressLbl: Label 'Company address \ Denmark 2800';
    begin
        exit(ExampleAddressLbl);
    end;

    procedure HelloWorld()
    var
        AddressProvider: Interface IAddressProvider;
        CompanyAddressProvider2: codeunit CompanyAddressProvider2;
    begin
        AddressProvider.GetAddress();
        CompanyAddressProvider2.HelloWorld2();
        GetAddress();
    end;
}

codeunit 50201 CompanyAddressProvider2 implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddressLbl: Label 'Company address \ Denmark 2800';
    begin
        exit(ExampleAddressLbl);
    end;

    procedure HelloWorld2()
    var
        Counter: Integer;
        IAddressProvider: Interface IAddressProvider;
    begin
        IAddressProvider.GetAddress();
        repeat
            Counter += 1;
        until Counter = 0;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on "HelloWorld2" in the call site (line 20)
        // Line 20: "        CompanyAddressProvider2.HelloWorld2();"
        // "        CompanyAddressProvider2." = 8 + 23 + 1 = 32 chars
        let params = make_ref_params(uri.clone(), 20, 32, true);
        let result = handle_references(&state, params);

        assert!(
            result.is_some(),
            "expected references from codeunit method call"
        );
        let locs = result.unwrap();
        let lines: Vec<u32> = locs.iter().map(|l| l.range.start.line).collect();
        eprintln!("HelloWorld2 from call site reference lines: {lines:?}");

        // Should include the procedure definition (line 34)
        assert!(
            lines.contains(&34),
            "expected procedure definition on line 34, got: {lines:?}"
        );
    }
}
