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
    fn test_no_errors_for_interface_methods_without_semicolons() {
        let source = r#"interface "Demo IFunctions"
{
    Access = Internal;

    procedure ReadLocalVar(var LastSlipNo: Code[20])
    procedure WriteLocalVar(LastSlipNo: Code[20])
    procedure RetrieveSusp(SlipNumber: Code[20]; var ErrorText: Text): Boolean
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for interface methods without semicolons, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_interface_named_return_without_semicolon() {
        let source = r#"interface "Demo IFunctions"
{
    procedure InsertTmpTrans(var LastSlipNo: Code[20]; ShiftNo: Code[1]; SetSalesType: Code[20]; TableNo: Integer; TrainingActive: Boolean; TableDescr: Text) NewSlipNo: Code[20]
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for interface named return without semicolon, got: {:?}",
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
        SelectionBuffer: Record "Posted Customer Order Line" temporary;
        [Obsolete('This variable is not used in Compressed lines mode.')]
        GuiLineBuffer: Record "Posted Customer Order Line" temporary;
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
        field(9; Status; Enum "CO Line Status")
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
    fn test_no_errors_for_dictionary_and_list_primitives() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        dict: Dictionary of [Text, Text];
        userGeneratedTags: List of [Text];
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for Dictionary/List primitives, got: {:?}",
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
        X := Enum::"Trans. Numpad Trigger"::TenderKeyPressedEx;
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
    fn test_no_errors_for_event_publisher_and_subscriber_attributes() {
        let source = r#"codeunit 50100 "My Publisher"
{
    [IntegrationEvent(false, false)]
    procedure OnAfterPost()
    begin
    end;
}

codeunit 50101 "My Subscriber"
{
    [EventSubscriber(ObjectType::Codeunit, Codeunit::"My Publisher", 'OnAfterPost', '', false, false)]
    local procedure HandleOnAfterPost()
    begin
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for EventSubscriber/IntegrationEvent attributes, got: {:?}",
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
            ToolTip = 'Specifies the description of the codeunit that executes the command.';
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
    fn test_no_errors_for_tablerelation_if_else_expression() {
        let source = r#"table 50100 "Dummy Config"
{
    fields
    {
        field(1; "Restaurant No."; Code[20])
        {
        }
        field(2; "Target Restaurant"; Code[20])
        {
        }
        field(3; "Dining Area Id"; Integer)
        {
            TableRelation = IF ("Target Restaurant" = FILTER('')) "Dummy Dining Area".ID WHERE("Restaurant No." = FIELD("Restaurant No."))
                            ELSE
                            IF ("Target Restaurant" = FILTER(<> '')) "Dummy Dining Area".ID WHERE("Restaurant No." = FIELD("Target Restaurant"));
        }
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for TableRelation IF/ELSE expression, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_property_name_keyword_prefix_and_tablerelation_where_consts() {
        let source = r#"table 50100 "Dummy Config"
{
    fields
    {
        field(21; "After Action"; Option)
        {
            OptionCaption = 'Return,Stay';
            OptionMembers = "Return","Stay";
        }
        field(22; "Income Account 1"; Code[20])
        {
            Caption = 'Income Account 1';
            TableRelation = "Dummy Income/Expense Account"."No." WHERE("Store No." = FIELD("Restaurant No."),
                                                                       "Account Type" = CONST(Income),
                                                                       "Gratuity Type" = CONST(Tips));
        }
        field(23; "Income Account 2"; Code[20])
        {
            Caption = 'Income Account 2';
            TableRelation = "Dummy Income/Expense Account"."No." WHERE("Store No." = FIELD("Restaurant No."),
                                                                       "Account Type" = CONST(Income),
                                                                       "Gratuity Type" = CONST(Tips));
        }
        field(24; "Restaurant No."; Code[20])
        {
        }
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for keyword-prefixed property names and WHERE CONST filters, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_enum_with_implements_clause() {
        let source = r#"enum 50100 "Dummy Action" implements "Dummy Contract"
{
    value(0; None)
    {
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for enum with implements clause, got: {:?}",
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
    local procedure HelperFunc(): Interface "Demo IFunctions";
    begin
        exit(ServiceLocator.CoreFunctions());
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
        if FunctionSetup2."Run Codeunit" in [Codeunit::"Pop-up Commands", Codeunit::"Popup Controller"] then begin
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
    fn test_no_errors_for_in_operator_with_interval_range() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        OfferValidFromDate: Date;
        OfferValidToDate: Date;
    begin
        if (Today in [OfferValidFromDate .. OfferValidToDate]) then begin
        end;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for `in` with interval range, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_codeunit_run_invocation() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        FunctionSetup2: Record "Function Setup";
        MenuLine2_l: Record "Menu Line";
    begin
        CODEUNIT.Run(FunctionSetup2."Run Codeunit", MenuLine2_l);
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for CODEUNIT.Run invocation, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_named_return_value() {
        let source = r#"codeunit 50100 Test
{
    procedure GetFunctionModeEnum() FunctionModeEnum: Enum "Command Enum";
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
        UiDialog.OpenNumericKeyboard(AmountMsg, Formatting.FormatAmountToShow(PaymentAbsValue), UsePaymentToken ? Enum::"Trans. Numpad Trigger"::CardOnFilePressed : Enum::"Trans. Numpad Trigger"::TenderKeyPressedEx);
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
    fn test_no_errors_for_temporal_literals() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        LineRec: Record "Sales Line";
        D: Date;
        T: Time;
        DT: DateTime;
    begin
        if (LineRec."Customer Order Line") and (LineRec."Trans. Time" = 0T) then begin
        end;
        D := 0D;
        T := 0T;
        DT := 0DT;
    end;
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for temporal literals, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_page_usercontrol_scanner_dialog() {
        let source = r#"page 99008876 "Scanner Dialog"
{
    ApplicationArea = All;
    Caption = 'Barcode scanning';
    Editable = false;
    PageType = Document;
    SourceTable = "Scanner";

    layout
    {
        area(content)
        {
            usercontrol(control; "DialogHost")
            {
                trigger AddInReady(data: Text)
                begin
                    CurrPage.control.SendRequestToAddInEx('SCANNER:SCANDATA', '', '');
                end;
            }
        }
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for page usercontrol scanner dialog, got: {:?}",
            diags
        );
    }

    #[test]
    fn test_no_errors_for_report_with_nested_dataitems_requestpage_and_labels() {
        let source = r#"report 10000712 "Stores In Location Profile"
{
    DefaultLayout = RDLC;
    RDLCLayout = 'src/MicrosoftExtension/Layouts/Stores In Location Profile.rdlc';

    dataset
    {
        dataitem(Store; "Store")
        {
            DataItemTableView = SORTING("No.") ORDER(Ascending);
            PrintOnlyIfDetail = true;
            column(Name; Store.Name)
            {
            }
            dataitem("Inventory Lookup Table"; "Inventory Lookup Table")
            {
                DataItemLink = "Store No." = FIELD("No.");
                column(ItemNo_Test; "Inventory Lookup Table"."Item No.")
                {
                }
            }
        }
    }

    requestpage
    {
        layout
        {
        }
    }

    labels
    {
    }
}"#;
        let tree = al_parser::parse(source).unwrap();
        let diags = extract_diagnostics(&tree, source);
        assert!(
            diags.is_empty(),
            "expected no errors for report with nested dataitems/requestpage/labels, got: {:?}",
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
