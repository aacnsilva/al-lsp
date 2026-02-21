use lsp_types::{
    ParameterInformation, ParameterLabel, SignatureHelp, SignatureHelpParams,
    SignatureInformation,
};

use al_syntax::ast::AlSymbolKind;
use al_syntax::navigation::find_call_context;

use crate::convert::lsp_position_to_byte_offset;
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
    let sym = ctx.symbol?;

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

    Some(SignatureHelp {
        signatures: vec![signature],
        active_signature: Some(0),
        active_parameter: Some(ctx.active_parameter as u32),
    })
}
