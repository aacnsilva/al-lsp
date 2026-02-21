table 50100 "Customer"
{
    DataClassification = CustomerContent;

    fields
    {
        field(1; "No."; Code[20])
        {
            Caption = 'Customer No.';
        }
        field(2; Name; Text[100])
        {
            Caption = 'Name';
        }
        field(3; "Phone No."; Text[30])
        {
            Caption = 'Phone No.';
        }
        field(4; "Balance (LCY)"; Decimal)
        {
            Caption = 'Balance (LCY)';
        }
    }

    keys
    {
        key(PK; "No.")
        {
            Clustered = true;
        }
        key(Name; Name)
        {
        }
    }

    trigger OnInsert()
    begin
        // Auto-generate number
    end;
}
