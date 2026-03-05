use lsp_types::{
    ParameterInformation, ParameterLabel, SignatureHelp, SignatureHelpParams, SignatureInformation,
};

use al_syntax::ast::extract_name;
use al_syntax::ast::AlSymbolKind;
use al_syntax::navigation::{find_call_context, node_at_offset};

use crate::builtins::find_builtin_method;
use crate::convert::lsp_position_to_byte_offset;
use crate::handlers::completion::resolve_object_type_from_expression;
use crate::state::WorldState;

pub fn handle_signature_help(
    state: &WorldState,
    params: SignatureHelpParams,
) -> Option<SignatureHelp> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = state.documents.get(&uri)?;
    let byte_offset = lsp_position_to_byte_offset(&doc.rope, position)?;
    let source = doc.source();

    let ctx = find_call_context(&doc.tree, &source, &doc.symbol_table, byte_offset)?;
    if let Some(sym) = ctx.symbol {
        // Build parameter list from the procedure's Parameter children
        let parameters: Vec<ParameterInformation> = sym
            .children
            .iter()
            .filter(|c| matches!(c.kind, AlSymbolKind::Parameter))
            .map(|p| {
                let label = if let Some(ref t) = p.type_info {
                    format!("{}: {}", p.name, t)
                } else {
                    p.name.clone()
                };
                ParameterInformation {
                    label: ParameterLabel::Simple(label),
                    documentation: None,
                }
            })
            .collect();

        // Build the signature label: "ProcName(param1: Type, param2: Type): ReturnType"
        let params_str = parameters
            .iter()
            .map(|p| match &p.label {
                ParameterLabel::Simple(s) => s.clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>()
            .join(", ");

        let return_str = sym
            .type_info
            .as_ref()
            .map(|t| format!(": {}", t))
            .unwrap_or_default();

        let label = format!("{}({}){}", ctx.function_name, params_str, return_str);

        let signature = SignatureInformation {
            label,
            documentation: None,
            parameters: if parameters.is_empty() {
                None
            } else {
                Some(parameters)
            },
            active_parameter: Some(ctx.active_parameter as u32),
        };

        return Some(SignatureHelp {
            signatures: vec![signature],
            active_signature: Some(0),
            active_parameter: Some(ctx.active_parameter as u32),
        });
    }

    builtin_method_signature_help(state, &doc, &source, byte_offset, ctx.active_parameter)
}

fn builtin_method_signature_help(
    state: &WorldState,
    doc: &al_syntax::document::DocumentState,
    source: &str,
    byte_offset: usize,
    active_parameter: usize,
) -> Option<SignatureHelp> {
    let mut node = node_at_offset(&doc.tree, byte_offset)?;
    loop {
        let kind = node.kind();
        if kind == "method_call" || kind == "function_call" {
            break;
        }
        node = node.parent()?;
    }

    if node.kind() != "method_call" {
        return None;
    }

    let method_node = node.child_by_field_name("method")?;
    let object_node = node.child_by_field_name("object")?;
    let method_name = extract_name(method_node, source);
    let (object_kind, _object_name) =
        resolve_object_type_from_expression(state, doc, source, object_node, node.start_byte(), 0)?;
    let method = find_builtin_method(&object_kind, &method_name)?;

    let parameters: Vec<ParameterInformation> = method
        .params
        .iter()
        .map(|param| ParameterInformation {
            label: ParameterLabel::Simple(param.label.to_string()),
            documentation: Some(lsp_types::Documentation::String(
                param.documentation.to_string(),
            )),
        })
        .collect();
    let active_param = active_parameter.min(parameters.len().saturating_sub(1)) as u32;

    let signature = SignatureInformation {
        label: method.signature.to_string(),
        documentation: Some(lsp_types::Documentation::String(format!(
            "{}\n{}",
            method.summary, method.docs_url
        ))),
        parameters: if parameters.is_empty() {
            None
        } else {
            Some(parameters)
        },
        active_parameter: Some(active_param),
    };

    Some(SignatureHelp {
        signatures: vec![signature],
        active_signature: Some(0),
        active_parameter: Some(active_param),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WorldState;
    use al_syntax::document::DocumentState;
    use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Url};

    fn make_signature_help_params(uri: Url, line: u32, character: u32) -> SignatureHelpParams {
        SignatureHelpParams {
            context: None,
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
        }
    }

    fn cursor_after(source: &str, marker: &str) -> (u32, u32) {
        let idx = source
            .find(marker)
            .unwrap_or_else(|| panic!("marker not found: {marker}"))
            + marker.len();
        let line = source[..idx].bytes().filter(|&b| b == b'\n').count() as u32;
        let line_start = source[..idx].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let character = (idx - line_start) as u32;
        (line, character)
    }

    #[test]
    fn test_signature_help_for_builtin_record_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Rec: Record Customer;
    begin
        Rec.SetRange("No.", '10000');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "SetRange(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert_eq!(help.active_signature, Some(0));
        assert_eq!(help.active_parameter, Some(0));
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("SetRange(Field: Any")),
            "expected SetRange signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_text_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Txt: Text;
    begin
        if Txt.Contains('A') then;
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "Contains(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("Contains(Value: Text): Boolean")),
            "expected Contains signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_instream_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        StreamIn: InStream;
        ValueText: Text;
    begin
        StreamIn.ReadText(ValueText, 50);
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "ReadText(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("ReadText(var Value: Text")),
            "expected ReadText signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_sessionsettings_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        SessionCfg: SessionSettings;
    begin
        SessionCfg.RequestSessionUpdate(true);
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "RequestSessionUpdate(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures.first().is_some_and(|s| s
                .label
                .contains("RequestSessionUpdate([DoUpdate: Boolean])")),
            "expected RequestSessionUpdate signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_notification_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Notice: Notification;
    begin
        Notice.AddAction('Open', 50100, 'RunAction');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "AddAction(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("AddAction(Caption: Text")),
            "expected AddAction signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_taskscheduler_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        TaskScheduler.CreateTask(50100);
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "CreateTask(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("CreateTask(CodeunitId: Integer")),
            "expected CreateTask signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_errorinfo_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Err: ErrorInfo;
    begin
        Err.AddNavigationAction('Open', 50100, 'RunAction');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "AddNavigationAction(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("AddNavigationAction(Caption: Text")),
            "expected AddNavigationAction signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_testfield_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        FieldVar: TestField;
    begin
        FieldVar.SetValue('X');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "SetValue(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures
                .first()
                .is_some_and(|s| s.label.contains("SetValue(Value: Any)")),
            "expected SetValue signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_testrequestpage_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        RequestVar: TestRequestPage "Dummy Sales Report";
    begin
        RequestVar.SaveAsXml('dataset.xml', 'labels.xml');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "SaveAsXml(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures.first().is_some_and(|s| s
                .label
                .contains("SaveAsXml(DataSetFileName: Text, LabelsFileName: Text)")),
            "expected SaveAsXml signature, got: {help:?}"
        );
    }

    #[test]
    fn test_signature_help_for_builtin_webserviceactioncontext_method() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Ctx: WebServiceActionContext;
    begin
        Ctx.AddEntityKey('No.', '10000');
    end;
}"#;
        let uri = Url::parse("file:///test/all.al").unwrap();
        let state = WorldState::new();
        state
            .documents
            .insert(uri.clone(), DocumentState::new(source).unwrap());

        let (line, character) = cursor_after(source, "AddEntityKey(");
        let params = make_signature_help_params(uri, line, character);
        let result = handle_signature_help(&state, params);
        assert!(result.is_some(), "expected signature help");
        let help = result.unwrap();
        assert!(
            help.signatures.first().is_some_and(|s| s
                .label
                .contains("AddEntityKey(KeyName: Text, KeyValue: Any)")),
            "expected AddEntityKey signature, got: {help:?}"
        );
    }
}
