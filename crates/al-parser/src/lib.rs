use tree_sitter::Language;

unsafe extern "C" {
    fn tree_sitter_al() -> *const std::ffi::c_void;
}

/// Returns the tree-sitter [`Language`] for AL.
pub fn language() -> Language {
    unsafe { Language::from_raw(tree_sitter_al() as *const tree_sitter::ffi::TSLanguage) }
}

/// Parse AL source code, returning the tree-sitter [`Tree`].
pub fn parse(source: &str) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language())
        .expect("failed to set AL language");
    parser.parse(source, None)
}

/// Parse AL source with an old tree for incremental reparsing.
pub fn parse_with(source: &str, old_tree: Option<&tree_sitter::Tree>) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language())
        .expect("failed to set AL language");
    parser.parse(source, old_tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_codeunit() {
        let source = r#"codeunit 50100 "My Codeunit"
{
    procedure Hello()
    begin
        Message('Hello, World!')
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_interface() {
        let source = r#"interface ICustomer
{
    procedure GetName(): Text;
    procedure SetName(NewName: Text);
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());

        // Verify structure: should have an interface_declaration with 2 procedure_declarations
        let interface = root.named_child(0).expect("no interface node");
        assert_eq!(interface.kind(), "interface_declaration");
    }

    #[test]
    fn test_parse_interface_without_method_semicolons() {
        let source = r#"interface "Demo IFunctions"
{
    Access = Internal;

    procedure ReadLocalVar(var LastSlipNo: Code[20])
    procedure WriteLocalVar(LastSlipNo: Code[20])
    procedure RetrieveSusp(SlipNumber: Code[20]; var ErrorText: Text): Boolean
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(
            !root.has_error(),
            "tree has errors for interface methods without semicolons: {}",
            root.to_sexp()
        );
    }

    #[test]
    fn test_parse_interface_named_return_without_semicolon() {
        let source = r#"interface "Demo IFunctions"
{
    procedure InsertTmpTrans(var LastSlipNo: Code[20]; ShiftNo: Code[1]; SetSalesType: Code[20]; TableNo: Integer; TrainingActive: Boolean; TableDescr: Text) NewSlipNo: Code[20]
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(
            !root.has_error(),
            "tree has errors for interface named return without semicolon: {}",
            root.to_sexp()
        );
    }

    #[test]
    fn test_parse_address_provider_fixture() {
        let source = r#"interface IAddressProvider
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
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        eprintln!("S-expr: {}", root.to_sexp());
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_variable_of_interface_type() {
        let source = r#"codeunit 50200 CompanyAddressProvider implements IAddressProvider
{
    procedure DoWork()
    var
        AddressProvider: Interface IAddressProvider;
    begin
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        eprintln!("S-expr: {}", root.to_sexp());
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_interface_with_property() {
        let source = r#"interface ICustomer
{
    Access = Internal;

    procedure GetOrders(): Integer;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_table() {
        let source = r#"table 50100 "My Table"
{
    fields
    {
        field(1; "No."; Code[20])
        {
        }
        field(2; Name; Text[100])
        {
        }
    }

    keys
    {
        key(PK; "No.")
        {
            Clustered = true;
        }
    }
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_compound_assignment() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X += 1;
        X -= 2;
        X *= 3;
        X /= 4;
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());

        // Verify compound_assignment_statement nodes exist
        let sexp = root.to_sexp();
        assert!(
            sexp.contains("compound_assignment_statement"),
            "expected compound_assignment_statement in tree: {sexp}"
        );
    }

    #[test]
    fn test_parse_codeunit_type_variable() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        ServiceLocator: Codeunit "Demo Service Locator";
        MyPage: Page "Customer Card";
        MyReport: Report "Sales Invoice";
    begin
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_dictionary_and_list_primitives() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        dict: Dictionary of [Text, Text];
        userGeneratedTags: List of [Text];
    begin
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_quoted_enum_value() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := "My Enum"::MyValue;
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
        let sexp = root.to_sexp();
        assert!(
            sexp.contains("qualified_enum_value"),
            "expected qualified_enum_value in tree: {sexp}"
        );
    }

    #[test]
    fn test_parse_incomplete_qualified_enum_value_is_permitted() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := MyEnum::;
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(
            !root.has_error(),
            "tree should allow incomplete qualified enum value for completion scenarios: {}",
            root.to_sexp()
        );
    }

    #[test]
    fn test_parse_incomplete_qualified_enum_value_on_member_access_is_permitted() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(
            !root.has_error(),
            "tree should allow incomplete member-access enum qualification for completion scenarios: {}",
            root.to_sexp()
        );
    }

    #[test]
    fn test_parse_multi_name_variable() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        LineRec, NewLine: Record "Demo Trans. Line";
    begin
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_procedure_and_trigger_attributes() {
        let source = r#"codeunit 50100 Test
{
    [Test]
    [HandlerFunctions('MyHandler')]
    procedure DoWork()
    begin
    end;

    [Test]
    trigger OnRun()
    begin
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_label_with_locked_option() {
        let source = r#"codeunit 50100 Test
{
    var
        EBTText: Label 'EBT', Locked = true;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_region_directives() {
        let source = r#"codeunit 50100 Test
{
    #region Transaction GUI
    procedure DoWork()
    begin
    end;
    #endregion
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_if_then_empty_statement() {
        let source = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        if true then;
    end;
}"#;
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_temporal_literals() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_in_operator_with_interval_range() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
        let sexp = root.to_sexp();
        assert!(
            sexp.contains("range_expression"),
            "expected range_expression in tree: {sexp}"
        );
    }

    #[test]
    fn test_parse_codeunit_run_invocation() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_page_usercontrol_scanner_dialog() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_report_with_nested_dataitems_requestpage_and_labels() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
    }

    #[test]
    fn test_parse_tablerelation_if_else_expression() {
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
        let tree = parse(source).expect("parse failed");
        let root = tree.root_node();
        assert!(!root.has_error(), "tree has errors: {}", root.to_sexp());
        let sexp = root.to_sexp();
        assert!(
            sexp.contains("table_relation_if_expression"),
            "expected table_relation_if_expression in tree: {sexp}"
        );
    }
}
