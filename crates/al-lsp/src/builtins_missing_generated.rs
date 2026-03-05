// Auto-generated from Microsoft Learn AL methods library index tables + method pages.
// Do not edit manually.

const MISSING_BIGTEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Read",
        signature: "Read(InStream: InStream): Boolean",
        summary: "Streams a BigText object that is stored as a BLOB in a table to a BigText variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/bigtext/bigtext-read-method",
        params: &[
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "The InStream object type that you use to stream a BLOB to a BigText variable.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Write",
        signature: "Write(OutStream: OutStream): Boolean",
        summary: "Streams a BigText object to a BLOB field in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/bigtext/bigtext-write-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The stream to which you write a BigText.",
            },
        ],
    },
];

const MISSING_BLOB_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Export",
        signature: "Export(Name: Text): Text",
        summary: "Exports a binary large object (BLOB) to a file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/blob/blob-export-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The path and name of the BLOB that you want to export. When you enter the path, consider these shortcuts: -   You can omit the drive letter if the command is located on the current drive. -   You can omit the full path if the command is located in the current directory. -   You can enter only the subdirectory name if the command is located in a subdirectory of the current directory.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Import",
        signature: "Import(Name: Text): Text",
        summary: "Imports a binary large object (BLOB) from a file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/blob/blob-import-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The path and name of the BLOB that you want to import. When you enter the path, consider the following shortcuts: -   You can omit the drive letter if the command is located on the current drive. -   You can omit the full path if the command is located in the current directory. -   You can enter only the subdirectory name if the command is located in a subdirectory of the current directory.",
            },
        ],
    },
];

const MISSING_COMPANYPROPERTY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ID",
        signature: "ID(): Guid",
        summary: "Gets the current company ID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/companyproperty/companyproperty-id-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "UrlName",
        signature: "UrlName(): Text",
        summary: "Gets the string that represents the company name in a URL.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/companyproperty/companyproperty-urlname-method",
        params: PARAM_NONE,
    },
];

const MISSING_DATABASE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AlterKey",
        signature: "AlterKey(KeyRef: KeyRef, Enable: Boolean)",
        summary: "Alter a table's key in SQL, either disabling or enabling it.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-alterkey-method",
        params: &[
            BuiltinMethodParam {
                label: "KeyRef: KeyRef",
                documentation: "A keyref of the key to alter.",
            },
            BuiltinMethodParam {
                label: "Enable: Boolean",
                documentation: "Whether to enable or disable a key. Keys that are created as disabled cannot be enabled.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ChangeUserPassword",
        signature: "ChangeUserPassword(OldPassword: Text, NewPassword: Text): Boolean",
        summary: "Changes the password for the current user.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-changeuserpassword-method",
        params: &[
            BuiltinMethodParam {
                label: "OldPassword: Text",
                documentation: "The old password for the user.",
            },
            BuiltinMethodParam {
                label: "NewPassword: Text",
                documentation: "The new password for the user.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CheckLicenseFile",
        signature: "CheckLicenseFile(KeyNumber: Integer)",
        summary: "Checks a key in the license file of the system.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-checklicensefile-method",
        params: &[
            BuiltinMethodParam {
                label: "KeyNumber: Integer",
                documentation: "The number of the key you want to check.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CopyCompany",
        signature: "CopyCompany(SourceName: Text, DestinationName: Text): Boolean",
        summary: "Creates a new company and copies all data from an existing company in the same database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-copycompany-method",
        params: &[
            BuiltinMethodParam {
                label: "SourceName: Text",
                documentation: "The name of the company that you want to copy data from.",
            },
            BuiltinMethodParam {
                label: "DestinationName: Text",
                documentation: "The name of the company that you want to create and copy data to. The company name can have a maximum of 30 characters. If the database collation is case-sensitive, you can have one company called COMPANY and another called Company. However, if the database is case-insensitive, you cannot create companies with names that differ only by case.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DataFileInformation",
        signature: "DataFileInformation(ShowDialog: Boolean, var FileName: Text, var Description: Text, var HasApplication: Boolean, var HasApplicationData: Boolean, var HasGlobalData: Boolean, var tenantId: Text, var exportDate: DateTime, var CompanyRecord: Record): Boolean",
        summary: "Specifies data from a file that has been exported from a database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-datafileinformation-method",
        params: &[
            BuiltinMethodParam {
                label: "ShowDialog: Boolean",
                documentation: "Specifies if you want to display a dialog box where the user can confirm the action.",
            },
            BuiltinMethodParam {
                label: "FileName: Text",
                documentation: "Specifies the name and location of the file that you want to read information from. The file must have been exported from a database.",
            },
            BuiltinMethodParam {
                label: "Description: Text",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
            BuiltinMethodParam {
                label: "HasApplication: Boolean",
                documentation: "Specifies if the file contains application objects. Create a variable of type Boolean to specify this parameter.",
            },
            BuiltinMethodParam {
                label: "HasApplicationData: Boolean",
                documentation: "Specifies if the file contains the data that defines the application in the database. This includes the permissions, permission sets, profiles, and style sheets. Create a variable of type Boolean to specify this parameter.",
            },
            BuiltinMethodParam {
                label: "HasGlobalData: Boolean",
                documentation: "Specifies if the file contains global, non-company specific data. Create a variable of type Boolean to specify this parameter.",
            },
            BuiltinMethodParam {
                label: "tenantId: Text",
                documentation: "Specifies the tenant ID of the database that the data was exported from. Create a variable of type Text to specify this parameter.",
            },
            BuiltinMethodParam {
                label: "exportDate: DateTime",
                documentation: "Specifies the date and time when the data was exported. Create a variable of type DateTime to specify this parameter.",
            },
            BuiltinMethodParam {
                label: "CompanyRecord: Record",
                documentation: "Specifies the company or companies in the file.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDefaultTableConnection",
        signature: "GetDefaultTableConnection(Type: TableConnectionType): Text",
        summary: "Gets the default table connection based on the specified connection type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-getdefaulttableconnection-method",
        params: &[
            BuiltinMethodParam {
                label: "Type: TableConnectionType",
                documentation: "The type of table connection as defined in the TableType property.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "HasTableConnection",
        signature: "HasTableConnection(Type: TableConnectionType, Name: Text): Boolean",
        summary: "Verifies if a connection to an external database exists based on the specified name.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-hastableconnection-method",
        params: &[
            BuiltinMethodParam {
                label: "Type: TableConnectionType",
                documentation: "Specifies the type of table connection as defined in the TableType property.",
            },
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the external table connection. You must already have registered a table connection with this name.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "LastUsedRowVersion",
        signature: "LastUsedRowVersion(): BigInteger",
        summary: "Gets the last used RowVersion from the database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-lastusedrowversion-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "MinimumActiveRowVersion",
        signature: "MinimumActiveRowVersion(): BigInteger",
        summary: "Returns the lowest active RowVersion in the database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-minimumactiverowversion-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RegisterTableConnection",
        signature: "RegisterTableConnection(Type: TableConnectionType, Name: Text, Connection: Text)",
        summary: "Registers a table connection to an external database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-registertableconnection-method",
        params: &[
            BuiltinMethodParam {
                label: "Type: TableConnectionType",
                documentation: "Specifies the type of table connection as defined in the TableType property.",
            },
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Specifies the name of the connection in your code, or the name of the primary key field on the table.",
            },
            BuiltinMethodParam {
                label: "Connection: Text",
                documentation: "Specifies the connection to the external database.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SerialNumber",
        signature: "SerialNumber(): Text",
        summary: "Gets a string that contains the serial number of the license file for your system.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-serialnumber-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ServiceInstanceId",
        signature: "ServiceInstanceId(): Integer",
        summary: "Gets the ID of the service instance.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-serviceinstanceid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetUserPassword",
        signature: "SetUserPassword(USID: Guid, Password: Text): Boolean",
        summary: "Sets a password for the user iwith the given user security ID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-setuserpassword-method",
        params: &[
            BuiltinMethodParam {
                label: "USID: Guid",
                documentation: "User security ID of the user for which to set the password.",
            },
            BuiltinMethodParam {
                label: "Password: Text",
                documentation: "The password to set for the user.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "UnregisterTableConnection",
        signature: "UnregisterTableConnection(Type: TableConnectionType, Name: Text)",
        summary: "Unregisters a table connection to an external database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-unregistertableconnection-method",
        params: &[
            BuiltinMethodParam {
                label: "Type: TableConnectionType",
                documentation: "Specifies the type of table connection as defined in the TableType property. If the table is of type ExternalSQL, UNREGISTERTABLECONNECTION rolls back the current transaction.",
            },
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Specifies the name of the connection in your code, or the name of the primary key field on the table.",
            },
        ],
    },
];

const MISSING_DATATRANSFER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CopyRows",
        signature: "CopyRows()",
        summary: "Copies the rows from the source table to the destination table with the fields selected with AddFields and the filters applied with AddSourceFilter, in one bulk operation in SQL.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/datatransfer/datatransfer-copyrows-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetTables",
        signature: "SetTables(SourceTable: Integer, DestinationTable: Integer)",
        summary: "Sets the source and destination tables for the data transfer.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/datatransfer/datatransfer-settables-method",
        params: &[
            BuiltinMethodParam {
                label: "SourceTable: Integer",
                documentation: "The source table for the transfer.",
            },
            BuiltinMethodParam {
                label: "DestinationTable: Integer",
                documentation: "The destination table for the transfer.",
            },
        ],
    },
];

const MISSING_DEBUGGER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Activate",
        signature: "Activate(): Boolean",
        summary: "Activates the debugger and attaches the debugger to the next session that is started.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-activate-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Attach",
        signature: "Attach(SessionID: Integer): Boolean",
        summary: "Activates the debugger and attaches it to the specified session.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-attach-method",
        params: &[
            BuiltinMethodParam {
                label: "SessionID: Integer",
                documentation: "The ID of the session that you want to attach the debugger to.The session can be any of the following: -   Windows client -   Web client -   NAS services session -   SOAP web services client session -   OData web services client session -   Background session",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "BreakOnRecordChanges",
        signature: "BreakOnRecordChanges(Ok: Boolean): Boolean",
        summary: "Breaks execution before a change to a record occurs.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-breakonrecordchanges-method",
        params: &[
            BuiltinMethodParam {
                label: "Ok: Boolean",
                documentation: "Specifies whether the debugger should break execution when a change to a record occurs. If Ok is true, then the debugger breaks before creating, updating, or deleting a record.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Continue",
        signature: "Continue(): Boolean",
        summary: "Executes code until the next breakpoint or until execution ends.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-continue-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Deactivate",
        signature: "Deactivate(): Boolean",
        summary: "Deactivates the debugger.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-deactivate-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DebuggedSessionID",
        signature: "DebuggedSessionID(): Integer",
        summary: "Gets the ID of the previous session that the debugger was attached to.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-debuggedsessionid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DebuggingSessionID",
        signature: "DebuggingSessionID(): Integer",
        summary: "Gets the ID of the session that the debugger is currently attached to.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-debuggingsessionid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetLastErrorText",
        signature: "GetLastErrorText(): Text",
        summary: "Gets the last error that occurred in the debugger.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-getlasterrortext-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsActive",
        signature: "IsActive(): Boolean",
        summary: "Indicates whether the debugger is active.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-isactive-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsBreakpointHit",
        signature: "IsBreakpointHit(): Boolean",
        summary: "Specifies if a breakpoint is hit in a debugging session.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-isbreakpointhit-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SkipSystemTriggers",
        signature: "SkipSystemTriggers(Ok: Boolean): Boolean",
        summary: "Enables the debugger to skip code that is inside system triggers.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-skipsystemtriggers-method",
        params: &[
            BuiltinMethodParam {
                label: "Ok: Boolean",
                documentation: "Specifies if the debugger should skip system triggers.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "StepInto",
        signature: "StepInto(): Boolean",
        summary: "Executes a method call and then stops at the first line of code inside the method.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-stepinto-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "StepOut",
        signature: "StepOut(): Boolean",
        summary: "Enables debugging to return to the calling method after it steps into a method call.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-stepout-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "StepOver",
        signature: "StepOver(): Boolean",
        summary: "Executes a method call and then stops at the first line outside the method call.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-stepover-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Stop",
        signature: "Stop(): Boolean",
        summary: "Stops execution as if the code hits an error.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-stop-method",
        params: PARAM_NONE,
    },
];

const MISSING_DIALOG_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Error",
        signature: "Error(Message: ErrorInfo)",
        summary: "Displays an error message and ends the execution of AL code.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/dialog/dialog-error-errorinfo-method",
        params: &[
            BuiltinMethodParam {
                label: "Message: ErrorInfo",
                documentation: "The ErrorInfo structure that contains error message, error type, verbosity, and data classification.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "LogInternalError",
        signature: "LogInternalError(Message: Text, DataClassificationInstance: DataClassification, VerbosityInstance: Verbosity)",
        summary: "Log internal errors for telemetry.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/dialog/dialog-loginternalerror-string-dataclassification-verbosity-method",
        params: &[
            BuiltinMethodParam {
                label: "Message: Text",
                documentation: "This string contains the text of the error message you want to log into telemetry. It is not what the user will get, they will only get a generic error message.",
            },
            BuiltinMethodParam {
                label: "DataClassificationInstance: DataClassification",
                documentation: "Sets the classification of the data in the error message.",
            },
            BuiltinMethodParam {
                label: "VerbosityInstance: Verbosity",
                documentation: "Represents the security level of events.",
            },
        ],
    },
];

const MISSING_ENUM_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsInteger",
        signature: "AsInteger(): Integer",
        summary: "Get the enum value as an integer value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/enum/enum-asinteger-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FromInteger",
        signature: "FromInteger(Value: Integer): Any",
        summary: "Returns an enum with the integer value",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/enum/enum-frominteger-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Integer",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
];

const MISSING_ERRORINFO_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAction",
        signature: "AddAction(Caption: Text, CodeunitID: Integer, MethodName: Text)",
        summary: "Specifies an action for the error.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/errorinfo/errorinfo-addaction-string-integer-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Caption: Text",
                documentation: "The text string that appears as the caption of the action in the error UI. The string can be a label that is enabled for multilanguage functionality.",
            },
            BuiltinMethodParam {
                label: "CodeunitID: Integer",
                documentation: "The ID of the Codeunit to run when the action is initiated from the error UI. The codeunit should contain at least one global method to be called by the error action. The global method must have an ErrorInfo data type parameter for accepting the ErrorInfo object.",
            },
            BuiltinMethodParam {
                label: "MethodName: Text",
                documentation: "The name of the method in the Codeunit, which is specified by the CodeunitID parameter, that you want to run for the action.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Callstack",
        signature: "Callstack(): Text",
        summary: "Specifies a callstack where the ErrorInfo was collected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/errorinfo/errorinfo-callstack-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(): ErrorInfo",
        summary: "Creates a new ErrorInfo object with Collectible set to true.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/errorinfo/errorinfo-create--method",
        params: PARAM_NONE,
    },
];

const MISSING_FIELDREF_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Active",
        signature: "Active(): Boolean",
        summary: "Checks whether the field that is currently selected is enabled.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-active-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CalcField",
        signature: "CalcField(): Boolean",
        summary: "Updates FlowFields in a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-calcfield-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CalcSum",
        signature: "CalcSum(): Boolean",
        summary: "Calculates the total of all values of a SumIndexField in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-calcsum-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Class",
        signature: "Class(): FieldClass",
        summary: "Gets the value of the FieldClass Property of the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-class-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "EnumValueCount",
        signature: "EnumValueCount(): Integer",
        summary: "Gets the number of Enum values (or Option members) from the Enum metadata for the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-enumvaluecount-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FieldError",
        signature: "FieldError(ErrorInfo: ErrorInfo)",
        summary: "Stops the execution of the code, causing a run-time error, and creates an error message for a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-fielderror-errorinfo-method",
        params: &[
            BuiltinMethodParam {
                label: "ErrorInfo: ErrorInfo",
                documentation: "Additional information to include in the error if the test fails.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetEnumValueCaption",
        signature: "GetEnumValueCaption(Index: Integer): Text",
        summary: "Gets an Enum value (or Option member) caption for the from the Enum metadata for the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getenumvaluecaption-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index in the list of Enum values to get the Enum value (or Option member) caption for. The index is 1-based.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetEnumValueCaptionFromOrdinalValue",
        signature: "GetEnumValueCaptionFromOrdinalValue(Ordinal: Integer): Text",
        summary: "Gets an Enum value (or Option member) caption for the from the Enum metadata for the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getenumvaluecaptionfromordinalvalue-method",
        params: &[
            BuiltinMethodParam {
                label: "Ordinal: Integer",
                documentation: "The Enum value's ordinal value to get the Enum value (or Option member) caption for.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetEnumValueName",
        signature: "GetEnumValueName(Index: Integer): Text",
        summary: "Gets an Enum value (or Option member) name from the Enum metadata for the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getenumvaluename-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index in the list of Enum values to get the Enum value (or Option member) name for. The index is 1-based.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetEnumValueNameFromOrdinalValue",
        signature: "GetEnumValueNameFromOrdinalValue(Ordinal: Integer): Text",
        summary: "Gets an Enum value (or Option member) name from the Enum metadata for the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getenumvaluenamefromordinalvalue-method",
        params: &[
            BuiltinMethodParam {
                label: "Ordinal: Integer",
                documentation: "The Enum value's ordinal value to get the Enum value (or Option member) name for.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetEnumValueOrdinal",
        signature: "GetEnumValueOrdinal(Index: Integer): Integer",
        summary: "Gets the Enum value (or Option member) ordinal value from the Enum metadata for the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getenumvalueordinal-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index in the list of Enum ordinal values to get the Enum value (or Option member) ordinal value for. The index is 1-based.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetFilter",
        signature: "GetFilter(): Text",
        summary: "Gets the filter that is currently applied to the field referred to by FieldRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetRangeMax",
        signature: "GetRangeMax(): Any",
        summary: "Gets the maximum value in a range for a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getrangemax-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetRangeMin",
        signature: "GetRangeMin(): Any",
        summary: "Gets the minimum value in a range for a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-getrangemin-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsEnum",
        signature: "IsEnum(): Boolean",
        summary: "Checks if the currently selected field is an enum.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-isenum-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsOptimizedForTextSearch",
        signature: "IsOptimizedForTextSearch(): Boolean",
        summary: "Gets if the field is optimized for textual search.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-isoptimizedfortextsearch-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): Integer",
        summary: "Gets the maximum size of the field (the size specified in the DataLength property of the field).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-length-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OptionCaption",
        signature: "OptionCaption(): Text",
        summary: "Gets the option caption of the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-optioncaption-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OptionMembers",
        signature: "OptionMembers(): Text",
        summary: "Gets the list of options that are available in the field that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-optionmembers-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OptionString",
        signature: "OptionString(): Text",
        summary: "The 'OptionString' property has been deprecated and will be removed in the future.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-optionstring-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Relation",
        signature: "Relation(): Integer",
        summary: "Finds the table relationship of a given field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-relation-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TestField",
        signature: "TestField()",
        summary: "Tests that the content of the field is not zero or blank (empty string).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-testfield--method",
        params: PARAM_NONE,
    },
];

const MISSING_FILE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Copy",
        signature: "Copy(FromName: Text, ToName: Text): Boolean",
        summary: "Copies a file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-copy-method",
        params: &[
            BuiltinMethodParam {
                label: "FromName: Text",
                documentation: "The name of the file that you want to make a copy of, including its path. When you enter the path, consider these shortcuts: -   You can omit the drive designation if the file is located on the current drive. -   You can omit the full path if the file is located in the current directory. -   You can enter only the subdirectory name if the file is located in a subdirectory of the current directory.",
            },
            BuiltinMethodParam {
                label: "ToName: Text",
                documentation: "The name that you want to assign to the copy that includes its path. When you enter the path, consider these shortcuts: -   You can omit the drive designation if the file is located on the current drive. -   You can omit the full path if the file is located in the current directory. -   You can enter only the subdirectory name if the file is located in a subdirectory of the current directory.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Download",
        signature: "Download(FromFile: Text, DialogTitle: Text, ToFolder: Text, ToFilter: Text, var ToFile: Text): Boolean",
        summary: "Sends a file from a server computer to the client computer.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-download-method",
        params: &[
            BuiltinMethodParam {
                label: "FromFile: Text",
                documentation: "The name of the file on the server computer that you want to download to the client computer.",
            },
            BuiltinMethodParam {
                label: "DialogTitle: Text",
                documentation: "The title that you want to display in the dialog box for downloading the file. This parameter is not supported by the web client. The title is determined by the end-user's browser.",
            },
            BuiltinMethodParam {
                label: "ToFolder: Text",
                documentation: "The default folder in which to save the file to be downloaded. The folder name is displayed in the dialog box for downloading the file. The folder can be changed by the user. This parameter is not supported by the web client. By default, the files are saved to the default download location that is configured in the end-user's browser.",
            },
            BuiltinMethodParam {
                label: "ToFilter: Text",
                documentation: "The type of file that can be downloaded to the client computer. The type is displayed in the dialog box for downloading the file. This parameter is not supported by the web client.",
            },
            BuiltinMethodParam {
                label: "ToFile: Text",
                documentation: "The name to give the downloaded file. This is the default file name that is shown in the dialog box for downloading the file. This value can be changed by the user.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DownloadFromStream",
        signature: "DownloadFromStream(InStream: InStream, DialogTitle: Text, ToFolder: Text, ToFilter: Text, var ToFile: Text): Boolean",
        summary: "Sends a file from server computer to the client computer.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-downloadfromstream-method",
        params: &[
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "An InStream that you want to use to send the data in a file on Business Central Server to a file on the client computer.",
            },
            BuiltinMethodParam {
                label: "DialogTitle: Text",
                documentation: "The title that you want to display in the dialog box for downloading the file. This parameter is not supported by the web client. The title is determined by the end-user's browser.",
            },
            BuiltinMethodParam {
                label: "ToFolder: Text",
                documentation: "The default folder in which to save the file to be downloaded. The folder name is displayed in the dialog box for downloading the file. The folder can be changed by the user. This parameter is not supported by the web client. By default, files are saved to the default download location that is configured in the end-user's browser.",
            },
            BuiltinMethodParam {
                label: "ToFilter: Text",
                documentation: "The type of file that can be downloaded to the client computer. The type is displayed in the dialog box for downloading the file. This parameter is not supported by the web client.",
            },
            BuiltinMethodParam {
                label: "ToFile: Text",
                documentation: "The name to give the downloaded file. This is the default file name that is shown in the dialog box for downloading the file. This value, can be changed by the user.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsPathTemporary",
        signature: "IsPathTemporary(Name: Text): Boolean",
        summary: "Validates whether the given path is located in the current users temporary folder within the current service.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-ispathtemporary-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the file, including the path.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Len",
        signature: "Len(): Integer",
        summary: "Gets the length of an ASCII or binary file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-len-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Gets the name of an ASCII or binary file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-name-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Pos",
        signature: "Pos(): Integer",
        summary: "Gets the current position of the file pointer in an ASCII or binary file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-pos-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Seek",
        signature: "Seek(Position: Integer)",
        summary: "Sets a file pointer to a new position in an ASCII or binary file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-seek-method",
        params: &[
            BuiltinMethodParam {
                label: "Position: Integer",
                documentation: "The position at which to set the pointer.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Trunc",
        signature: "Trunc()",
        summary: "Truncate an ASCII or binary file to the current position of the file pointer.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-trunc-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Upload",
        signature: "Upload(DialogTitle: Text, FromFolder: Text, FromFilter: Text, FromFile: Text, var ToFile: Text): Boolean",
        summary: "Sends a file from the client computer to the server computer.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-upload-method",
        params: &[
            BuiltinMethodParam {
                label: "DialogTitle: Text",
                documentation: "The title that you want to display in the dialog box for uploading the file. This parameter is not supported by the web client. The title is determined by the end-user's browser.",
            },
            BuiltinMethodParam {
                label: "FromFolder: Text",
                documentation: "The name of the folder that is displayed in the dialog box. This is the default value, and the user can change it. This parameter is not supported by the web client. The browser uses the folder that was last accessed.",
            },
            BuiltinMethodParam {
                label: "FromFilter: Text",
                documentation: "The type of file that can be uploaded to the server. In the Windows client, the type is displayed in the upload dialog box, so that the user can only select files of the specified type. For the web client, a user can try to upload any file type but an error occurs if the file is not the specified type.",
            },
            BuiltinMethodParam {
                label: "FromFile: Text",
                documentation: "The default file that you want to upload to the service. The name displays in the dialog box for uploading the file. The user can change the file. This parameter is not supported by the web client.",
            },
            BuiltinMethodParam {
                label: "ToFile: Text",
                documentation: "The path and file name to give the uploaded file. If you do not provide a path, or you upload the file that uses web client, then the file is uploaded to the following folder on the computer that is running the server: \\\\ProgramData\\\\Microsoft\\\\Microsoft Dynamics NAV\\\\110\\\\Server\\\\MicrosoftDynamicsNAVServer$DynamicsNAV110\\\\users\\\\ServiceAccount",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "UploadIntoStream",
        signature: "UploadIntoStream(DialogTitle: Text, FromFolder: Text, FromFilter: Text, var FromFile: Text, var InStream: InStream): Boolean",
        summary: "Sends a file from the client computer to the corresponding server.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-uploadintostream-string-string-string-text-instream-method",
        params: &[
            BuiltinMethodParam {
                label: "DialogTitle: Text",
                documentation: "The text displayed in the title bar of the Open dialog box. This parameter is not supported by the web client. The title is determined by the end-user's browser.",
            },
            BuiltinMethodParam {
                label: "FromFolder: Text",
                documentation: "The path of the folder that is displayed in the File Open dialog box. This is the default folder, but the user can browse to any available location. This parameter is not supported by the web client. By default, the browser uses the folder that was last accessed.",
            },
            BuiltinMethodParam {
                label: "FromFilter: Text",
                documentation: "The type of file that can be uploaded to the server. A user can try to upload any file type but an error occurs if the file is not the specified type.",
            },
            BuiltinMethodParam {
                label: "FromFile: Text",
                documentation: "The default file to upload to the service. The name displays in the dialog box for uploading the file. The user can change the file. This parameter is not supported by the web client.",
            },
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
];

const MISSING_FILTERPAGEBUILDER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddRecordRef",
        signature: "AddRecordRef(Name: Text, RecordRef: RecordRef): Text",
        summary: "Adds a filter control for a table to a filter page.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/filterpagebuilder/filterpagebuilder-addrecordref-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Assigns a name to the filter control for the table. The text displays as the caption for the filter control on the rendered filter page in the client.",
            },
            BuiltinMethodParam {
                label: "RecordRef: RecordRef",
                documentation: "The record reference to use in the filter control.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddTable",
        signature: "AddTable(Name: Text, TableNo: Integer): Text",
        summary: "Adds filter control for a table to a filter page.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/filterpagebuilder/filterpagebuilder-addtable-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Assigns a name to the filter control for the table. The text displays as the caption for the filter control on the rendered filter page in the client.",
            },
            BuiltinMethodParam {
                label: "TableNo: Integer",
                documentation: "The ID of the table object that you want to filter on.",
            },
        ],
    },
];

const MISSING_GUID_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CreateSequentialGuid",
        signature: "CreateSequentialGuid(): Guid",
        summary: "Creates a new sequential unique GUID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/guid/guid-createsequentialguid-method",
        params: PARAM_NONE,
    },
];

const MISSING_HTTPCLIENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear()",
        summary: "Sets the HttpClient variable to the default value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-clear-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetBaseAddress",
        signature: "GetBaseAddress(): Text",
        summary: "Gets the base address of Uniform Resource Identifier (URI) of the Internet resource used when sending requests.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-getbaseaddress-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Patch",
        signature: "Patch(Path: Text, Content: HttpContent, var Response: HttpResponseMessage): Boolean",
        summary: "Sends a PATCH request to the specified URI as an asynchronous operation.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-patch-method",
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "The path the request is sent to.",
            },
            BuiltinMethodParam {
                label: "Content: HttpContent",
                documentation: "The HTTP request content sent to the server.",
            },
            BuiltinMethodParam {
                label: "Response: HttpResponseMessage",
                documentation: "The response received from the remote endpoint.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetBaseAddress",
        signature: "SetBaseAddress(NewBaseAddress: Text): Boolean",
        summary: "Sets the base address of Uniform Resource Identifier (URI) of the Internet resource used when sending requests.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-setbaseaddress-method",
        params: &[
            BuiltinMethodParam {
                label: "NewBaseAddress: Text",
                documentation: "The base address of the Uniform Resource Identifier (URI) of the Internet resource used when sending requests.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "UseDefaultNetworkWindowsAuthentication",
        signature: "UseDefaultNetworkWindowsAuthentication(): Boolean",
        summary: "Sets the HttpClient credentials to use the default network credentials for Windows authentication.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-usedefaultnetworkwindowsauthentication-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "UseServerCertificateValidation",
        signature: "UseServerCertificateValidation(UseServerCertificateValidation: Boolean): Boolean",
        summary: "If true, the client validates the server certificate for all HTTP requests.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-useservercertificatevalidation-method",
        params: &[
            BuiltinMethodParam {
                label: "UseServerCertificateValidation: Boolean",
                documentation: "Set to `true` to validate the server certificate; when set to `false` the validation is skipped. Default value is `true`.",
            },
        ],
    },
];

const MISSING_HTTPCONTENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "IsSecretContent",
        signature: "IsSecretContent(): Boolean",
        summary: "Returns if the content is secret.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpcontent/httpcontent-issecretcontent-method",
        params: PARAM_NONE,
    },
];

const MISSING_HTTPHEADERS_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ContainsSecret",
        signature: "ContainsSecret(Key: Text): Boolean",
        summary: "Returns if the header for the given key has a secret value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpheaders/httpheaders-containssecret-method",
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "The specified header.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Keys",
        signature: "Keys(): List of [Text",
        summary: "Gets the key name of all the headers",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpheaders/httpheaders-keys-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TryAddWithoutValidation",
        signature: "TryAddWithoutValidation(Name: Text, Value: Text): Boolean",
        summary: "Adds the specified header and its value into the HttpHeaders collection.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpheaders/httpheaders-tryaddwithoutvalidation-string-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The header to add to the collection.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The content of the header.",
            },
        ],
    },
];

const MISSING_HTTPREQUESTMESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "GetCookie",
        signature: "GetCookie(Name: Text, var Cookie: Cookie): Boolean",
        summary: "Gets the specified cookie given a name.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-getcookie-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the cookie.",
            },
            BuiltinMethodParam {
                label: "Cookie: Cookie",
                documentation: "The cookie object.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetCookieNames",
        signature: "GetCookieNames(): List of [Text",
        summary: "Gets the list of cookie names.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-getcookienames-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetSecretRequestUri",
        signature: "GetSecretRequestUri(): SecretText",
        summary: "Gets the secret URI used for the HTTP request.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-getsecretrequesturi-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RemoveCookie",
        signature: "RemoveCookie(Name: Text): Boolean",
        summary: "Removes the specified cookie given a name.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-removecookie-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the cookie.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetCookie",
        signature: "SetCookie(Name: Text, Value: Text): Boolean",
        summary: "Sets the cookie given a name and value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-setcookie-string-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the cookie.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The value of the cookie.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetSecretRequestUri",
        signature: "SetSecretRequestUri(RequestUri: SecretText): Boolean",
        summary: "Sets the secret URI used for the HTTP request.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-setsecretrequesturi-method",
        params: &[
            BuiltinMethodParam {
                label: "RequestUri: SecretText",
                documentation: "The secret URI to use for the HTTP request.",
            },
        ],
    },
];

const MISSING_HTTPRESPONSEMESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "GetCookie",
        signature: "GetCookie(Name: Text, var Cookie: Cookie): Boolean",
        summary: "Gets the specified cookie given a name.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpresponsemessage/httpresponsemessage-getcookie-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the cookie.",
            },
            BuiltinMethodParam {
                label: "Cookie: Cookie",
                documentation: "The cookie object.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetCookieNames",
        signature: "GetCookieNames(): List of [Text",
        summary: "Gets the list of cookie names.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpresponsemessage/httpresponsemessage-getcookienames-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Headers",
        signature: "Headers(): HttpHeaders",
        summary: "Gets the HTTP response's HTTP headers.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpresponsemessage/httpresponsemessage-headers-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsBlockedByEnvironment",
        signature: "IsBlockedByEnvironment(): Boolean",
        summary: "Gets a value that indicates if the HTTP response is the result of the environment blocking an outgoing HTTP request.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpresponsemessage/httpresponsemessage-isblockedbyenvironment-method",
        params: PARAM_NONE,
    },
];

const MISSING_JSONARRAY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsToken",
        signature: "AsToken(): JsonToken",
        summary: "Converts the value in a JsonArray to a JsonToken data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-astoken-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetArray",
        signature: "GetArray(Index: Integer): JsonArray",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getarray-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetBigInteger",
        signature: "GetBigInteger(Index: Integer): BigInteger",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getbiginteger-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetBoolean",
        signature: "GetBoolean(Index: Integer): Boolean",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getboolean-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetByte",
        signature: "GetByte(Index: Integer): Byte",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getbyte-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetChar",
        signature: "GetChar(Index: Integer): Char",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getchar-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDate",
        signature: "GetDate(Index: Integer): Date",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getdate-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDateTime",
        signature: "GetDateTime(Index: Integer): DateTime",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getdatetime-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDecimal",
        signature: "GetDecimal(Index: Integer): Decimal",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getdecimal-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDuration",
        signature: "GetDuration(Index: Integer): Option",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getduration-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetInteger",
        signature: "GetInteger(Index: Integer): Integer",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getinteger-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetObject",
        signature: "GetObject(Index: Integer): JsonObject",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getobject-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetOption",
        signature: "GetOption(Index: Integer): Option",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-getoption-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetText",
        signature: "GetText(Index: Integer): Text",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-gettext-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetTime",
        signature: "GetTime(Index: Integer): Time",
        summary: "Retrieves the value at the given index in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-gettime-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the element in the JsonArray that you want to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IndexOf",
        signature: "IndexOf(Value: JsonToken): Integer",
        summary: "Determines the index of a specific value in the JsonArray.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-indexof-jsontoken-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: JsonToken",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Path",
        signature: "Path(): Text",
        summary: "Retrieves the JSON path of the array relative to the root of its containing tree.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-path-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RemoveAt",
        signature: "RemoveAt(Index: Integer): Boolean",
        summary: "Removes the token at the given index.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-removeat-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The position of the element that will be removed.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectToken",
        signature: "SelectToken(Path: Text, var Result: JsonToken): Boolean",
        summary: "Selects a JsonToken using a JPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-selecttoken-method",
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "A valid JPath expression.",
            },
            BuiltinMethodParam {
                label: "Result: JsonToken",
                documentation: "A **JsonToken** variable that will contain the result if the operation is successful.",
            },
        ],
    },
];

const MISSING_JSONOBJECT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsToken",
        signature: "AsToken(): JsonToken",
        summary: "Converts the value in a JsonObject to a JsonToken data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-astoken-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Path",
        signature: "Path(): Text",
        summary: "Retrieves the JSON path of the object relative to the root of its containing tree.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-path-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadFromYaml",
        signature: "ReadFromYaml(String: Text): Boolean",
        summary: "Reads the YAML data from the string into a JsonObject variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-readfromyaml-string-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The String object from which the YAML data will be read.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Values",
        signature: "Values(): List of [JsonToken",
        summary: "Gets a set of values of the JsonObject.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-values-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "WriteToYaml",
        signature: "WriteToYaml(var String: Text): Boolean",
        summary: "Serializes and writes the JsonObject as YAML to a given Text object.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-writetoyaml-text-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The Text object to which the YAML data will be written.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteWithSecretsTo",
        signature: "WriteWithSecretsTo(Path: Text, Secret: SecretText, var Result: SecretText): Boolean",
        summary: "Replaces the placeholder value in the path with the secret and then serializes and writes the content of the JsonObject to a SecretText.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-writewithsecretsto-text-secrettext-secrettext-method",
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "The path to the placeholder value, which will be replaced by the secret, in a JPath format.",
            },
            BuiltinMethodParam {
                label: "Secret: SecretText",
                documentation: "The secret with which the placeholder value will be replaced.",
            },
            BuiltinMethodParam {
                label: "Result: SecretText",
                documentation: "The SecretText object to which the JSON data will be written.",
            },
        ],
    },
];

const MISSING_JSONVALUE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsBigInteger",
        signature: "AsBigInteger(): BigInteger",
        summary: "Converts the value in a JsonValue to an BigInteger data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-asbiginteger-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsByte",
        signature: "AsByte(): Byte",
        summary: "Converts the value in a JsonValue to a Byte data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-asbyte-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsChar",
        signature: "AsChar(): Char",
        summary: "Converts the value in a JsonValue to a Char data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-aschar-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsCode",
        signature: "AsCode(): Code",
        summary: "Converts the value in a JsonValue to a Code data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-ascode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDuration",
        signature: "AsDuration(): Duration",
        summary: "Converts the value in a JsonValue to a Duration data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-asduration-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsOption",
        signature: "AsOption(): Option",
        summary: "Converts the value in a JsonValue to an Option data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-asoption-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsToken",
        signature: "AsToken(): JsonToken",
        summary: "Converts the value in a JsonValue to a JsonToken data type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-astoken-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Clone",
        signature: "Clone(): JsonToken",
        summary: "Creates a deep-copy of the JsonToken value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-clone-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsUndefined",
        signature: "IsUndefined(): Boolean",
        summary: "Indicates whether the JsonValue contains the JSON value of UNDEFINED.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-isundefined-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Path",
        signature: "Path(): Text",
        summary: "Retrieves the JSON path of the value relative to its containing tree.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-path-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SelectToken",
        signature: "SelectToken(Path: Text, var Result: JsonToken): Boolean",
        summary: "Selects a JsonToken using a JPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-selecttoken-method",
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
            BuiltinMethodParam {
                label: "Result: JsonToken",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetValue",
        signature: "SetValue(Value: Boolean)",
        summary: "Set the contents of the JsonValue variable to the JSON representation of the given value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-setvalue-boolean-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Boolean",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetValueToNull",
        signature: "SetValueToNull()",
        summary: "Set the contents of the JsonValue variable to the JSON representation of NULL.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-setvaluetonull-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetValueToUndefined",
        signature: "SetValueToUndefined()",
        summary: "Set the contents of the JsonValue variable to the JSON representation of UNDEFINED.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-setvaluetoundefined-method",
        params: PARAM_NONE,
    },
];

const MISSING_KEYREF_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Active",
        signature: "Active(): Boolean",
        summary: "Indicates whether the key is enabled.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/keyref/keyref-active-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Record",
        signature: "Record(): RecordRef",
        summary: "Returns a RecordRef for the current record referred to by the key.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/keyref/keyref-record-method",
        params: PARAM_NONE,
    },
];

const MISSING_LIST_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "LastIndexOf",
        signature: "LastIndexOf(Value: T): Integer",
        summary: "Searches for the specified value and returns the one-based index of the last occurrence within the entire List.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/list/list-lastindexof-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: T",
                documentation: "The value to locate in the List.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "RemoveRange",
        signature: "RemoveRange(Index: Integer, Count: Integer): Boolean",
        summary: "Removes a range of elements from the List.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/list/list-removerange-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The one-based starting index of the range of elements to remove.",
            },
            BuiltinMethodParam {
                label: "Count: Integer",
                documentation: "The number of elements to remove.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Reverse",
        signature: "Reverse()",
        summary: "Reverses the order of the elements in the entire List.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/list/list-reverse-method",
        params: PARAM_NONE,
    },
];

const MISSING_MEDIA_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "FindOrphans",
        signature: "FindOrphans(): List of [Guid",
        summary: "Discovers all orphaned media.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/media/media-findorphans-method",
        params: PARAM_NONE,
    },
];

const MISSING_MEDIASET_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ExportFile",
        signature: "ExportFile(FilenamePrefix: Text): Integer",
        summary: "Exports the media objects in the current media set of a record to individual files on your computer or network.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/mediaset/mediaset-exportfile-method",
        params: &[
            BuiltinMethodParam {
                label: "FilenamePrefix: Text",
                documentation: "Specifies the location and name of the exported media files. Each exported media file is given a name that consists of a prefix that you specify, plus an index number that is automatically assigned. The file name has the format prefix-index.type, for example, Image-1.jpg, Image-2.jpg, and Image-3.jpg. To set the parameter value, use the format: path\\\\prefix.type. -   path is the folder path where you want to store the files. -   prefix is the text that you want before the index number. -   type is the media type extension.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindOrphans",
        signature: "FindOrphans(): List of [Guid",
        summary: "Discovers all orphaned media sets.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/mediaset/mediaset-findorphans-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(MediaId: Guid): Boolean",
        summary: "Adds a media object that already exists in the database to a MediaSet of a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/mediaset/mediaset-insert-method",
        params: &[
            BuiltinMethodParam {
                label: "MediaId: Guid",
                documentation: "Specifies the unique ID that is assigned to the media object that you want to insert. Existing media objects are stored in the system table 2000000184 Tenant Media of the application database. There are different ways of obtaining the GUID of a media object. You could identify the media object ID by looking in the table. Or programmatically, you can use either the Item function on a MediaSet data type field of a record or the MEDIAID function on Media data type field of a record.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "MediaId",
        signature: "MediaId(): Guid",
        summary: "Gets the unique identifier that is assigned to a MediaSet of a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/mediaset/mediaset-mediaid-method",
        params: PARAM_NONE,
    },
];

const MISSING_MODULEINFO_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Dependencies",
        signature: "Dependencies(): List of [ModuleDependencyInfo",
        summary: "Gets the collection of application dependencies.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/moduleinfo/moduleinfo-dependencies-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "PackageId",
        signature: "PackageId(): Guid",
        summary: "Gets the package ID of the specified application.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/moduleinfo/moduleinfo-packageid-method",
        params: PARAM_NONE,
    },
];

const MISSING_NAVAPP_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "DeleteArchiveData",
        signature: "DeleteArchiveData(TableNo: Integer)",
        summary: "Deletes the archived data for a specified table of an extension during installation.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-deletearchivedata-method",
        params: &[
            BuiltinMethodParam {
                label: "[TableNo: Integer]",
                documentation: "The ID of the table for which to delete archived data. If you omit this optional return value and if archived data cannot be deleted for the specified table, then a run-time error occurs. If you include a return value, then it is assumed that you will handle any errors and no run-time error occurs, even though the archived data is not deleted.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetArchiveRecordRef",
        signature: "GetArchiveRecordRef(TableNo: Integer, var RecordRef: RecordRef): Boolean",
        summary: "Returns a RecordRef for the specified table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-getarchiverecordref-method",
        params: &[
            BuiltinMethodParam {
                label: "TableNo: Integer",
                documentation: "Specifies the table ID.",
            },
            BuiltinMethodParam {
                label: "RecordRef: RecordRef",
                documentation: "Specifies the reference.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetArchiveVersion",
        signature: "GetArchiveVersion(): Text",
        summary: "Returns the version of the extension that the specified table is part of.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-getarchiveversion-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetCallerCallstackModuleInfos",
        signature: "GetCallerCallstackModuleInfos(): List of [ModuleInfo",
        summary: "Gets information about extensions on the callstack that contain the method, which called the currently running method.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-getcallercallstackmoduleinfos-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetCallerModuleInfo",
        signature: "GetCallerModuleInfo(var Info: ModuleInfo): Boolean",
        summary: "Gets information about the extension that contains the method that called the currently running method.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-getcallermoduleinfo-method",
        params: &[
            BuiltinMethodParam {
                label: "Info: ModuleInfo",
                documentation: "A value containing information about the calling application.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsInstalling",
        signature: "IsInstalling(): Boolean",
        summary: "Returns **true** if the application that contains the AL object that is currently running is being installed, otherwise it returns **false**.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-isinstalling-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LoadPackageData",
        signature: "LoadPackageData(TableNo: Integer)",
        summary: "Loads default, or starting, table data into the specified table of an extension during installation.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-loadpackagedata-method",
        params: &[
            BuiltinMethodParam {
                label: "TableNo: Integer",
                documentation: "The ID of the table for which to load package data.",
            },
        ],
    },
];

const MISSING_NOTIFICATION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "HasData",
        signature: "HasData(Name: Text): Boolean",
        summary: "Checks if data was passed to a notification instance as specified by a SETDATA method call.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/notification/notification-hasdata-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The name of the data item that is specified by the SETDATA method call.",
            },
        ],
    },
];

const MISSING_PAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CancelBackgroundTask",
        signature: "CancelBackgroundTask(TaskId: Integer): Boolean",
        summary: "Attempt to cancel a page background task.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/page/page-cancelbackgroundtask-method",
        params: &[
            BuiltinMethodParam {
                label: "TaskId: Integer",
                documentation: "Specifies the ID of the page background task to cancel. The ID is assigned to the task when it is queued by the EnqueueBackgroundTask method.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetBackgroundParameters",
        signature: "GetBackgroundParameters(): Dictionary of [Text, Text",
        summary: "Gets the page background task input parameters.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/page/page-getbackgroundparameters-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SaveRecord",
        signature: "SaveRecord()",
        summary: "Saves the current record as if performed by the client.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/page/page-saverecord-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetSelectionFilter",
        signature: "SetSelectionFilter(var Record: Record)",
        summary: "Notes the records that the user has selected on the page, marks those records in the table specified, and sets the filter to marked only.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/page/page-setselectionfilter-method",
        params: &[
            BuiltinMethodParam {
                label: "Record: Record",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
];

const MISSING_PRODUCTNAME_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Marketing",
        signature: "Marketing(): Text",
        summary: "MARKETING returns a text string that contains the application's marketing name.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/productname/productname-marketing-method",
        params: PARAM_NONE,
    },
];

const MISSING_QUERY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ColumnCaption",
        signature: "ColumnCaption(Column: Any): Text",
        summary: "Returns the current caption of a query column as a text string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/queryinstance-columncaption-method",
        params: &[
            BuiltinMethodParam {
                label: "Column: Any",
                documentation: "Refers to the name of the query column. The name of a query column is specified by the Name Property.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ColumnName",
        signature: "ColumnName(Column: Any): Text",
        summary: "Returns the name of a query column as a text string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/queryinstance-columnname-method",
        params: &[
            BuiltinMethodParam {
                label: "Column: Any",
                documentation: "Refers to the name of the query column. The name of a query column is specified by the Name Property.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ColumnNo",
        signature: "ColumnNo(Column: Any): Integer",
        summary: "Returns the ID that is assigned to a query column in the query definition.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/queryinstance-columnno-method",
        params: &[
            BuiltinMethodParam {
                label: "Column: Any",
                documentation: "Refers to the name of the query column. The name of a query column is specified by the Name Property.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetFilter",
        signature: "GetFilter(Column: Any): Text",
        summary: "Returns the filters that are set on the field of a specified column in the query.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/queryinstance-getfilter-method",
        params: &[
            BuiltinMethodParam {
                label: "Column: Any",
                documentation: "The name of the column in the query. A column name is defined by the Name Property.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetFilters",
        signature: "GetFilters(): Text",
        summary: "Returns the filters that are applied to all columns in the query.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/queryinstance-getfilters-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SaveAsJson",
        signature: "SaveAsJson(Number: Integer, OutStream: OutStream): Boolean",
        summary: "Saves the resulting data set of a query as an .json file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/query-saveasjson-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "The ID of the query object that you want to save as an .json file. If the query that you specify does not exist, then a run-time error occurs.",
            },
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The stream that you want to save the query as Json to.",
            },
        ],
    },
];

const MISSING_RECORD_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AreFieldsLoaded",
        signature: "AreFieldsLoaded(Fields: Any, ...): Boolean",
        summary: "Checks whether the specified fields are all initially loaded.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-arefieldsloaded-method",
        params: &[
            BuiltinMethodParam {
                label: "Fields: Any",
                documentation: "The FieldNo's of the fields to check.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ClearMarks",
        signature: "ClearMarks()",
        summary: "Removes all the marks from a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-clearmarks-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Consistent",
        signature: "Consistent(Consistent: Boolean)",
        summary: "Marks a table as being consistent or inconsistent.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-consistent-method",
        params: &[
            BuiltinMethodParam {
                label: "Consistent: Boolean",
                documentation: "The mark to be set on the table. If this parameter is true, the table is marked as consistent. If this parameter is false, the table is marked as inconsistent.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CopyFilter",
        signature: "CopyFilter(FromField: Any, ToField: Any)",
        summary: "Copies the filter that has been set for one field and applies it to another field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-copyfilter-method",
        params: &[
            BuiltinMethodParam {
                label: "FromField: Any",
                documentation: "The field from which the filter will be copied.",
            },
            BuiltinMethodParam {
                label: "ToField: Any",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CopyFilters",
        signature: "CopyFilters(var FromRecord: Record)",
        summary: "Copies all the filters set by the SETFILTER method (Record) or the SETRANGE method (Record) from one record to another.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-copyfilters-method",
        params: &[
            BuiltinMethodParam {
                label: "FromRecord: Record",
                documentation: "The record from which you want to copy filters.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CopyLinks",
        signature: "CopyLinks(var FromRecord: Record)",
        summary: "Copies all the links from a specified record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-copylinks-table-method",
        params: &[
            BuiltinMethodParam {
                label: "FromRecord: Record",
                documentation: "Specifies the record from which you want to copy links.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Counts the number of records in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-count-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CountApprox",
        signature: "CountApprox(): Integer",
        summary: "Returns an approximate count of the number of records in the table, for example, for updating progress bars or displaying informational messages.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-countapprox-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CurrentCompany",
        signature: "CurrentCompany(): Text",
        summary: "Gets the current company of a database table record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-currentcompany-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CurrentKey",
        signature: "CurrentKey(): Text",
        summary: "Gets the current key of a database table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-currentkey-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DeleteLink",
        signature: "DeleteLink(ID: Integer)",
        summary: "Deletes a specified link from a record in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-deletelink-method",
        params: &[
            BuiltinMethodParam {
                label: "ID: Integer",
                documentation: "The ID of the link to delete.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DeleteLinks",
        signature: "DeleteLinks()",
        summary: "Deletes all of the links that have been added to a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-deletelinks-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FieldActive",
        signature: "FieldActive(Field: Any): Boolean",
        summary: "Checks whether a field is enabled.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-fieldactive-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field that you want to check.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FieldCaption",
        signature: "FieldCaption(Field: Any): Text",
        summary: "Gets the current caption of the specified field as a string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-fieldcaption-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field for which you want to retrieve the caption.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FieldError",
        signature: "FieldError(Field: Any, ErrorInfo: ErrorInfo)",
        summary: "Stops the execution of the code causing a run-time error, and creates an error message for a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-fielderror-joker-errorinfo-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field for which you want to create an error message.",
            },
            BuiltinMethodParam {
                label: "ErrorInfo: ErrorInfo",
                documentation: "Additional information to include in the error if the test fails.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FieldName",
        signature: "FieldName(Field: Any): Text",
        summary: "Gets the name of a field as a string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-fieldname-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The name of the field in the record.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FieldNo",
        signature: "FieldNo(Field: Any): Integer",
        summary: "Gets the number assigned to a field in the table description.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-fieldno-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The name of the field in the record.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindFirst",
        signature: "FindFirst(): Boolean",
        summary: "Finds the first record in a table based on the current key and filter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-findfirst-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FindLast",
        signature: "FindLast(): Boolean",
        summary: "Finds the last record in a table based on the current key and filter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-findlast-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FindSet",
        signature: "FindSet(ForUpdate: Boolean, UpdateKey: Boolean): Boolean",
        summary: "Finds a set of records in a table based on the current key and filter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-findset-boolean-boolean-method",
        params: &[
            BuiltinMethodParam {
                label: "ForUpdate: Boolean",
                documentation: "Set this parameter to true if you want to modify any records in the set; otherwise, set the parameter to false. If you set this parameter to true, then the records will be read with Updlock. If you set this parameter to false, then you can still modify the records in the set, but these updates will not be performed optimally. The default value is false.",
            },
            BuiltinMethodParam {
                label: "UpdateKey: Boolean",
                documentation: "The `UpdateKey` parameter has been deprecated and will be removed in the future.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetAscending",
        signature: "GetAscending(Field: Any): Boolean",
        summary: "Gets the sort order for the records returned.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-getascending-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field that you want to get the sort order for.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetBySystemId",
        signature: "GetBySystemId(SystemId: Guid): Boolean",
        summary: "Gets a record by its SystemId.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-getbysystemid-method",
        params: &[
            BuiltinMethodParam {
                label: "SystemId: Guid",
                documentation: "The SystemId of the record to retrieve.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetFilter",
        signature: "GetFilter(Field: Any): Text",
        summary: "Gets a list of the filters within the current filter group that are applied to a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-getfilter-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The input field.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetFilters",
        signature: "GetFilters(): Text",
        summary: "Gets a string that contains a list of the filters within the current filter group for all fields in a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-getfilters-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetRangeMax",
        signature: "GetRangeMax(Field: Any): Any",
        summary: "Gets the maximum value in a range for a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-getrangemax-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field for which you want to find the maximum value. The current filter on Field must be a single range filter; otherwise, a run-time error occurs.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetRangeMin",
        signature: "GetRangeMin(Field: Any): Any",
        summary: "Gets the minimum value in a range for a field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-getrangemin-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field for which you want to find the minimum value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "HasFilter",
        signature: "HasFilter(): Boolean",
        summary: "Determines whether a filter is attached to a record within the current filter group.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-hasfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasLinks",
        signature: "HasLinks(): Boolean",
        summary: "Determines whether a record contains any links.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-haslinks-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Init",
        signature: "Init()",
        summary: "Initializes a record in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-init-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(): Boolean",
        summary: "Inserts a record into a table without executing the code in the OnInsert trigger.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-insert--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsEmpty",
        signature: "IsEmpty(): Boolean",
        summary: "Determines whether a table or a filtered set of records is empty.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-isempty-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTemporary",
        signature: "IsTemporary(): Boolean",
        summary: "Determines whether a record refers to a temporary table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-istemporary-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LoadFields",
        signature: "LoadFields(Fields: Any, ...): Boolean",
        summary: "Accesses the table's corresponding data source and loads the values of the specified fields on the record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-loadfields-method",
        params: &[
            BuiltinMethodParam {
                label: "Fields: Any",
                documentation: "The FieldNo's of the fields to be loaded.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ReadConsistency",
        signature: "ReadConsistency(): Boolean",
        summary: "Determines if the table supports read consistency.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-readconsistency-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadPermission",
        signature: "ReadPermission(): Boolean",
        summary: "Determines whether a user is granted read permission to the table that contains a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-readpermission-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RecordId",
        signature: "RecordId(): RecordId",
        summary: "Gets the RecordId of the record that is currently selected in the table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-recordid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RecordLevelLocking",
        signature: "RecordLevelLocking(): Boolean",
        summary: "Determines whether the table supports record-level locking.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-recordlevellocking-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Relation",
        signature: "Relation(Field: Any): Integer",
        summary: "Determines the table relationship of a given field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-relation-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field for which you want to find the table relationship.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Reset",
        signature: "Reset()",
        summary: "Removes all filters, including any special filters set by MarkedOnly, changes fields select for loading back to all, sets the read isolation level to the default value, and changes the current key to the primary key.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-reset-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetAscending",
        signature: "SetAscending(Field: Any, Ascending: Boolean)",
        summary: "Sets the sort order for the records returned.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-setascending-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field that you want to set the sort order for.",
            },
            BuiltinMethodParam {
                label: "Ascending: Boolean",
                documentation: "The sort order. Specify false if data must be sorted in descending order; otherwise true.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetBaseLoadFields",
        signature: "SetBaseLoadFields(): Boolean",
        summary: "Sets that only fields for the base table to be initially loaded when the record is retrieved from its data source.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-setbaseloadfields-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetPermissionFilter",
        signature: "SetPermissionFilter()",
        summary: "Applies the user's security filter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-setpermissionfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetPosition",
        signature: "SetPosition(String: Text)",
        summary: "Sets the fields in a primary key on a record to the values specified in the supplied string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-setposition-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string that is used to set the primary key. This string contains the primary key value to set.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetRecFilter",
        signature: "SetRecFilter()",
        summary: "Sets the values in the current key of the current record as a record filter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-setrecfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetView",
        signature: "SetView(String: Text)",
        summary: "Sets the current sort order, key, and filters on a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-setview-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "A string that contains the sort order, key, and filter to set. The string format is the same as the SourceTableView Property on pages.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "TableCaption",
        signature: "TableCaption(): Text",
        summary: "Gets the current caption of a table as a string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-tablecaption-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TableName",
        signature: "TableName(): Text",
        summary: "Gets the name of a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-tablename-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TestField",
        signature: "TestField(Field: Any)",
        summary: "Tests that the content of the field is not zero or blank (empty string).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-testfield-joker-method",
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "The field that you want to test.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "TransferFields",
        signature: "TransferFields(var FromRecord: Record, InitPrimaryKeyFields: Boolean, SkipFieldsNotMatchingType: Boolean)",
        summary: "Copies all matching fields in one record to another record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-transferfields-table-boolean-boolean-method",
        params: &[
            BuiltinMethodParam {
                label: "FromRecord: Record",
                documentation: "The record from which to copy.",
            },
            BuiltinMethodParam {
                label: "InitPrimaryKeyFields: Boolean",
                documentation: "Default: true If this parameter is true and the records are in the same table, both the timestamp and the Primary Key fields of the destination record will be changed. If this parameter is true and the records are not in the same table, then the Primary Key fields of the destination record will be changed but the timestamp of the destination record will not be changed. If this parameter is false, then neither the timestamp nor the Primary Key fields of the destination record are changed.",
            },
            BuiltinMethodParam {
                label: "SkipFieldsNotMatchingType: Boolean",
                documentation: "Specifies whether fields where the field type on the source record do not match the field type on the target record should be ignored.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WritePermission",
        signature: "WritePermission(): Boolean",
        summary: "Determines whether a user can write to a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-writepermission-method",
        params: PARAM_NONE,
    },
];

const MISSING_RECORDREF_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AreFieldsLoaded",
        signature: "AreFieldsLoaded(Fields: Integer, ...): Boolean",
        summary: "Checks whether the specified fields are all initially loaded.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-arefieldsloaded-method",
        params: &[
            BuiltinMethodParam {
                label: "Fields: Integer",
                documentation: "The FieldNo's of the fields to check.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption(): Text",
        summary: "Gets the caption of the table that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-caption-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ClearMarks",
        signature: "ClearMarks()",
        summary: "Removes all the marks from a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-clearmarks-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CopyLinks",
        signature: "CopyLinks(FromRecord: Record)",
        summary: "Copies all the links from a particular record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-copylinks-table-method",
        params: &[
            BuiltinMethodParam {
                label: "FromRecord: Record",
                documentation: "Specifies the record from which you want to copy links.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Counts the number of records that are in the filters that are currently applied to the table referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-count-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CountApprox",
        signature: "CountApprox(): Integer",
        summary: "Gets an approximate count of the number of records in the table",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-countapprox-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CurrentCompany",
        signature: "CurrentCompany(): Text",
        summary: "Gets the current company of a database table referred to by a RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-currentcompany-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CurrentKey",
        signature: "CurrentKey(): Text",
        summary: "Gets the current key of the table referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-currentkey-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DeleteLink",
        signature: "DeleteLink(ID: Integer)",
        summary: "Deletes a specified link from a record in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-deletelink-method",
        params: &[
            BuiltinMethodParam {
                label: "ID: Integer",
                documentation: "The ID of the link you want to delete.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DeleteLinks",
        signature: "DeleteLinks()",
        summary: "Deletes all of the links that have been added to a record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-deletelinks-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Duplicate",
        signature: "Duplicate(): RecordRef",
        summary: "Duplicates the table that contains the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-duplicate-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FieldCount",
        signature: "FieldCount(): Integer",
        summary: "Gets the number of fields in the table that is currently selected or returns the number of fields that have been defined in a key.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-fieldcount-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FieldExist",
        signature: "FieldExist(FieldNo: Integer): Boolean",
        summary: "Determines if the field that has the number FieldNo exists in the table that is referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-fieldexist-integer-method",
        params: &[
            BuiltinMethodParam {
                label: "FieldNo: Integer",
                documentation: "The FieldNo that you want to know whether it exists in the table.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FieldIndex",
        signature: "FieldIndex(Index: Integer): FieldRef",
        summary: "Gets the FieldRef of the field that has the specified index in the table that is referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-fieldindex-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The index of the field.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindLast",
        signature: "FindLast(): Boolean",
        summary: "Finds the last record in a table based on the current key and filter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-findlast-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(RecordID: RecordId): Boolean",
        summary: "Gets a record based on the ID of the record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-get-method",
        params: &[
            BuiltinMethodParam {
                label: "RecordID: RecordId",
                documentation: "The RecordID that contains the table number and the primary key of the table and is used to identify the record that you want to get.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetBySystemId",
        signature: "GetBySystemId(SystemId: Guid): Boolean",
        summary: "Gets a record based on the ID of the record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-getbysystemid-method",
        params: &[
            BuiltinMethodParam {
                label: "SystemId: Guid",
                documentation: "The systemid which uniquely identifies the record that you want to get.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetFilters",
        signature: "GetFilters(): Text",
        summary: "Determines which filters have been applied to the table referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-getfilters-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasFilter",
        signature: "HasFilter(): Boolean",
        summary: "Determines whether a filter has been applied to the table that the RecordRef refers to.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-hasfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasLinks",
        signature: "HasLinks(): Boolean",
        summary: "Determines whether a record contains any links.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-haslinks-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Init",
        signature: "Init()",
        summary: "Initializes a record in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-init-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(): Boolean",
        summary: "Inserts a record into a table without executing the code in the OnInsert trigger.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-insert--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDirty",
        signature: "IsDirty(): Boolean",
        summary: "Gets a boolean value that indicates whether the current in-memory instance of a record or filtered set of records has changed since being retrieved from the database.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-isdirty-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsEmpty",
        signature: "IsEmpty(): Boolean",
        summary: "Determines whether any records exist in a filtered set of records in a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-isempty-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTemporary",
        signature: "IsTemporary(): Boolean",
        summary: "Determines whether a RecordRef refers to a temporary table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-istemporary-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "KeyCount",
        signature: "KeyCount(): Integer",
        summary: "Gets the number of keys that exist in the table that is referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-keycount-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "KeyIndex",
        signature: "KeyIndex(Index: Integer): KeyRef",
        summary: "Gets the KeyRef of the key that has the index specified in the table that is currently selected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-keyindex-method",
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "The number of the index in which you are interested.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "LoadFields",
        signature: "LoadFields(Fields: Integer, ...): Boolean",
        summary: "Accesses the table's corresponding data source and loads the values of the specified fields on the record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-loadfields-method",
        params: &[
            BuiltinMethodParam {
                label: "Fields: Integer",
                documentation: "The FieldNo's of the fields to be loaded.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Identifies the name of the table",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-name-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Number",
        signature: "Number(): Integer",
        summary: "Gets the table ID (number) of the table that contains the record that was referred to by the RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-number-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadConsistency",
        signature: "ReadConsistency(): Boolean",
        summary: "Gets a value indicating whether read consistency is enabled.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-readconsistency-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadPermission",
        signature: "ReadPermission(): Boolean",
        summary: "Determines if you can read from a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-readpermission-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RecordId",
        signature: "RecordId(): RecordId",
        summary: "Gets the RecordID of the record that is currently selected in the table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-recordid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RecordLevelLocking",
        signature: "RecordLevelLocking(): Boolean",
        summary: "Gets a value indicating whether record level locking is enabled.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-recordlevellocking-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Reset",
        signature: "Reset()",
        summary: "Removes all filters, including any special filters set by the MarkedOnly method (Record), changes fields select for loading back to all, sets the read isolation level to the default value, and changes the current key to the primary key.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-reset-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetPermissionFilter",
        signature: "SetPermissionFilter()",
        summary: "Applies the user's security filter to the referenced record.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-setpermissionfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetPosition",
        signature: "SetPosition(String: Text)",
        summary: "Sets the fields in a primary key on a record to the values specified in the String parameter.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-setposition-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string that is used to set the primary key. This string contains the primary key value to set.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetRecFilter",
        signature: "SetRecFilter()",
        summary: "Sets a filter on a record that is referred to by a RecordRef.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-setrecfilter-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetView",
        signature: "SetView(String: Text)",
        summary: "Sets the current sort order, key, and filters on a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-setview-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "A string that contains the sort order, key, and filter to set. The string format is the same as the SourceTableView property on pages.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SystemCreatedAtNo",
        signature: "SystemCreatedAtNo(): Integer",
        summary: "Gets the field number that is used by the SystemCreatedAt field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-systemcreatedatno-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SystemCreatedByNo",
        signature: "SystemCreatedByNo(): Integer",
        summary: "Gets the field number that is used by the SystemCreatedBy field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-systemcreatedbyno-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SystemIdNo",
        signature: "SystemIdNo(): Integer",
        summary: "Gets the field number that is used by the SystemId field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-systemidno-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SystemModifiedAtNo",
        signature: "SystemModifiedAtNo(): Integer",
        summary: "Gets the field number that is used by the SystemModifiedAt field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-systemmodifiedatno-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SystemModifiedByNo",
        signature: "SystemModifiedByNo(): Integer",
        summary: "Gets the field number that is used by the SystemModifiedBy field.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-systemmodifiedbyno-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "WritePermission",
        signature: "WritePermission(): Boolean",
        summary: "Determines if you can write to a table.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-writepermission-method",
        params: PARAM_NONE,
    },
];

const MISSING_REPORT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Break",
        signature: "Break()",
        summary: "Exits from a loop or a trigger in a data item trigger of a report or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-break-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DefaultLayout",
        signature: "DefaultLayout(Number: Integer): DefaultLayout",
        summary: "Gets the default built-in layout type that is used on a specified report.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-defaultlayout-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "The ID of the report that you want to save. If the report that you specify does not exist, then a run-time error occurs.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ExcelLayout",
        signature: "ExcelLayout(Number: Integer, InStream: InStream): Boolean",
        summary: "Gets the Excel layout that is used on a report and returns it as a data stream.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-excellayout-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "The ID of the report object for which you want to get the Excel layout.",
            },
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "The variable in which to return the Excel layout.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetSubstituteReportId",
        signature: "GetSubstituteReportId(ReportId: Integer): Integer",
        summary: "Gets the ID of the report that will be run by the platform after considering any substitutions made by extensions.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-getsubstitutereportid-method",
        params: &[
            BuiltinMethodParam {
                label: "ReportId: Integer",
                documentation: "The ID of the report for which you want to retrieve the ID of the possible report substitute.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsReadOnly",
        signature: "IsReadOnly(): Boolean",
        summary: "Gets if the current report's data access intent is readonly.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-isreadonly-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "NewPage",
        signature: "NewPage()",
        summary: "Forces a page break when printing a report.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-newpage-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Preview",
        signature: "Preview(): Boolean",
        summary: "Indicates whether a report is being printed in preview mode.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-preview-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Quit",
        signature: "Quit()",
        summary: "Aborts the processing of a report or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-quit-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RdlcLayout",
        signature: "RdlcLayout(Number: Integer, InStream: InStream): Boolean",
        summary: "Gets the RDLC layout that is used on a report and returns it as a data stream.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-rdlclayout-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "The ID of the report object for which you want to get the RDLC layout.",
            },
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "The variable in which to return the RDLC layout.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SaveAsExcel",
        signature: "SaveAsExcel(FileName: Text): Boolean",
        summary: "Saves a report on the computer that is running the server as a Microsoft Excel (.xls) workbook.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-saveasexcel-method",
        params: &[
            BuiltinMethodParam {
                label: "FileName: Text",
                documentation: "The path and the name of the file that you want to save the report as. The path must exist, the file must not be being used, and the server process must have permission to write to the file. Otherwise, you will get errors.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SaveAsHtml",
        signature: "SaveAsHtml(FileName: Text): Boolean",
        summary: "Saves a report as an HTML file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-saveashtml-method",
        params: &[
            BuiltinMethodParam {
                label: "FileName: Text",
                documentation: "The folder path and name of the file that you want to save the report as. The path must already exist and the service (login) account that is used by the server instance must have permission to write to the target folder. Otherwise, you will get errors.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SaveAsPdf",
        signature: "SaveAsPdf(FileName: Text): Boolean",
        summary: "Saves a report as a .pdf file.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-saveaspdf-method",
        params: &[
            BuiltinMethodParam {
                label: "FileName: Text",
                documentation: "The path and name of the file that you want to save the report as.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SaveAsWord",
        signature: "SaveAsWord(FileName: Text): Boolean",
        summary: "Saves a report on the computer that is running the server as a Microsoft Word (.doc) document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-saveasword-method",
        params: &[
            BuiltinMethodParam {
                label: "FileName: Text",
                documentation: "The path and the name of the file that you want to save the report as. The path must exist, the file must not be being used, and the server process must have permission to write to the file. Otherwise, you will get errors.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SaveAsXml",
        signature: "SaveAsXml(FileName: Text): Boolean",
        summary: "Saves the resulting data set of a query as an .xml file.The following code shows the syntax of the SaveAsXml method.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-saveasxml-method",
        params: &[
            BuiltinMethodParam {
                label: "FileName: Text",
                documentation: "The path and name of the file that you want to save the query to.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ShowOutput",
        signature: "ShowOutput(): Boolean",
        summary: "Returns the current setting of whether a section should be printed, and changes this setting.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-showoutput--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Skip",
        signature: "Skip()",
        summary: "Skips the current iteration of the current report or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-skip-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TargetFormat",
        signature: "TargetFormat(): ReportFormat",
        summary: "Gets the current report's target format.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-targetformat-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TotalsCausedBy",
        signature: "TotalsCausedBy(): Integer",
        summary: "Determines which field caused a group total to be calculated.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/reportinstance-totalscausedby-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ValidateAndPrepareLayout",
        signature: "ValidateAndPrepareLayout(Number: Integer, LayoutStream: InStream, var PreparedLayoutStream: InStream, ReportLayoutType: ReportLayoutType): Boolean",
        summary: "Validates if the provided report layout is compatible with the specified report and performs the required modifications so that it can be used for rendering.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-validateandpreparelayout-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "The ID of the report to ensure compatibility with.",
            },
            BuiltinMethodParam {
                label: "LayoutStream: InStream",
                documentation: "The stream containing the layout to be validated.",
            },
            BuiltinMethodParam {
                label: "PreparedLayoutStream: InStream",
                documentation: "The stream in which to return the prepared layout.",
            },
            BuiltinMethodParam {
                label: "ReportLayoutType: ReportLayoutType",
                documentation: "The type of the layout.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WordLayout",
        signature: "WordLayout(Number: Integer, InStream: InStream): Boolean",
        summary: "Gets the Word report layout that is used on a report and returns it as a data stream.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-wordlayout-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "The ID of the report object for which you want to get the Word report layout.",
            },
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "The variable in which to return the Word report layout.",
            },
        ],
    },
];

const MISSING_REQUESTPAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes the current page.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/requestpage/requestpage-close-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SaveRecord",
        signature: "SaveRecord()",
        summary: "Saves the current record as if performed by the client.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/requestpage/requestpage-saverecord-method",
        params: PARAM_NONE,
    },
];

const MISSING_SESSION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ApplicationIdentifier",
        signature: "ApplicationIdentifier(): Text",
        summary: "Gets the application ID associated with the current thread.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-applicationidentifier-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "BindSubscription",
        signature: "BindSubscription(Codeunit: Codeunit): Boolean",
        summary: "Binds the event subscriber methods in the codeunit to the current codeunit instance for handling the events that they subscribe to.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-bindsubscription-method",
        params: &[
            BuiltinMethodParam {
                label: "Codeunit: Codeunit",
                documentation: "The codeunit that contains the event subscribers.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CurrentClientType",
        signature: "CurrentClientType(): ClientType",
        summary: "Gets the client type that is running in current session.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-currentclienttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CurrentExecutionMode",
        signature: "CurrentExecutionMode(): ExecutionMode",
        summary: "Specifies the mode in which the session is running.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-currentexecutionmode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DefaultClientType",
        signature: "DefaultClientType(): ClientType",
        summary: "Gets the default client that is configured for the server instance that is used by the current session.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-defaultclienttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "EnableVerboseTelemetry",
        signature: "EnableVerboseTelemetry(EnableFullALFunctionTracing: Boolean, Duration: Duration)",
        summary: "Temporarily enable verbose telemetry on the current session.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-enableverbosetelemetry-method",
        params: &[
            BuiltinMethodParam {
                label: "EnableFullALFunctionTracing: Boolean",
                documentation: "Specifies whether to enable method tracing.",
            },
            BuiltinMethodParam {
                label: "Duration: Duration",
                documentation: "Specifies the amount of time, in milliseconds, that verbose telemetry is enabled on the session. When the time is exceeded, system specified telemetry level is used again. The maximum value is 3600000, one hour.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetCurrentModuleExecutionContext",
        signature: "GetCurrentModuleExecutionContext(): ExecutionContext",
        summary: "Gets the current session's execution context for the currently executing module.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-getcurrentmoduleexecutioncontext-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetExecutionContext",
        signature: "GetExecutionContext(): ExecutionContext",
        summary: "Gets the current session's execution context.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-getexecutioncontext-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsSessionActive",
        signature: "IsSessionActive(SessionID: Integer): Boolean",
        summary: "Tests if the specified SessionID is active on the server instance where it was started.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-issessionactive-method",
        params: &[
            BuiltinMethodParam {
                label: "SessionID: Integer",
                documentation: "The ID of the session that you want to test if it is still active.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetDocumentServiceToken",
        signature: "SetDocumentServiceToken(Token: Text)",
        summary: "Sets the document service token in the current session.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-setdocumentservicetoken-method",
        params: &[
            BuiltinMethodParam {
                label: "Token: Text",
                documentation: "The access token.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "UnbindSubscription",
        signature: "UnbindSubscription(Codeunit: Codeunit): Boolean",
        summary: "Unbinds the event subscriber methods from in the codeunit instance.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-unbindsubscription-method",
        params: &[
            BuiltinMethodParam {
                label: "Codeunit: Codeunit",
                documentation: "The codeunit that contains the event subscribers.",
            },
        ],
    },
];

const MISSING_SESSIONINFORMATION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AITokensUsed",
        signature: "AITokensUsed(): BigInteger",
        summary: "Gets the total amount of AI tokens consumed on the session, since the session started.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/sessioninformation/sessioninformation-aitokensused-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Callstack",
        signature: "Callstack(): Text",
        summary: "Gets the current callstack.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/sessioninformation/sessioninformation-callstack-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SqlRowsRead",
        signature: "SqlRowsRead(): BigInteger",
        summary: "Gets the amount of SQL rows read on the session, since the session started.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/sessioninformation/sessioninformation-sqlrowsread-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SqlStatementsExecuted",
        signature: "SqlStatementsExecuted(): BigInteger",
        summary: "Gets the amount of SQL statements executed on the session, since the session started.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/sessioninformation/sessioninformation-sqlstatementsexecuted-method",
        params: PARAM_NONE,
    },
];

const MISSING_SYSTEM_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ApplicationPath",
        signature: "ApplicationPath(): Text",
        summary: "Returns the path of the directory where the executable file for the product is installed.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-applicationpath-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CanLoadType",
        signature: "CanLoadType(DotNet: DotNet): Boolean",
        summary: "Tests if the specified .NET Framework type can be loaded.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-canloadtype-method",
        params: &[
            BuiltinMethodParam {
                label: "DotNet: DotNet",
                documentation: "A variable of the DotNet data type to represent the .NET Framework type.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CaptionClassTranslate",
        signature: "CaptionClassTranslate(CaptionClassText: Text): Text",
        summary: "Returns a translated version of the caption string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-captionclasstranslate-method",
        params: &[
            BuiltinMethodParam {
                label: "CaptionClassText: Text",
                documentation: "A literal string that defines the caption.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear(var Variable: SecretText)",
        summary: "Clears the value of a single variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-clear-secrettext-method",
        params: &[
            BuiltinMethodParam {
                label: "Variable: SecretText",
                documentation: "The identifier (variable) of type SecretText.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ClearAll",
        signature: "ClearAll()",
        summary: "Clears all internal variables (except REC variables), keys, and filters in the object and in any associated objects, such as reports, pages, codeunits, and so on that contain AL code.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-clearall-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ClearCollectedErrors",
        signature: "ClearCollectedErrors()",
        summary: "Clears all collected errors from the current collection scope.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-clearcollectederrors-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ClearLastError",
        signature: "ClearLastError()",
        summary: "Removes the last error message from memory.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-clearlasterror-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ClosingDate",
        signature: "ClosingDate(Date: Date): Date",
        summary: "Gets the closing date for a Date Data Type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-closingdate-method",
        params: &[
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "The input date.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CodeCoverageInclude",
        signature: "CodeCoverageInclude(var ObjectRecord: Record)",
        summary: "Includes the code that has been logged.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-codecoverageinclude-method",
        params: &[
            BuiltinMethodParam {
                label: "ObjectRecord: Record",
                documentation: "Parameter documentation from Microsoft Learn.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CodeCoverageLoad",
        signature: "CodeCoverageLoad()",
        summary: "Loads the code that has been logged.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-codecoverageload-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CodeCoverageRefresh",
        signature: "CodeCoverageRefresh()",
        summary: "Refreshes the code that has been logged.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-codecoveragerefresh-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CreateDateTime",
        signature: "CreateDateTime(Date: Date, Time: Time): DateTime",
        summary: "Creates a DateTime object from a date and a time.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-createdatetime-method",
        params: &[
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "The date that you want to use to create a DateTime. You cannot use an undefined date to create a DateTime.",
            },
            BuiltinMethodParam {
                label: "Time: Time",
                documentation: "The time that you want to use to create a DateTime. You cannot use an undefined time to create a DateTime.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CreateEncryptionKey",
        signature: "CreateEncryptionKey(): Boolean",
        summary: "Creates an encryption key for the current tenant.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-createencryptionkey-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CreateGuid",
        signature: "CreateGuid(): Guid",
        summary: "Creates a new unique GUID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-createguid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Date2DMY",
        signature: "Date2DMY(Date: Date, Value: Integer): Integer",
        summary: "Gets the day, month, or year of a Date Data Type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-date2dmy-method",
        params: &[
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "The input date.",
            },
            BuiltinMethodParam {
                label: "Value: Integer",
                documentation: "Specifies what the method should return. The valid options are 1, 2, and 3. - The value 1 corresponds to Day (1-31). - The value 2 corresponds to Month (1-12). - The value 3 corresponds to Year.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Date2DWY",
        signature: "Date2DWY(Date: Date, Value: Integer): Integer",
        summary: "Gets the day of the week, week number, or year of a Date Data Type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-date2dwy-method",
        params: &[
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "The input date.",
            },
            BuiltinMethodParam {
                label: "Value: Integer",
                documentation: "Specifies what the function returns. The valid options are 1, 2, and 3. - The value 1 corresponds to day of the week (1-7, Monday = 1). - The value 2 corresponds to week number (1-53). - The value 3 corresponds to year.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DaTi2Variant",
        signature: "DaTi2Variant(Date: Date, Time: Time): Variant",
        summary: "Creates a variant that contains an encapsulation of a COM VT\\\\_DATE.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-dati2variant-method",
        params: &[
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "The input date.",
            },
            BuiltinMethodParam {
                label: "Time: Time",
                documentation: "The input time.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Decrypt",
        signature: "Decrypt(EncryptedString: Text): Text",
        summary: "Takes a string as input and returns the decrypted value of the string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-decrypt-method",
        params: &[
            BuiltinMethodParam {
                label: "EncryptedString: Text",
                documentation: "The input string that will be decrypted.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DeleteEncryptionKey",
        signature: "DeleteEncryptionKey()",
        summary: "Deletes an encryption key for the current tenant.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-deleteencryptionkey-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DT2Date",
        signature: "DT2Date(Datetime: DateTime): Date",
        summary: "Gets the date part of a DateTime object.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-dt2date-method",
        params: &[
            BuiltinMethodParam {
                label: "Datetime: DateTime",
                documentation: "The DateTime of which to return the date part.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DT2Time",
        signature: "DT2Time(Datetime: DateTime): Time",
        summary: "Gets the time part of a DateTime object.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-dt2time-method",
        params: &[
            BuiltinMethodParam {
                label: "Datetime: DateTime",
                documentation: "The DateTime of which to return the time part.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Encrypt",
        signature: "Encrypt(PlainTextString: Text): Text",
        summary: "Takes a string as input and returns the encrypted value of the string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-encrypt-method",
        params: &[
            BuiltinMethodParam {
                label: "PlainTextString: Text",
                documentation: "The input string that will be encrypted. The input string cannot exceed a length of 215 plain characters. If the input string includes special characters the length is even more reduced.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "EncryptionEnabled",
        signature: "EncryptionEnabled(): Boolean",
        summary: "Checks if the tenant is configured to allow encryption.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-encryptionenabled-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "EncryptionKeyExists",
        signature: "EncryptionKeyExists(): Boolean",
        summary: "Checks whether an encryption key for the current tenant is present on the server tenant.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-encryptionkeyexists-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ExportEncryptionKey",
        signature: "ExportEncryptionKey(Password: Text): Text",
        summary: "Returns a password protected temporary filepath containing the encryption key.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-exportencryptionkey-method",
        params: &[
            BuiltinMethodParam {
                label: "Password: Text",
                documentation: "Specifies the password for the encryption key file.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocumentUrl",
        signature: "GetDocumentUrl(ID: Guid): Text",
        summary: "Gets the URL for the specified temporary media object ID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-getdocumenturl-method",
        params: &[
            BuiltinMethodParam {
                label: "ID: Guid",
                documentation: "The temporary media object ID.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDotNetType",
        signature: "GetDotNetType(Expression: Any): DotNet",
        summary: "Gets the System.Type that corresponds to the given value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-getdotnettype-method",
        params: &[
            BuiltinMethodParam {
                label: "Expression: Any",
                documentation: "The value for which to retrieve the System.Type.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetLastErrorCallStack",
        signature: "GetLastErrorCallStack(): Text",
        summary: "Gets the call stack from where the last error occurred.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-getlasterrorcallstack-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetLastErrorCode",
        signature: "GetLastErrorCode(): Text",
        summary: "Gets the classification of the last error that occurred.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-getlasterrorcode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetLastErrorObject",
        signature: "GetLastErrorObject(): DotNet",
        summary: "Gets the last System.Exception object that occurred.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-getlasterrorobject-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetLastErrorText",
        signature: "GetLastErrorText(): Text",
        summary: "Gets the last error that occurred in the debugger.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-getlasterrortext--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GuiAllowed",
        signature: "GuiAllowed(): Boolean",
        summary: "Checks whether the AL code can show any information on the screen.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-guiallowed-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasCollectedErrors",
        signature: "HasCollectedErrors(): Boolean",
        summary: "Gets a value indicating whether errors have been collected in the current error collection scope.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-hascollectederrors-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Hyperlink",
        signature: "Hyperlink(URL: Text)",
        summary: "Passes a URL as an argument to an Internet browser, such as Microsoft Edge.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-hyperlink-method",
        params: &[
            BuiltinMethodParam {
                label: "URL: Text",
                documentation: "A URL that is passed to the Internet browser as an argument.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ImportEncryptionKey",
        signature: "ImportEncryptionKey(Path: Text, Password: Text): Boolean",
        summary: "Points to a password protected file that contains the key on the current server.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-importencryptionkey-method",
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "Specifies the file that contains the encryption key.",
            },
            BuiltinMethodParam {
                label: "Password: Text",
                documentation: "Specifies the password the protects the file.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsCollectingErrors",
        signature: "IsCollectingErrors(): Boolean",
        summary: "Gets a value indicating whether errors are currently being collected.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-iscollectingerrors-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsNull",
        signature: "IsNull(DotNet: DotNet): Boolean",
        summary: "Gets a value indicating whether a DotNet object has been created or not.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-isnull-method",
        params: &[
            BuiltinMethodParam {
                label: "DotNet: DotNet",
                documentation: "A DotNet expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsNullGuid",
        signature: "IsNullGuid(Guid: Guid): Boolean",
        summary: "Indicates whether a value has been assigned to a GUID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-isnullguid-method",
        params: &[
            BuiltinMethodParam {
                label: "Guid: Guid",
                documentation: "The GUID that you want to check whether it is null.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsServiceTier",
        signature: "IsServiceTier(): Boolean",
        summary: "Gets a value indicating whether the runtime is a service tier.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-isservicetier-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "NormalDate",
        signature: "NormalDate(Date: Date): Date",
        summary: "Gets the regular date (instead of the closing date) for the argument Date.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-normaldate-method",
        params: &[
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "The input date. You can enter a closing date or a normal date. A run-time error occurs if the value of Date is set to the undefined date (0D).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Power",
        signature: "Power(Number: Decimal, Power: Decimal): Decimal",
        summary: "Raises a number to a power.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-power-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Decimal",
                documentation: "The number you want to raise exponentially. This number is the base in the exponential method.",
            },
            BuiltinMethodParam {
                label: "Power: Decimal",
                documentation: "The exponent in the exponential method.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Random",
        signature: "Random(MaxNumber: Integer): Integer",
        summary: "Returns a pseudo-random number.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-random-method",
        params: &[
            BuiltinMethodParam {
                label: "MaxNumber: Integer",
                documentation: "The largest acceptable number. In effect, you are setting a range from one (1) to the number that you specify with the MaxNumber parameter.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Sleep",
        signature: "Sleep(Duration: Integer)",
        summary: "Returns control to the operating system for a specified time.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-sleep-method",
        params: &[
            BuiltinMethodParam {
                label: "Duration: Integer",
                documentation: "The number of milliseconds to return control to the operating system.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "TemporaryPath",
        signature: "TemporaryPath(): Text",
        summary: "Gets the path of the directory where the temporary file is stored.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-temporarypath-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Time",
        signature: "Time(): Time",
        summary: "Gets the current time from the operating system.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-time-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Today",
        signature: "Today(): Date",
        summary: "Gets the current date set in the operating system.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-today-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Variant2Date",
        signature: "Variant2Date(Variant: Variant): Date",
        summary: "Gets a date from a variant.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-variant2date-method",
        params: &[
            BuiltinMethodParam {
                label: "Variant: Variant",
                documentation: "The input variant.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Variant2Time",
        signature: "Variant2Time(Variant: Variant): Time",
        summary: "Gets a time from a variant.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-variant2time-method",
        params: &[
            BuiltinMethodParam {
                label: "Variant: Variant",
                documentation: "The input variant.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WindowsLanguage",
        signature: "WindowsLanguage(): Integer",
        summary: "Gets the current Windows language setting.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-windowslanguage-method",
        params: PARAM_NONE,
    },
];

const MISSING_TESTFIELD_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Drilldown",
        signature: "Drilldown()",
        summary: "Applies drill-down capability for a field on a test page.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testfield/testfield-drilldown-method",
        params: PARAM_NONE,
    },
];

const MISSING_TESTHTTPREQUESTMESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "HasSecretUri",
        signature: "HasSecretUri(): Boolean",
        summary: "**true** if the request has a secret URI set, otherwise **false**.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testhttprequestmessage/testhttprequestmessage-hassecreturi-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "QueryParameters",
        signature: "QueryParameters(): Dictionary of [Text, Text",
        summary: "Gets the query parameters of the HTTP request if the request does not have a secret URI, otherwise an empty Dictionary.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testhttprequestmessage/testhttprequestmessage-queryparameters-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RequestType",
        signature: "RequestType(): HttpRequestType",
        summary: "Gets the HTTP method type.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testhttprequestmessage/testhttprequestmessage-requesttype-method",
        params: PARAM_NONE,
    },
];

const MISSING_TEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ConvertStr",
        signature: "ConvertStr(String: Text, FromCharacters: Text, ToCharacters: Text): Text",
        summary: "Replaces all chars in source found in FromCharacters with the corresponding char in ToCharacters and returns the converted string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-convertstr-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string that you want to convert.",
            },
            BuiltinMethodParam {
                label: "FromCharacters: Text",
                documentation: "The characters that you want to replace. This function is case-sensitive.",
            },
            BuiltinMethodParam {
                label: "ToCharacters: Text",
                documentation: "The new characters with which you want to replace the FromCharacters. This function is case-sensitive.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IncStr",
        signature: "IncStr(String: Text): Text",
        summary: "Increases the last positive number or decreases the last negative number inside a string by one (1).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-incstr-string-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string that you want to increase or decrease.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "InsStr",
        signature: "InsStr(String: Text, SubString: Text, Position: Integer): Text",
        summary: "Inserts a substring into a string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-insstr-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string into which you want to insert a substring.",
            },
            BuiltinMethodParam {
                label: "SubString: Text",
                documentation: "The substring that you want to insert into String.",
            },
            BuiltinMethodParam {
                label: "Position: Integer",
                documentation: "Specifies where to insert SubString. Position must be greater than or equal to 1. If Position is greater than the length of String, then the result is concatenated and copied to NewString.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "LowerCase",
        signature: "LowerCase(String: Text): Text",
        summary: "Converts all letters in a string to lowercase.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-lowercase-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string that you want to convert to lowercase. Only letters in the range A to Z and, if applicable, special language characters are converted.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectStr",
        signature: "SelectStr(Number: Integer, CommaString: Text): Text",
        summary: "Retrieves a substring from a comma-separated string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-selectstr-method",
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "Specifies which substring to retrieve. The substrings in the comma-separated string are numbered 1, 2, 3, and so on. If Number is greater than the actual number of substrings, then a run-time error occurs.",
            },
            BuiltinMethodParam {
                label: "CommaString: Text",
                documentation: "A string that contains substrings separated by commas. The maximum length of this string is 391 characters.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "StrLen",
        signature: "StrLen(String: Text): Integer",
        summary: "Gets the length of a string you define.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-strlen-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string for which you want to determine the length.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "StrPos",
        signature: "StrPos(String: Text, SubString: Text): Integer",
        summary: "Searches for the first occurrence of substring inside a string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-strpos-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string in which you want to search.",
            },
            BuiltinMethodParam {
                label: "SubString: Text",
                documentation: "The substring for which you want to search.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "UpperCase",
        signature: "UpperCase(String: Text): Text",
        summary: "Converts all letters in a string to uppercase.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-uppercase-method",
        params: &[
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "The string that you want to convert to uppercase.",
            },
        ],
    },
];

const MISSING_TEXTBUILDER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "EnsureCapacity",
        signature: "EnsureCapacity(NewCapacity: Integer): Boolean",
        summary: "Ensures that the capacity of this TextBuilder instance is at least the specified value.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textbuilder/textbuilder-ensurecapacity-method",
        params: &[
            BuiltinMethodParam {
                label: "NewCapacity: Integer",
                documentation: "The minimum capacity to ensure.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(Position: Integer, Text: Text): Boolean",
        summary: "Inserts a string into this TextBuilder instance at the specified character position.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textbuilder/textbuilder-insert-method",
        params: &[
            BuiltinMethodParam {
                label: "Position: Integer",
                documentation: "The position in this TextBuilder instance where insertion begins.",
            },
            BuiltinMethodParam {
                label: "Text: Text",
                documentation: "The string to insert.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "MaxCapacity",
        signature: "MaxCapacity(): Integer",
        summary: "Gets the maximum capacity of this TextBuilder instance.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textbuilder/textbuilder-maxcapacity-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(StartIndex: Integer, Count: Integer): Boolean",
        summary: "Removes the specified range of characters from this TextBuilder instance.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textbuilder/textbuilder-remove-method",
        params: &[
            BuiltinMethodParam {
                label: "StartIndex: Integer",
                documentation: "The one-based position in this TextBuilder instance where removal begins.",
            },
            BuiltinMethodParam {
                label: "Count: Integer",
                documentation: "The number of characters to remove.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Replace",
        signature: "Replace(OldText: Text, NewText: Text): Boolean",
        summary: "Replaces all occurrences of a specified string in this TextBuilder instance with another specified string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textbuilder/textbuilder-replace-text-text-method",
        params: &[
            BuiltinMethodParam {
                label: "OldText: Text",
                documentation: "The string to replace.",
            },
            BuiltinMethodParam {
                label: "NewText: Text",
                documentation: "The string that replaces OldText.",
            },
        ],
    },
];

const MISSING_TEXTCONST_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Value: Text): Boolean",
        summary: "Returns a value indicating whether a specified substring occurs within this string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-contains-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The string to seek.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "EndsWith",
        signature: "EndsWith(Value: Text): Boolean",
        summary: "Determines whether the end of this string instance matches the specified string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-endswith-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The string to compare to the substring at the end of this instance.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Replace",
        signature: "Replace(OldValue: Text, NewValue: Text): Text",
        summary: "Returns a new Text in which all occurrences of a specified string in the current instance are replaced with another specified string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-replace-method",
        params: &[
            BuiltinMethodParam {
                label: "OldValue: Text",
                documentation: "The string to be replaced.",
            },
            BuiltinMethodParam {
                label: "NewValue: Text",
                documentation: "The string to replace all occurrences of OldValue.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "StartsWith",
        signature: "StartsWith(Value: Text): Boolean",
        summary: "Determines whether the beginning of this instance matches a specified string.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-startswith-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The string to compare.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ToLower",
        signature: "ToLower(): Text",
        summary: "Returns a copy of this string converted to lowercase.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-tolower-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ToUpper",
        signature: "ToUpper(): Text",
        summary: "Returns a copy of this string converted to uppercase.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-toupper-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Trim",
        signature: "Trim(): Text",
        summary: "Returns a new Text in which all leading and trailing white-space characters from the current Text object are removed.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textconst/textconst-trim-method",
        params: PARAM_NONE,
    },
];

const MISSING_VARIANT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "IsAction",
        signature: "IsAction(): Boolean",
        summary: "Indicates whether an AL variant contains an Action variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isaction-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsAutomation",
        signature: "IsAutomation(): Boolean",
        summary: "Indicates whether an AL variant contains an Automation variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isautomation-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsBigInteger",
        signature: "IsBigInteger(): Boolean",
        summary: "Indicates whether an AL variant contains a BigInteger variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isbiginteger-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsBinary",
        signature: "IsBinary(): Boolean",
        summary: "Indicates whether an AL variant contains a Binary variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isbinary-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsByte",
        signature: "IsByte(): Boolean",
        summary: "Indicates whether an AL variant contains a Byte data type variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isbyte-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsChar",
        signature: "IsChar(): Boolean",
        summary: "Indicates whether an AL variant contains a Char variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-ischar-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsClientType",
        signature: "IsClientType(): Boolean",
        summary: "Indicates whether an AL variant contains a ClientType variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isclienttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsCodeunit",
        signature: "IsCodeunit(): Boolean",
        summary: "Indicates whether an AL variant contains a Codeunit variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-iscodeunit-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDataClassification",
        signature: "IsDataClassification(): Boolean",
        summary: "Indicates whether an AL variant contains a DataClassification variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isdataclassification-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDataClassificationType",
        signature: "IsDataClassificationType(): Boolean",
        summary: "Indicates whether an AL variant contains a DataClassification variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isdataclassificationtype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDateFormula",
        signature: "IsDateFormula(): Boolean",
        summary: "Indicates whether an AL variant contains a DateFormula variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isdateformula-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDefaultLayout",
        signature: "IsDefaultLayout(): Boolean",
        summary: "Indicates whether an AL variant contains a DefaultLayout variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isdefaultlayout-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDotNet",
        signature: "IsDotNet(): Boolean",
        summary: "Indicates whether an AL variant contains a DotNet variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isdotnet-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsExecutionMode",
        signature: "IsExecutionMode(): Boolean",
        summary: "Indicates whether an AL variant contains an ExecutionMode variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isexecutionmode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsFieldRef",
        signature: "IsFieldRef(): Boolean",
        summary: "Indicates whether an AL variant contains a FieldRef variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isfieldref-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsFile",
        signature: "IsFile(): Boolean",
        summary: "Indicates whether an AL variant contains a File variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isfile-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsFilterPageBuilder",
        signature: "IsFilterPageBuilder(): Boolean",
        summary: "Indicates whether an AL variant contains a FilterPageBuilder variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isfilterpagebuilder-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsInStream",
        signature: "IsInStream(): Boolean",
        summary: "Indicates whether an AL variant contains an InStream variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isinstream-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsNotification",
        signature: "IsNotification(): Boolean",
        summary: "Indicates whether an AL variant contains a Notification variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isnotification-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsObjectType",
        signature: "IsObjectType(): Boolean",
        summary: "Indicates whether an AL variant contains an ObjectType variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isobjecttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsOutStream",
        signature: "IsOutStream(): Boolean",
        summary: "Indicates whether an AL variant contains an OutStream variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isoutstream-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsPromptMode",
        signature: "IsPromptMode(): Boolean",
        summary: "Indicates whether an AL variant contains a PromptMode variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-ispromptmode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsReportFormat",
        signature: "IsReportFormat(): Boolean",
        summary: "Indicates whether an AL variant contains a ReportFormat variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isreportformat-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsSecurityFiltering",
        signature: "IsSecurityFiltering(): Boolean",
        summary: "Indicates whether an AL variant contains a SecurityFiltering variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-issecurityfiltering-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTableConnectionType",
        signature: "IsTableConnectionType(): Boolean",
        summary: "Indicates whether an AL variant contains a TableConnectionType variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-istableconnectiontype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTestPermissions",
        signature: "IsTestPermissions(): Boolean",
        summary: "Indicates whether an AL variant contains a TestPermissions variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-istestpermissions-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTextBuilder",
        signature: "IsTextBuilder(): Boolean",
        summary: "Indicates whether an AL variant contains a TextBuilder variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-istextbuilder-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTextConstant",
        signature: "IsTextConstant(): Boolean",
        summary: "Indicates whether an AL variant contains a Text constant.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-istextconstant-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTextEncoding",
        signature: "IsTextEncoding(): Boolean",
        summary: "Indicates whether an AL variant contains a TextEncoding variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-istextencoding-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTransactionType",
        signature: "IsTransactionType(): Boolean",
        summary: "Indicates whether an AL variant contains a TransactionType variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-istransactiontype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsWideChar",
        signature: "IsWideChar(): Boolean",
        summary: "Indicates whether an AL variant contains a WideChar variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-iswidechar-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlAttribute",
        signature: "IsXmlAttribute(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlAttribute variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlattribute-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlAttributeCollection",
        signature: "IsXmlAttributeCollection(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlAttributeCollection variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlattributecollection-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlCData",
        signature: "IsXmlCData(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlCData variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlcdata-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlComment",
        signature: "IsXmlComment(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlComment variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlcomment-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlDeclaration",
        signature: "IsXmlDeclaration(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlDeclaration variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmldeclaration-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlDocument",
        signature: "IsXmlDocument(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlDocument variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmldocument-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlDocumentType",
        signature: "IsXmlDocumentType(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlDocumentType variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmldocumenttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlElement",
        signature: "IsXmlElement(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlElement variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlelement-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlNamespaceManager",
        signature: "IsXmlNamespaceManager(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlNamespaceManager variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlnamespacemanager-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlNameTable",
        signature: "IsXmlNameTable(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlNameTable variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlnametable-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlNode",
        signature: "IsXmlNode(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlNode variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlnode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlNodeList",
        signature: "IsXmlNodeList(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlNodeList variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlnodelist-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlProcessingInstruction",
        signature: "IsXmlProcessingInstruction(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlProcessingInstruction variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlprocessinginstruction-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlReadOptions",
        signature: "IsXmlReadOptions(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlReadOptions variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlreadoptions-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlText",
        signature: "IsXmlText(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlText variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmltext-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlWriteOptions",
        signature: "IsXmlWriteOptions(): Boolean",
        summary: "Indicates whether an AL variant contains an XmlWriteOptions variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-isxmlwriteoptions-method",
        params: PARAM_NONE,
    },
];

const MISSING_WEBSERVICEACTIONCONTEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "GetObjectId",
        signature: "GetObjectId(): Integer",
        summary: "Gets the object ID.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/webserviceactioncontext/webserviceactioncontext-getobjectid-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetResultCode",
        signature: "GetResultCode(): WebServiceActionResultCode",
        summary: "Gets the web service action result status code.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/webserviceactioncontext/webserviceactioncontext-getresultcode-method",
        params: PARAM_NONE,
    },
];

const MISSING_XMLATTRIBUTE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Converts the node to an XmlNode.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-asxmlnode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Name: Text, Value: Text): XmlAttribute",
        summary: "Creates an XmlAttribute node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-create-string-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The qualified name of the attribute. If the name is of the form {{namespace}}localName, it will be qualified with the given namespace.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The value of the attribute.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CreateNamespaceDeclaration",
        signature: "CreateNamespaceDeclaration(Prefix: Text, NamespaceUri: Text): XmlAttribute",
        summary: "Creates an attribute that represents a namespace declaration.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-createnamespacedeclaration-method",
        params: &[
            BuiltinMethodParam {
                label: "Prefix: Text",
                documentation: "The prefix of the attribute (if any).",
            },
            BuiltinMethodParam {
                label: "NamespaceUri: Text",
                documentation: "The URI of the attribute. If the prefix is xmlns, then this parameter must be http://www.w3.org/2000/xmlns/; otherwise an exception is thrown.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsNamespaceDeclaration",
        signature: "IsNamespaceDeclaration(): Boolean",
        summary: "Determines if this attribute is a namespace declaration.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-isnamespacedeclaration-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LocalName",
        signature: "LocalName(): Text",
        summary: "Gets the local name of the attribute.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-localname-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "NamespacePrefix",
        signature: "NamespacePrefix(): Text",
        summary: "Gets the prefix of the attribute (if any).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-namespaceprefix-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "NamespaceUri",
        signature: "NamespaceUri(): Text",
        summary: "Gets the namespace URI of the attribute.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-namespaceuri-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLATTRIBUTECOLLECTION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "RemoveAll",
        signature: "RemoveAll()",
        summary: "Removes all attributes from the collection.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattributecollection/xmlattributecollection-removeall-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Set",
        signature: "Set(Name: Text, Value: Text)",
        summary: "Sets the value of the specified attribute or creates it if is not part of the collection.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattributecollection/xmlattributecollection-set-string-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The fully qualified name of the attribute to set.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The value to set for the attribute.",
            },
        ],
    },
];

const MISSING_XMLCDATA_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLCOMMENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLDECLARATION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLDOCUMENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Content: Any, ...): Boolean",
        summary: "Adds the specified content as a child of this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-add-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to be added as a child of this document.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddFirst",
        signature: "AddFirst(Content: Any, ...): Boolean",
        summary: "Adds the specified content at the start of the child list of this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-addfirst-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to be added as a child of this document.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Converts the node to an XmlNode.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-asxmlnode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetChildElements",
        signature: "GetChildElements(): XmlNodeList",
        summary: "Gets a list containing the child elements for this document, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getchildelements--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetChildNodes",
        signature: "GetChildNodes(): XmlNodeList",
        summary: "Gets a list containing the child elements for this document, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getchildnodes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDeclaration",
        signature: "GetDeclaration(var Result: XmlDeclaration): Boolean",
        summary: "Gets the XML declaration for this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getdeclaration-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: XmlDeclaration",
                documentation: "The XML declaration for this document.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDescendantElements",
        signature: "GetDescendantElements(): XmlNodeList",
        summary: "Gets a list containing the descendant elements for this document, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getdescendantelements--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDescendantNodes",
        signature: "GetDescendantNodes(): XmlNodeList",
        summary: "Gets a list containing the descendant nodes for this document, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getdescendantnodes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocumentType",
        signature: "GetDocumentType(var DocumentType: XmlDocumentType): Boolean",
        summary: "Gets the Document Type Definition (DTD) for this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getdocumenttype-method",
        params: &[
            BuiltinMethodParam {
                label: "DocumentType: XmlDocumentType",
                documentation: "The Document Type Definition (DTD) for this document.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "NameTable",
        signature: "NameTable(): XmlNameTable",
        summary: "Gets the XmlNameTable associated with this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-nametable-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RemoveNodes",
        signature: "RemoveNodes()",
        summary: "Removes the child nodes from this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-removenodes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceNodes",
        signature: "ReplaceNodes(Content: Any, ...): Boolean",
        summary: "Replaces the children nodes of this document with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-replacenodes-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content that replaces the children nodes.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetDeclaration",
        signature: "SetDeclaration(Declaration: XmlDeclaration): Boolean",
        summary: "Sets the XML declaration for this document.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-setdeclaration-method",
        params: &[
            BuiltinMethodParam {
                label: "Declaration: XmlDeclaration",
                documentation: "The new value of the XML declaration of this document.",
            },
        ],
    },
];

const MISSING_XMLDOCUMENTTYPE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetInternalSubset",
        signature: "GetInternalSubset(var Result: Text): Boolean",
        summary: "Gets the internal subset for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-getinternalsubset-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "A string that contains the internal subset for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetName",
        signature: "GetName(var Result: Text): Boolean",
        summary: "Gets the name for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-getname-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "A string that contains the name for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetPublicId",
        signature: "GetPublicId(var Result: Text): Boolean",
        summary: "Gets the public identifier for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-getpublicid-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "A string that contains the public identifier for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetSystemId",
        signature: "GetSystemId(var Result: Text): Boolean",
        summary: "Gets the system identifier for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-getsystemid-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "A string that contains the system identifier for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetInternalSubset",
        signature: "SetInternalSubset(Value: Text): Boolean",
        summary: "Sets the internal subset for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-setinternalsubset-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "A string that contains the new internal subset for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetName",
        signature: "SetName(Value: Text): Boolean",
        summary: "Sets the name for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-setname-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "A string that contains the new name for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetPublicId",
        signature: "SetPublicId(Value: Text): Boolean",
        summary: "Sets the public identifier for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-setpublicid-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "A string that contains the new public identifier for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetSystemId",
        signature: "SetSystemId(Value: Text): Boolean",
        summary: "Sets the system identifier for this Document Type Definition (DTD).",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-setsystemid-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "A string that contains the new system identifier for this Document Type Definition (DTD).",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLELEMENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Content: Any, ...): Boolean",
        summary: "Adds the specified content as a child of this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-add-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to be added as a child of this element.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddFirst",
        signature: "AddFirst(Content: Any, ...): Boolean",
        summary: "Adds the specified content at the start of the child list of this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-addfirst-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to be added as a child of this element.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Converts the node to an XmlNode.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-asxmlnode-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Attributes",
        signature: "Attributes(): XmlAttributeCollection",
        summary: "Gets a collection of the attributes of this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-attributes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Name: Text): XmlElement",
        summary: "Creates an XmlElement node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-create-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The fully qualified name of the element to create.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetChildElements",
        signature: "GetChildElements(): XmlNodeList",
        summary: "Gets a list containing the child elements for this element, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getchildelements--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetChildNodes",
        signature: "GetChildNodes(): XmlNodeList",
        summary: "Gets a list containing the child elements for this element, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getchildnodes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDescendantElements",
        signature: "GetDescendantElements(): XmlNodeList",
        summary: "Gets a list containing the descendant elements for this element, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getdescendantelements--method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDescendantNodes",
        signature: "GetDescendantNodes(): XmlNodeList",
        summary: "Gets a list containing the descendant nodes for this element, in document order.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getdescendantnodes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetNamespaceOfPrefix",
        signature: "GetNamespaceOfPrefix(Prefix: Text, var Result: Text): Boolean",
        summary: "Gets the namespace associated with a particular prefix for this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getnamespaceofprefix-method",
        params: &[
            BuiltinMethodParam {
                label: "Prefix: Text",
                documentation: "A string that contains the namespace prefix to look up.",
            },
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "The namespace URI associated with the given prefix for this element.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetPrefixOfNamespace",
        signature: "GetPrefixOfNamespace(Namespace: Text, var Result: Text): Boolean",
        summary: "Gets the prefix associated with a namespace URI for this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-getprefixofnamespace-method",
        params: &[
            BuiltinMethodParam {
                label: "Namespace: Text",
                documentation: "A namespace URI to look up.",
            },
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "A string that contains the namespace prefix.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "HasAttributes",
        signature: "HasAttributes(): Boolean",
        summary: "Gets a boolean value indicating whether this element has at least one attribute.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-hasattributes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasElements",
        signature: "HasElements(): Boolean",
        summary: "Gets a value indicating whether this element has at least one child element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-haselements-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsEmpty",
        signature: "IsEmpty(): Boolean",
        summary: "Gets a value indicating whether this element contains no content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-isempty-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LocalName",
        signature: "LocalName(): Text",
        summary: "Gets the local name of this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-localname-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "NamespaceUri",
        signature: "NamespaceUri(): Text",
        summary: "Gets the namespace URI of this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-namespaceuri-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RemoveAllAttributes",
        signature: "RemoveAllAttributes()",
        summary: "Removes the attributes of this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-removeallattributes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RemoveAttribute",
        signature: "RemoveAttribute(Name: Text)",
        summary: "Removes the specified attribute from this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-removeattribute-string-method",
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "The fully qualified name of the attribute to remove.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "RemoveNodes",
        signature: "RemoveNodes()",
        summary: "Removes the child nodes from this element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-removenodes-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceNodes",
        signature: "ReplaceNodes(Content: Any, ...): Boolean",
        summary: "Replaces the children nodes of this element with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-replacenodes-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content that replaces the children nodes.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLNODE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlCData",
        signature: "AsXmlCData(): XmlCData",
        summary: "Converts the node to an XmlCData node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmlcdata-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlComment",
        signature: "AsXmlComment(): XmlComment",
        summary: "Converts the node to an XmlComment node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmlcomment-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlDeclaration",
        signature: "AsXmlDeclaration(): XmlDeclaration",
        summary: "Converts the node to an XmlDeclaration node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmldeclaration-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlDocument",
        signature: "AsXmlDocument(): XmlDocument",
        summary: "Converts the node to an XmlDocument node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmldocument-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlDocumentType",
        signature: "AsXmlDocumentType(): XmlDocumentType",
        summary: "Converts the node to an XmlDocumentType node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmldocumenttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlProcessingInstruction",
        signature: "AsXmlProcessingInstruction(): XmlProcessingInstruction",
        summary: "Converts the node to an XmlProcessingInstruction node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmlprocessinginstruction-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlText",
        signature: "AsXmlText(): XmlText",
        summary: "Converts the node to an XmlText node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-asxmltext-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsXmlAttribute",
        signature: "IsXmlAttribute(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlAttribute.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmlattribute-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlCData",
        signature: "IsXmlCData(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlCData.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmlcdata-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlComment",
        signature: "IsXmlComment(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlComment.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmlcomment-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlDeclaration",
        signature: "IsXmlDeclaration(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlDeclaration.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmldeclaration-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlDocument",
        signature: "IsXmlDocument(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlDocument.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmldocument-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlDocumentType",
        signature: "IsXmlDocumentType(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlDocumentType.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmldocumenttype-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlElement",
        signature: "IsXmlElement(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlElement.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmlelement-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlProcessingInstruction",
        signature: "IsXmlProcessingInstruction(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlProcessingInstruction.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmlprocessinginstruction-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsXmlText",
        signature: "IsXmlText(): Boolean",
        summary: "Gets a value indicating whether this node is an XmlText.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-isxmltext-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLPORT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Break",
        signature: "Break()",
        summary: "Exits from a loop or a trigger in a data item trigger of a report or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlportinstance-break-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "BreakUnbound",
        signature: "BreakUnbound()",
        summary: "Exits from a loop on records in an XmlPort trigger.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlportinstance-breakunbound-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CurrentPath",
        signature: "CurrentPath(): Text",
        summary: "Returns the CurrentPath for a given node, used when exporting an XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlportinstance-currentpath-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Quit",
        signature: "Quit()",
        summary: "Aborts the processing of a report or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlportinstance-quit-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetTableView",
        signature: "SetTableView(var Record: Record)",
        summary: "Applies the table view on the current record as the table view for the page, report, or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlportinstance-settableview-method",
        params: &[
            BuiltinMethodParam {
                label: "Record: Record",
                documentation: "The record that has a table view that you want to apply to the page or data item.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Skip",
        signature: "Skip()",
        summary: "Skips the current iteration of the current report or XmlPort.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlportinstance-skip-method",
        params: PARAM_NONE,
    },
];

const MISSING_XMLPROCESSINGINSTRUCTION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetData",
        signature: "GetData(var Result: Text): Boolean",
        summary: "Gets the content of the processing instruction, excluding the target.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-getdata-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "The content of the processing instruction, excluding the target.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetTarget",
        signature: "GetTarget(var Result: Text): Boolean",
        summary: "Gets the target of the processing instruction.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-gettarget-method",
        params: &[
            BuiltinMethodParam {
                label: "Result: Text",
                documentation: "The target of the processing instruction.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetData",
        signature: "SetData(Value: Text): Boolean",
        summary: "Sets the content of the processing instruction, excluding the target.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-setdata-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The new content of the processing instruction, excluding the target.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetTarget",
        signature: "SetTarget(Value: Text): Boolean",
        summary: "Sets the target of the processing instruction.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-settarget-method",
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "The new target of the processing instruction.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

const MISSING_XMLTEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddAfterSelf",
        signature: "AddAfterSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately after this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-addafterself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add after this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddBeforeSelf",
        signature: "AddBeforeSelf(Content: Any, ...): Boolean",
        summary: "Adds the specified content immediately before this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-addbeforeself-method",
        params: &[
            BuiltinMethodParam {
                label: "Content: Any",
                documentation: "The content to add before this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetDocument",
        signature: "GetDocument(var Document: XmlDocument): Boolean",
        summary: "Gets the XmlDocument for this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-getdocument-method",
        params: &[
            BuiltinMethodParam {
                label: "Document: XmlDocument",
                documentation: "The owning XmlDocument of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetParent",
        signature: "GetParent(var Parent: XmlElement): Boolean",
        summary: "Gets the parent XmlElement of this node.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-getparent-method",
        params: &[
            BuiltinMethodParam {
                label: "Parent: XmlElement",
                documentation: "The parent XmlElement of this node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(): Boolean",
        summary: "Removes this node from its parent element.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-remove-method",
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReplaceWith",
        signature: "ReplaceWith(Node: Any, ...): Boolean",
        summary: "Replaces this node with the specified content.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-replacewith-method",
        params: &[
            BuiltinMethodParam {
                label: "Node: Any",
                documentation: "The content with which to replace the current node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var NodeList: XmlNodeList): Boolean",
        summary: "Selects a list of nodes matching the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-selectnodes-string-xmlnodelist-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "NodeList: XmlNodeList",
                documentation: "An XmlNodeList containing a collection of nodes matching the XPath expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Node: XmlNode): Boolean",
        summary: "Selects the first XmlNode that matches the XPath expression.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-selectsinglenode-string-xmlnode-method",
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "The XPath expression.",
            },
            BuiltinMethodParam {
                label: "Node: XmlNode",
                documentation: "The first XmlNode that matches the XPath query.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(OutStream: OutStream): Boolean",
        summary: "Serializes and saves the current node to the given variable.",
        docs_url: "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-writeto-outstream-method",
        params: &[
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "The OutStream to which you want to save the serialized representation of the node.",
            },
        ],
    },
];

pub fn missing_methods_for_object_kind(object_kind: &str) -> &'static [BuiltinMethodDoc] {
    match object_kind {
        "bigtext" => MISSING_BIGTEXT_METHODS,
        "blob" => MISSING_BLOB_METHODS,
        "companyproperty" => MISSING_COMPANYPROPERTY_METHODS,
        "database" => MISSING_DATABASE_METHODS,
        "datatransfer" => MISSING_DATATRANSFER_METHODS,
        "debugger" => MISSING_DEBUGGER_METHODS,
        "dialog" => MISSING_DIALOG_METHODS,
        "enum" => MISSING_ENUM_METHODS,
        "errorinfo" => MISSING_ERRORINFO_METHODS,
        "fieldref" => MISSING_FIELDREF_METHODS,
        "file" => MISSING_FILE_METHODS,
        "filterpagebuilder" => MISSING_FILTERPAGEBUILDER_METHODS,
        "guid" => MISSING_GUID_METHODS,
        "httpclient" => MISSING_HTTPCLIENT_METHODS,
        "httpcontent" => MISSING_HTTPCONTENT_METHODS,
        "httpheaders" => MISSING_HTTPHEADERS_METHODS,
        "httprequestmessage" => MISSING_HTTPREQUESTMESSAGE_METHODS,
        "httpresponsemessage" => MISSING_HTTPRESPONSEMESSAGE_METHODS,
        "jsonarray" => MISSING_JSONARRAY_METHODS,
        "jsonobject" => MISSING_JSONOBJECT_METHODS,
        "jsonvalue" => MISSING_JSONVALUE_METHODS,
        "keyref" => MISSING_KEYREF_METHODS,
        "list" => MISSING_LIST_METHODS,
        "media" => MISSING_MEDIA_METHODS,
        "mediaset" => MISSING_MEDIASET_METHODS,
        "moduleinfo" => MISSING_MODULEINFO_METHODS,
        "navapp" => MISSING_NAVAPP_METHODS,
        "notification" => MISSING_NOTIFICATION_METHODS,
        "page" => MISSING_PAGE_METHODS,
        "productname" => MISSING_PRODUCTNAME_METHODS,
        "query" => MISSING_QUERY_METHODS,
        "record" => MISSING_RECORD_METHODS,
        "table" => MISSING_RECORD_METHODS,
        "recordref" => MISSING_RECORDREF_METHODS,
        "report" => MISSING_REPORT_METHODS,
        "requestpage" => MISSING_REQUESTPAGE_METHODS,
        "session" => MISSING_SESSION_METHODS,
        "sessioninformation" => MISSING_SESSIONINFORMATION_METHODS,
        "system" => MISSING_SYSTEM_METHODS,
        "testfield" => MISSING_TESTFIELD_METHODS,
        "testhttprequestmessage" => MISSING_TESTHTTPREQUESTMESSAGE_METHODS,
        "text" => MISSING_TEXT_METHODS,
        "textbuilder" => MISSING_TEXTBUILDER_METHODS,
        "textconst" => MISSING_TEXTCONST_METHODS,
        "variant" => MISSING_VARIANT_METHODS,
        "webserviceactioncontext" => MISSING_WEBSERVICEACTIONCONTEXT_METHODS,
        "xmlattribute" => MISSING_XMLATTRIBUTE_METHODS,
        "xmlattributecollection" => MISSING_XMLATTRIBUTECOLLECTION_METHODS,
        "xmlcdata" => MISSING_XMLCDATA_METHODS,
        "xmlcomment" => MISSING_XMLCOMMENT_METHODS,
        "xmldeclaration" => MISSING_XMLDECLARATION_METHODS,
        "xmldocument" => MISSING_XMLDOCUMENT_METHODS,
        "xmldocumenttype" => MISSING_XMLDOCUMENTTYPE_METHODS,
        "xmlelement" => MISSING_XMLELEMENT_METHODS,
        "xmlnode" => MISSING_XMLNODE_METHODS,
        "xmlport" => MISSING_XMLPORT_METHODS,
        "xmlprocessinginstruction" => MISSING_XMLPROCESSINGINSTRUCTION_METHODS,
        "xmltext" => MISSING_XMLTEXT_METHODS,
        _ => EMPTY_METHODS,
    }
}
