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

    // procedure HelloWorld()
    // var
    //     IAddressProvider: Interface IAddressProvider;
    // begin
    //     IAddressProvider.GetAddress();
    // end;
}
