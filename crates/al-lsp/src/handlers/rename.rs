use lsp_types::{
    PrepareRenameResponse, RenameParams, TextDocumentPositionParams, TextEdit, Url, WorkspaceEdit,
};
use std::collections::HashMap;

use al_syntax::ast::AlSymbolKind;
use al_syntax::navigation::{
    codeunit_method_call_at_offset, find_all_references, find_codeunit_method_calls,
    find_interface_method_calls, identifier_context_at_offset, interface_method_call_at_offset,
};

use crate::convert::{lsp_position_to_byte_offset, ts_range_to_lsp_range};
use crate::state::WorldState;

pub fn handle_prepare_rename(
    state: &WorldState,
    params: TextDocumentPositionParams,
) -> Option<PrepareRenameResponse> {
    let uri = params.text_document.uri;
    let position = params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    // Allow prepare-rename on interface method calls
    if interface_method_call_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
        .is_some()
    {
        let node = al_syntax::navigation::node_at_offset(&doc.tree, byte_offset)?;
        let range = ts_range_to_lsp_range(node.start_position(), node.end_position());
        return Some(PrepareRenameResponse::Range(range));
    }

    // Allow prepare-rename on codeunit method calls
    if codeunit_method_call_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
        .is_some()
    {
        let node = al_syntax::navigation::node_at_offset(&doc.tree, byte_offset)?;
        let range = ts_range_to_lsp_range(node.start_position(), node.end_position());
        return Some(PrepareRenameResponse::Range(range));
    }

    let ctx = identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    // Don't allow renaming triggers or object names (too complex for simple rename)
    if let Some(sym) = ctx.symbol {
        match sym.kind {
            AlSymbolKind::Trigger | AlSymbolKind::Object(_) => {
                return None;
            }
            _ => {}
        }
    }

    let range = ts_range_to_lsp_range(ctx.node.start_position(), ctx.node.end_position());

    Some(PrepareRenameResponse::Range(range))
}

/// Create a TextEdit for renaming, handling AL quoting rules.
fn make_rename_edit(
    start: tree_sitter::Point,
    end: tree_sitter::Point,
    new_name: &str,
) -> TextEdit {
    let range = ts_range_to_lsp_range(start, end);
    let needs_quotes = new_name.contains(' ');
    let new_text = if needs_quotes {
        format!("\"{}\"", new_name)
    } else {
        new_name.to_string()
    };
    TextEdit { range, new_text }
}

/// Insert a rename TextEdit into the changes map for the given URI.
fn insert_edit(changes: &mut HashMap<Url, Vec<TextEdit>>, uri: Url, edit: TextEdit) {
    changes.entry(uri).or_default().push(edit);
}

pub fn handle_rename(state: &WorldState, params: RenameParams) -> Option<WorkspaceEdit> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let new_name = params.new_name;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    // Case 1a: Interface-typed method call (e.g. `AddressProvider.GetAddress()`)
    if let Some((interface_name, method_name)) =
        interface_method_call_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
    {
        drop(doc);
        return Some(rename_interface_method(
            state,
            &interface_name,
            &method_name,
            &new_name,
        ));
    }

    // Case 1b: Codeunit-typed method call (e.g. `MyCodeunit.HelloWorld()`)
    if let Some((codeunit_name, method_name)) =
        codeunit_method_call_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)
    {
        drop(doc);
        return Some(rename_codeunit_procedure(
            state,
            &codeunit_name,
            &method_name,
            &new_name,
        ));
    }

    let ctx = identifier_context_at_offset(&doc.tree, &source, &doc.symbol_table, byte_offset)?;

    // Don't allow renaming triggers or object names
    if let Some(sym) = ctx.symbol {
        match sym.kind {
            AlSymbolKind::Trigger | AlSymbolKind::Object(_) => {
                return None;
            }
            _ => {}
        }
    }

    // Check if cursor is on a procedure identifier for cross-document cases
    let cursor_on_procedure = ctx
        .symbol
        .is_some_and(|sym| matches!(sym.kind, AlSymbolKind::Procedure));

    if cursor_on_procedure {
        // Case 2: Interface method (cursor on procedure inside an interface)
        if let Some((interface_name, method_name)) =
            doc.symbol_table.interface_method_at(byte_offset)
        {
            let interface_name = interface_name.to_string();
            let method_name = method_name.to_string();
            drop(doc);
            return Some(rename_interface_method(
                state,
                &interface_name,
                &method_name,
                &new_name,
            ));
        }

        // Case 3: Implementation procedure (cursor on procedure in `codeunit ... implements IFoo`)
        if let Some((iface_names, method_name)) =
            doc.symbol_table.implementation_procedure_at(byte_offset)
        {
            let iface_names: Vec<String> = iface_names.to_vec();
            let method_name = method_name.to_string();
            drop(doc);
            return Some(rename_implementation_procedure(
                state,
                &uri,
                &iface_names,
                &method_name,
                &new_name,
            ));
        }

        // Case 3b: Procedure inside a codeunit (not an interface)
        if let Some((object_name, method_name)) =
            doc.symbol_table.codeunit_procedure_at(byte_offset)
        {
            let object_name = object_name.to_string();
            let method_name = method_name.to_string();
            drop(doc);
            return Some(rename_codeunit_procedure(
                state,
                &object_name,
                &method_name,
                &new_name,
            ));
        }
    }

    // Case 4: Regular symbol (variables, parameters, fields) — single-document
    let refs = find_all_references(&doc.tree, &source, &doc.symbol_table, byte_offset, true);
    drop(doc);

    if refs.is_empty() {
        return None;
    }

    let mut changes = HashMap::new();
    let edits: Vec<TextEdit> = refs
        .into_iter()
        .map(|(start, end)| make_rename_edit(start, end, &new_name))
        .collect();
    changes.insert(uri, edits);

    Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    })
}

/// Rename an interface method across all documents:
/// - The interface method definition
/// - All implementation procedures matching the method name
/// - All call sites on interface-typed variables
fn rename_interface_method(
    state: &WorldState,
    interface_name: &str,
    method_name: &str,
    new_name: &str,
) -> WorkspaceEdit {
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();

    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();

        // Rename interface method definition
        if let Some(method_sym) = doc
            .symbol_table
            .find_interface_method(interface_name, method_name)
        {
            insert_edit(
                &mut changes,
                entry.key().clone(),
                make_rename_edit(method_sym.name_start_point, method_sym.name_end_point, new_name),
            );
        }

        // Rename implementation procedures
        let impl_procs =
            doc.symbol_table
                .find_implementation_procedures(interface_name, method_name);
        for proc_sym in impl_procs {
            insert_edit(
                &mut changes,
                entry.key().clone(),
                make_rename_edit(proc_sym.name_start_point, proc_sym.name_end_point, new_name),
            );
        }

        // Rename call sites on interface-typed variables
        let calls = find_interface_method_calls(
            &doc.tree,
            &source,
            &doc.symbol_table,
            interface_name,
            method_name,
        );
        for (start, end) in calls {
            insert_edit(
                &mut changes,
                entry.key().clone(),
                make_rename_edit(start, end, new_name),
            );
        }
    }

    WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    }
}

/// Rename an implementation procedure and find the corresponding interface method:
/// - Same-document references (the procedure name + local call sites)
/// - Cross-document interface method definition
fn rename_implementation_procedure(
    state: &WorldState,
    impl_uri: &Url,
    iface_names: &[String],
    method_name: &str,
    new_name: &str,
) -> WorkspaceEdit {
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();

    // Rename same-document references for the implementation procedure
    if let Some(doc) = state.documents.get(impl_uri) {
        let source = doc.source();

        // Find the procedure symbol and rename its name
        for sym in &doc.symbol_table.symbols {
            for child in &sym.children {
                if matches!(child.kind, AlSymbolKind::Procedure)
                    && child.name.to_lowercase() == method_name.to_lowercase()
                {
                    insert_edit(
                        &mut changes,
                        impl_uri.clone(),
                        make_rename_edit(
                            child.name_start_point,
                            child.name_end_point,
                            new_name,
                        ),
                    );

                    // Find all local references to this procedure within the same document
                    let refs = find_all_references(
                        &doc.tree,
                        &source,
                        &doc.symbol_table,
                        child.start_byte,
                        false, // exclude declaration since we already added the name above
                    );
                    for (start, end) in refs {
                        insert_edit(
                            &mut changes,
                            impl_uri.clone(),
                            make_rename_edit(start, end, new_name),
                        );
                    }
                }
            }
        }
    }

    // Cross-document: find and rename the interface method definition
    for entry in state.documents.iter() {
        let doc = entry.value();
        for iface_name in iface_names {
            if let Some(method_sym) = doc
                .symbol_table
                .find_interface_method(iface_name, method_name)
            {
                insert_edit(
                    &mut changes,
                    entry.key().clone(),
                    make_rename_edit(
                        method_sym.name_start_point,
                        method_sym.name_end_point,
                        new_name,
                    ),
                );
            }
        }
    }

    WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    }
}

/// Rename a procedure inside a codeunit across all documents:
/// - The procedure definition name
/// - Same-document unqualified calls
/// - Cross-document qualified calls (via codeunit-typed variables)
fn rename_codeunit_procedure(
    state: &WorldState,
    codeunit_name: &str,
    method_name: &str,
    new_name: &str,
) -> WorkspaceEdit {
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();

    for entry in state.documents.iter() {
        let doc = entry.value();
        let source = doc.source();

        // Rename the procedure definition in the target codeunit
        if let Some(proc_sym) = doc
            .symbol_table
            .find_object_procedure(codeunit_name, method_name)
        {
            insert_edit(
                &mut changes,
                entry.key().clone(),
                make_rename_edit(proc_sym.name_start_point, proc_sym.name_end_point, new_name),
            );

            // Rename unqualified calls within the same codeunit
            let refs = find_all_references(
                &doc.tree,
                &source,
                &doc.symbol_table,
                proc_sym.start_byte,
                false,
            );
            for (start, end) in refs {
                insert_edit(
                    &mut changes,
                    entry.key().clone(),
                    make_rename_edit(start, end, new_name),
                );
            }
        }

        // Rename qualified call sites (codeunit-typed variable calls)
        let calls = find_codeunit_method_calls(
            &doc.tree,
            &source,
            &doc.symbol_table,
            codeunit_name,
            method_name,
        );
        for (start, end) in calls {
            insert_edit(
                &mut changes,
                entry.key().clone(),
                make_rename_edit(start, end, new_name),
            );
        }
    }

    WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Url};

    fn make_rename_params(uri: Url, line: u32, character: u32, new_name: &str) -> RenameParams {
        RenameParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            new_name: new_name.to_string(),
            work_done_progress_params: Default::default(),
        }
    }

    #[test]
    fn test_rename_local_variable() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        MyVar: Integer;
    begin
        MyVar := 42;
    end;
}"#;
        let uri = Url::parse("file:///test/test.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on MyVar usage (line 6, col 8)
        let params = make_rename_params(uri.clone(), 6, 8, "NewVar");
        let result = handle_rename(&state, params);

        assert!(result.is_some());
        let edit = result.unwrap();
        let changes = edit.changes.unwrap();
        let edits = changes.get(&uri).unwrap();
        // Declaration + usage = at least 2
        assert!(edits.len() >= 2, "expected at least 2 edits, got {}", edits.len());
        assert!(edits.iter().all(|e| e.new_text == "NewVar"));
    }

    #[test]
    fn test_rename_interface_method_from_interface() {
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
        Addr: Interface IAddressProvider;
    begin
        Addr.GetAddress();
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

        // Rename from interface method definition (line 2, col 14 = "GetAddress")
        let params = make_rename_params(iface_uri.clone(), 2, 14, "FetchAddress");
        let result = handle_rename(&state, params);

        assert!(result.is_some(), "expected rename result");
        let edit = result.unwrap();
        let changes = edit.changes.unwrap();

        // Should have edits in the interface document
        let iface_edits = changes.get(&iface_uri);
        assert!(
            iface_edits.is_some(),
            "expected edits in interface document"
        );
        assert!(iface_edits.unwrap().iter().all(|e| e.new_text == "FetchAddress"));

        // Should have edits in the implementation document (procedure name + call site)
        let impl_edits = changes.get(&impl_uri);
        assert!(
            impl_edits.is_some(),
            "expected edits in implementation document"
        );
        let impl_edit_texts: Vec<&str> = impl_edits.unwrap().iter().map(|e| e.new_text.as_str()).collect();
        assert!(
            impl_edit_texts.iter().all(|t| *t == "FetchAddress"),
            "all impl edits should rename to FetchAddress, got: {:?}",
            impl_edit_texts
        );
        // At least the procedure name and the call site
        assert!(
            impl_edits.unwrap().len() >= 2,
            "expected at least 2 edits in impl (proc name + call site), got {}",
            impl_edits.unwrap().len()
        );
    }

    #[test]
    fn test_rename_from_implementation_procedure() {
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

        // Rename from implementation procedure (line 2, col 14 = "GetAddress")
        let params = make_rename_params(impl_uri.clone(), 2, 14, "FetchAddress");
        let result = handle_rename(&state, params);

        assert!(result.is_some(), "expected rename result");
        let edit = result.unwrap();
        let changes = edit.changes.unwrap();

        // Should have edits in the interface document
        assert!(
            changes.get(&iface_uri).is_some(),
            "expected edits in interface document"
        );

        // Should have edits in the implementation document
        assert!(
            changes.get(&impl_uri).is_some(),
            "expected edits in implementation document"
        );
    }

    #[test]
    fn test_rename_from_method_call() {
        let source = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}

codeunit 50200 CompanyAddressProvider implements IAddressProvider
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
        let uri = Url::parse("file:///test/all.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Cursor on GetAddress in `AddressProvider.GetAddress()` (line 15)
        // "        AddressProvider.GetAddress();"
        // 8 spaces + "AddressProvider." = 24, so GetAddress starts at col 24
        let params = make_rename_params(uri.clone(), 15, 24, "FetchAddress");
        let result = handle_rename(&state, params);

        assert!(result.is_some(), "expected rename result");
        let edit = result.unwrap();
        let changes = edit.changes.unwrap();
        let edits = changes.get(&uri).unwrap();

        // Should rename: interface method def, impl procedure name, call site = 3 edits
        assert!(
            edits.len() >= 3,
            "expected at least 3 edits (iface def + impl name + call site), got {}",
            edits.len()
        );
        assert!(edits.iter().all(|e| e.new_text == "FetchAddress"));
    }

    #[test]
    fn test_rename_variable_no_cross_doc_leak() {
        let source = r#"codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure GetAddress(): Text
    var
        ExampleAddr: Text;
    begin
        exit(ExampleAddr);
    end;
}"#;
        let uri = Url::parse("file:///test/impl.al").unwrap();

        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        // Rename ExampleAddr (line 4, col 8)
        let params = make_rename_params(uri.clone(), 4, 8, "AddressText");
        let result = handle_rename(&state, params);

        assert!(result.is_some());
        let edit = result.unwrap();
        let changes = edit.changes.unwrap();
        // Only edits in the same document
        assert_eq!(changes.len(), 1);
        assert!(changes.contains_key(&uri));
    }
}
