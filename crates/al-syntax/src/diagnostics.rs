use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use tree_sitter::Tree;

/// Walk the tree and extract diagnostics from ERROR and MISSING nodes.
pub fn extract_diagnostics(tree: &Tree, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    walk_for_errors(tree.root_node(), source, &mut diagnostics);
    diagnostics
}

fn walk_for_errors(node: tree_sitter::Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
    if node.is_error() {
        let start = node.start_position();
        let end = node.end_position();
        let text = node
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .chars()
            .take(50)
            .collect::<String>();

        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: start.row as u32,
                    character: start.column as u32,
                },
                end: Position {
                    line: end.row as u32,
                    character: end.column as u32,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("al-lsp".to_string()),
            message: format!("Syntax error: unexpected `{text}`"),
            ..Default::default()
        });
    } else if node.is_missing() {
        let start = node.start_position();
        let kind = node.kind();

        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: start.row as u32,
                    character: start.column as u32,
                },
                end: Position {
                    line: start.row as u32,
                    character: start.column as u32 + 1,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("al-lsp".to_string()),
            message: format!("Expected `{kind}`"),
            ..Default::default()
        });
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_for_errors(child, source, diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_errors_for_valid_code() {
        let source = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(diags.is_empty(), "expected no errors, got: {:?}", diags);
    }

    #[test]
    fn test_no_errors_for_interface() {
        let source = r#"interface ICustomer
{
    Access = Internal;

    procedure GetName(): Text;
    procedure SetName(NewName: Text);
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for interface, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_incomplete_qualified_enum_value() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := MyEnum::;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for incomplete enum qualification, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_incomplete_member_access_qualified_enum_value() {
        let source = r#"table 50100 Customer
{
    fields
    {
        field(1; Status; Enum MyEnum)
        {
        }
    }
}

codeunit 50100 Test
{
    procedure DoWork()
    var
        Rec: Record Customer;
    begin
        Rec.Status::;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for incomplete member-access enum qualification, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_label_region_and_empty_if_then() {
        let source = r#"codeunit 50100 Test
{
    #region Transaction GUI
    var
        EBTText: Label 'EBT', Locked = true;
    #endregion

    procedure DoWork()
    begin
        if true then;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for valid label/region/empty-if syntax, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_obsolete_attribute_on_global_variables() {
        let source = r#"codeunit 50100 Test
{
    var
        [Obsolete('This variable is not used in Compressed lines mode.')]
        SelectionBuffer: Record "LSC Posted Customer Order Line" temporary;
        [Obsolete('This variable is not used in Compressed lines mode.')]
        GuiLineBuffer: Record "LSC Posted Customer Order Line" temporary;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for obsolete attribute on globals, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_pragma_warning_directives() {
        let source = r#"table 50100 TestTable
{
    fields
    {
        #pragma warning disable AL0432
        field(9; Status; Enum "LSC CO Line Status")
        {
        }
        #pragma warning restore AL0432
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for pragma directives, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_enum_value_quoted_space() {
        let source = r#"enum 50100 "My Enum"
{
    value(0; " ") { Caption = ' ', Locked = True; }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for quoted-space enum value, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_option_field_type() {
        let source = r#"table 50100 TestTable
{
    fields
    {
        field(22; "Function Type"; Option)
        {
        }
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for Option field type, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_enum_prefix_qualified_value() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := Enum::"LSC POS Trans. Numpad Trigger"::TenderKeyPressedEx;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for Enum::<name>::<value>, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_tablerelation_and_calcformula_properties() {
        let source = r#"table 50100 TestTable
{
    fields
    {
        field(28; "Field Link"; Integer)
        {
            BlankZero = true;
            Caption = 'Field Link';
            TableRelation = Field."No." WHERE(TableNo = FIELD("Table Link"),
                                               Enabled = CONST(true),
                                               Class = CONST(Normal),
                                               Type = FILTER(<> BLOB));
            DataClassification = CustomerContent;
            ToolTip = 'Specifies the number of the field that this parameter is linked to. If a parameter is linked to a table and field you can look up the value from that table/field when setting up the parameter values.';
        }
        field(30; "Codeunit Name"; Text[30])
        {
            CalcFormula = Lookup(AllObj."Object Name" WHERE("Object Type" = CONST(Codeunit),
                                                             "Object ID" = FIELD("Run Codeunit")));
            Caption = 'Codeunit Name';
            Editable = false;
            FieldClass = FlowField;
            ToolTip = 'Specifies the description of the codeunit that executes the POS command.';
        }
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for TableRelation/CalcFormula properties, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_escaped_single_quote_in_label() {
        let source = r#"codeunit 50100 Test
{
    var
        LoginActionsInvalidInTransErr: Label 'Login actions can''t be done in transactions';
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for doubled-quote escape in string literal, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_procedure_semicolon_before_begin() {
        let source = r#"codeunit 50100 Test
{
    local procedure PosFunc(): Interface "LSC IFunctions";
    begin
        exit(ServiceLocator.POSFunctions());
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for semicolon before procedure body, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_in_operator_with_list_literal() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        FunctionSetup2: Record "Function Setup";
    begin
        if FunctionSetup2."Run Codeunit" in [Codeunit::"LSC Pop-up POS Commands", Codeunit::"LSC Popup Controller"] then begin
        end;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for `in` with list literal, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_named_return_value() {
        let source = r#"codeunit 50100 Test
{
    procedure GetFunctionModeEnum() FunctionModeEnum: Enum "LSC POS Command";
    begin
        exit(FunctionSetup.CommandToEnum(GetFunctionMode));
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for named return value syntax, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_ternary_with_enum_qualified_values() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        UsePaymentToken: Boolean;
        PaymentAbsValue: Decimal;
    begin
        PosTransactionGui.OpenNumericKeyboard(AmountMsg, Formatting.FormatAmountToShow(PaymentAbsValue), UsePaymentToken ? Enum::"LSC POS Trans. Numpad Trigger"::CardOnFilePressed : Enum::"LSC POS Trans. Numpad Trigger"::TenderKeyPressedEx);
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for ternary enum-qualified expression, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_errors_for_invalid_code() {
        let source = r#"codeunit 50100 Test
{
    procedure
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(!diags.is_empty(), "expected errors for invalid code");
    }
}
