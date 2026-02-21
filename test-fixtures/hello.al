codeunit 50100 "Hello World"
{
    procedure HelloWorld()
    var
        Greeting: Text;
    begin
        Greeting := 'Hello, World!';
        Message(Greeting);
    end;

    trigger OnRun()
    begin
        HelloWorld();
    end;
}
