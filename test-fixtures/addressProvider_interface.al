interface IAddressProvider
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
}
