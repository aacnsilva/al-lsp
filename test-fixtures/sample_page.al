page 50100 "Customer Card"
{
    PageType = Card;
    SourceTable = "Customer";
    Caption = 'Customer Card';

    layout
    {
        area(Content)
        {
            group(General)
            {
                field("No."; Rec."No.")
                {
                    ApplicationArea = All;
                }
                field(Name; Rec.Name)
                {
                    ApplicationArea = All;
                }
            }
        }
    }

    actions
    {
        area(Processing)
        {
            action("Print")
            {
                Caption = 'Print';
                trigger OnAction()
                var
                    AddressProvider: Interface IAddressProvider;
                begin
                    Message('Printing...');
                    AddressProvider.GetAddress();
                end;
            }
        }
    }
}
