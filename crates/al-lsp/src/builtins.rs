#[derive(Debug, Clone, Copy)]
pub struct BuiltinMethodParam {
    pub label: &'static str,
    pub documentation: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinMethodDoc {
    pub name: &'static str,
    pub signature: &'static str,
    pub summary: &'static str,
    pub docs_url: &'static str,
    pub params: &'static [BuiltinMethodParam],
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinPropertyDoc {
    pub name: &'static str,
    pub summary: &'static str,
    pub literal_values: &'static [&'static str],
}

const RECORD_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/record/record-data-type";
const CODEUNIT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/codeunit/codeunit-data-type";
const PAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/page/page-data-type";
const REPORT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/report/report-data-type";
const QUERY_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/query/query-data-type";
const XMLPORT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlport/xmlport-data-type";
const ENUM_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/enum/enum-data-type";
const LIST_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/list/list-data-type";
const DICTIONARY_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/dictionary/dictionary-data-type";
const TEXT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/text/text-data-type";
const BIGTEXT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/bigtext/bigtext-data-type";
const GUID_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/guid/guid-data-type";
const JSON_OBJECT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonobject/jsonobject-data-type";
const JSON_ARRAY_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonarray/jsonarray-data-type";
const JSON_TOKEN_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsontoken/jsontoken-data-type";
const JSON_VALUE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/jsonvalue/jsonvalue-data-type";
const HTTP_CLIENT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpclient/httpclient-data-type";
const HTTP_REQUEST_MESSAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httprequestmessage/httprequestmessage-data-type";
const HTTP_RESPONSE_MESSAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpresponsemessage/httpresponsemessage-data-type";
const HTTP_CONTENT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpcontent/httpcontent-data-type";
const HTTP_HEADERS_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/httpheaders/httpheaders-data-type";
const XML_DOCUMENT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocument/xmldocument-data-type";
const XML_ELEMENT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlelement/xmlelement-data-type";
const XML_NODE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnode/xmlnode-data-type";
const XML_NODE_LIST_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnodelist/xmlnodelist-data-type";
const XML_ATTRIBUTE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattribute/xmlattribute-data-type";
const RECORDREF_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordref/recordref-data-type";
const FIELDREF_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fieldref/fieldref-data-type";
const KEYREF_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/keyref/keyref-data-type";
const DATE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/date/date-data-type";
const TIME_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/time/time-data-type";
const DATETIME_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/datetime/datetime-data-type";
const DURATION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/duration/duration-data-type";
const INSTREAM_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/instream/instream-data-type";
const OUTSTREAM_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/outstream/outstream-data-type";
const RECORDID_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/recordid/recordid-data-type";
const VARIANT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/variant/variant-data-type";
const SESSION_SETTINGS_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/sessionsettings/sessionsettings-data-type";
const NOTIFICATION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/notification/notification-data-type";
const DIALOG_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/dialog/dialog-data-type";
const SESSION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/session/session-data-type";
const MODULE_INFO_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/moduleinfo/moduleinfo-data-type";
const SECRET_TEXT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/secrettext/secrettext-data-type";
const DATABASE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/database/database-data-type";
const SYSTEM_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/system/system-data-type";
const SESSION_INFORMATION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/sessioninformation/sessioninformation-data-type";
const TASK_SCHEDULER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/taskscheduler/taskscheduler-data-type";
const FILTER_PAGE_BUILDER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/filterpagebuilder/filterpagebuilder-data-type";
const BLOB_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/blob/blob-data-type";
const FILE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/file/file-data-type";
const VERSION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/version/version-data-type";
const MODULE_DEPENDENCY_INFO_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/moduledependencyinfo/moduledependencyinfo-data-type";
const NAV_APP_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/navapp/navapp-data-type";
const NUMBER_SEQUENCE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/numbersequence/numbersequence-data-type";
const TEXT_BUILDER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/textbuilder/textbuilder-data-type";
const MEDIA_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/media/media-data-type";
const MEDIA_SET_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/mediaset/mediaset-data-type";
const ERROR_INFO_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/errorinfo/errorinfo-data-type";
const TEST_PAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testpage/testpage-data-type";
const TEST_FIELD_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testfield/testfield-data-type";
const TEST_ACTION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testaction/testaction-data-type";
const TEST_REQUEST_PAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testrequestpage/testrequestpage-data-type";
const TEST_PART_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testpart/testpart-data-type";
const TEST_FILTER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testfilter/testfilter-data-type";
const BIG_INTEGER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/biginteger/biginteger-data-type";
const BOOLEAN_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/boolean/boolean-data-type";
const BYTE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/byte/byte-data-type";
const COMPANY_PROPERTY_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/companyproperty/companyproperty-data-type";
const COOKIE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/cookie/cookie-data-type";
const DATA_TRANSFER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/datatransfer/datatransfer-data-type";
const DEBUGGER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/debugger/debugger-data-type";
const DECIMAL_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/decimal/decimal-data-type";
const FILE_UPLOAD_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/fileupload/fileupload-data-type";
const INTEGER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/integer/integer-data-type";
const ISOLATED_STORAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/isolatedstorage/isolatedstorage-data-type";
const LABEL_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/label/label-data-type";
const PRODUCT_NAME_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/productname/productname-data-type";
const REQUEST_PAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/requestpage/requestpage-data-type";
const TEST_FILTER_FIELD_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testfilterfield/testfilterfield-data-type";
const TEST_HTTP_REQUEST_MESSAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testhttprequestmessage/testhttprequestmessage-data-type";
const TEST_HTTP_RESPONSE_MESSAGE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/testhttpresponsemessage/testhttpresponsemessage-data-type";
const WEB_SERVICE_ACTION_CONTEXT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/webserviceactioncontext/webserviceactioncontext-data-type";
const XML_ATTRIBUTE_COLLECTION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlattributecollection/xmlattributecollection-data-type";
const XML_CDATA_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcdata/xmlcdata-data-type";
const XML_COMMENT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlcomment/xmlcomment-data-type";
const XML_DECLARATION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldeclaration/xmldeclaration-data-type";
const XML_DOCUMENT_TYPE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmldocumenttype/xmldocumenttype-data-type";
const XML_NAMESPACE_MANAGER_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnamespacemanager/xmlnamespacemanager-data-type";
const XML_NAME_TABLE_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlnametable/xmlnametable-data-type";
const XML_PROCESSING_INSTRUCTION_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlprocessinginstruction/xmlprocessinginstruction-data-type";
const XML_READ_OPTIONS_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlreadoptions/xmlreadoptions-data-type";
const XML_TEXT_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmltext/xmltext-data-type";
const XML_WRITE_OPTIONS_DOCS: &str =
    "https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/methods-auto/xmlwriteoptions/xmlwriteoptions-data-type";

const PARAM_NONE: &[BuiltinMethodParam] = &[];

const RECORD_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddLink",
        signature: "AddLink(LinkId: Integer [, Description: Text])",
        summary: "Adds a link to the current record.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "LinkId: Integer",
                documentation: "Identifier of the link.",
            },
            BuiltinMethodParam {
                label: "Description: Text",
                documentation: "Optional description shown for the link.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddLoadFields",
        signature: "AddLoadFields(Field1: Any [, Field2: Any, ...]): Boolean",
        summary: "Adds fields to the set loaded from SQL.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First field to add to the load set.",
        }],
    },
    BuiltinMethodDoc {
        name: "AreFieldsLoaded",
        signature: "AreFieldsLoaded(Field1: Any [, Field2: Any, ...]): Boolean",
        summary: "Checks whether the specified fields are already loaded.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First field to test.",
        }],
    },
    BuiltinMethodDoc {
        name: "Ascending",
        signature: "Ascending([SetAscending: Boolean])",
        summary: "Gets or sets ascending sort direction for the current key.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "SetAscending: Boolean",
            documentation: "Optional new ascending mode.",
        }],
    },
    BuiltinMethodDoc {
        name: "CalcFields",
        signature: "CalcFields(Field1: Any [, Field2: Any, ...])",
        summary: "Calculates FlowField values for the record.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First FlowField to calculate.",
        }],
    },
    BuiltinMethodDoc {
        name: "CalcSums",
        signature: "CalcSums(Field1: Any [, Field2: Any, ...])",
        summary: "Calculates summed values for SumIndexFields.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First SumIndexField to aggregate.",
        }],
    },
    BuiltinMethodDoc {
        name: "ChangeCompany",
        signature: "ChangeCompany([CompanyName: Text]): Boolean",
        summary: "Redirects this record variable to another company.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "CompanyName: Text",
            documentation: "Target company name.",
        }],
    },
    BuiltinMethodDoc {
        name: "ClearMarks",
        signature: "ClearMarks()",
        summary: "Clears all marks in the table for the session.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Copy",
        signature: "Copy(FromRecord: Record)",
        summary: "Copies field values from another record instance.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "FromRecord: Record",
            documentation: "Record to copy from.",
        }],
    },
    BuiltinMethodDoc {
        name: "CopyFilters",
        signature: "CopyFilters(FromRecord: Record)",
        summary: "Copies current filters from another record.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "FromRecord: Record",
            documentation: "Record providing filters.",
        }],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of records in the current filter set.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Delete",
        signature: "Delete([RunTrigger: Boolean]): Boolean",
        summary: "Deletes the current record.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "RunTrigger: Boolean",
            documentation: "Run OnDelete trigger when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "DeleteAll",
        signature: "DeleteAll([RunTrigger: Boolean])",
        summary: "Deletes all records in current filter.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "RunTrigger: Boolean",
            documentation: "Run OnDelete trigger for each deleted record.",
        }],
    },
    BuiltinMethodDoc {
        name: "DeleteLinks",
        signature: "DeleteLinks()",
        summary: "Deletes links for the current record.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FieldActive",
        signature: "FieldActive(Field: Any): Boolean",
        summary: "Checks whether a field is active in current key/filter context.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field: Any",
            documentation: "Field to check.",
        }],
    },
    BuiltinMethodDoc {
        name: "FieldCaption",
        signature: "FieldCaption(Field: Any): Text",
        summary: "Returns caption for the specified field.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field: Any",
            documentation: "Field reference.",
        }],
    },
    BuiltinMethodDoc {
        name: "FilterGroup",
        signature: "FilterGroup([NewGroup: Integer]): Integer",
        summary: "Gets or sets active filter group.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewGroup: Integer",
            documentation: "Optional filter group to activate.",
        }],
    },
    BuiltinMethodDoc {
        name: "Find",
        signature: "Find([Which: Text]): Boolean",
        summary: "Finds a record based on the search direction.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Which: Text",
            documentation: "Search direction, for example '=' or '+'.",
        }],
    },
    BuiltinMethodDoc {
        name: "FindFirst",
        signature: "FindFirst(): Boolean",
        summary: "Finds the first record in the current key/filter.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FindLast",
        signature: "FindLast(): Boolean",
        summary: "Finds the last record in the current key/filter.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FindSet",
        signature: "FindSet([ForUpdate: Boolean] [, UpdateKey: Boolean]): Boolean",
        summary: "Finds a set of records for iteration.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "ForUpdate: Boolean",
                documentation: "Lock records for update.",
            },
            BuiltinMethodParam {
                label: "UpdateKey: Boolean",
                documentation: "Update key while iterating.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get([PrimaryKeyValue1: Any, ...]): Boolean",
        summary: "Gets a record by primary key.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "PrimaryKeyValue1: Any",
            documentation: "First primary key value.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetByRecordId",
        signature: "GetByRecordId(RecordId: RecordId): Boolean",
        summary: "Gets a record by RecordId.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "RecordId: RecordId",
            documentation: "RecordId to retrieve.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetBySystemId",
        signature: "GetBySystemId(SystemId: Guid): Boolean",
        summary: "Gets a record by SystemId.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "SystemId: Guid",
            documentation: "SystemId value.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetFilters",
        signature: "GetFilters(): Text",
        summary: "Returns current filters as text.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetPosition",
        signature: "GetPosition([UseNames: Boolean]): Text",
        summary: "Returns current record position as text.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "UseNames: Boolean",
            documentation: "Use field names instead of numbers.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRangeMax",
        signature: "GetRangeMax(Field: Any): Any",
        summary: "Returns upper bound of current range filter.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field: Any",
            documentation: "Field with range filter.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRangeMin",
        signature: "GetRangeMin(Field: Any): Any",
        summary: "Returns lower bound of current range filter.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field: Any",
            documentation: "Field with range filter.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetView",
        signature: "GetView([UseNames: Boolean]): Text",
        summary: "Returns key and filter definition as view text.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "UseNames: Boolean",
            documentation: "Use field names instead of numbers.",
        }],
    },
    BuiltinMethodDoc {
        name: "HasFilter",
        signature: "HasFilter(): Boolean",
        summary: "Checks whether any filter is set.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasLinks",
        signature: "HasLinks(): Boolean",
        summary: "Checks whether the current record has links.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Init",
        signature: "Init()",
        summary: "Initializes fields with their default values.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert([RunTrigger: Boolean] [, InsertWithSystemId: Boolean]): Boolean",
        summary: "Inserts the current record.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "RunTrigger: Boolean",
                documentation: "Run OnInsert trigger when true.",
            },
            BuiltinMethodParam {
                label: "InsertWithSystemId: Boolean",
                documentation: "Preserve assigned SystemId when true.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IsEmpty",
        signature: "IsEmpty(): Boolean",
        summary: "Checks whether the current filter set is empty.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LoadFields",
        signature: "LoadFields(Field1: Any [, Field2: Any, ...])",
        summary: "Loads specific fields for the current record.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First field to load.",
        }],
    },
    BuiltinMethodDoc {
        name: "LockTable",
        signature: "LockTable([Wait: Boolean] [, VersionCheck: Boolean])",
        summary: "Locks the table for write operations.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Wait: Boolean",
                documentation: "Wait for lock when true.",
            },
            BuiltinMethodParam {
                label: "VersionCheck: Boolean",
                documentation: "Enable optimistic version check.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Mark",
        signature: "Mark([MarkValue: Boolean]): Boolean",
        summary: "Gets or sets mark on the current record.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "MarkValue: Boolean",
            documentation: "New mark value.",
        }],
    },
    BuiltinMethodDoc {
        name: "MarkedOnly",
        signature: "MarkedOnly([MarkValue: Boolean])",
        summary: "Includes only marked records in current set.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "MarkValue: Boolean",
            documentation: "Enable marked-only filtering.",
        }],
    },
    BuiltinMethodDoc {
        name: "Modify",
        signature: "Modify([RunTrigger: Boolean]): Boolean",
        summary: "Modifies the current record.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "RunTrigger: Boolean",
            documentation: "Run OnModify trigger when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "ModifyAll",
        signature: "ModifyAll(Field: Any, NewValue: Any [, RunTrigger: Boolean])",
        summary: "Updates a field value for all records in filter set.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field to update.",
            },
            BuiltinMethodParam {
                label: "NewValue: Any",
                documentation: "New value for the field.",
            },
            BuiltinMethodParam {
                label: "RunTrigger: Boolean",
                documentation: "Run OnModify trigger when true.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Next",
        signature: "Next([Steps: Integer]): Integer",
        summary: "Moves to the next record and returns movement result.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Steps: Integer",
            documentation: "Number of records to move.",
        }],
    },
    BuiltinMethodDoc {
        name: "ReadIsolation",
        signature: "ReadIsolation([IsolationLevel: IsolationLevel]): IsolationLevel",
        summary: "Gets or sets read isolation level.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "IsolationLevel: IsolationLevel",
            documentation: "Isolation level to set.",
        }],
    },
    BuiltinMethodDoc {
        name: "RecordId",
        signature: "RecordId(): RecordId",
        summary: "Returns RecordId for the current record.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Rename",
        signature: "Rename([PrimaryKeyValue1: Any, ...]): Boolean",
        summary: "Renames the current record by changing primary key.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "PrimaryKeyValue1: Any",
            documentation: "First new primary key value.",
        }],
    },
    BuiltinMethodDoc {
        name: "Reset",
        signature: "Reset()",
        summary: "Removes filters, marks, and key settings.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetAscending",
        signature: "SetAscending(Field: Any, Ascending: Boolean)",
        summary: "Sets ascending/descending order for a key field.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field in current key.",
            },
            BuiltinMethodParam {
                label: "Ascending: Boolean",
                documentation: "True for ascending order.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetAutoCalcFields",
        signature: "SetAutoCalcFields(Field1: Any [, Field2: Any, ...])",
        summary: "Configures FlowFields to auto-calculate when read.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First FlowField to auto-calculate.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetBaseLoadFields",
        signature: "SetBaseLoadFields()",
        summary: "Resets load fields to the base set.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetCurrentKey",
        signature: "SetCurrentKey(Field1: Any [, Field2: Any, ...]): Boolean",
        summary: "Sets the current key for sorting.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First field in the desired key.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetFilter",
        signature: "SetFilter(Field: Any, String: Text [, Value: Any, ...])",
        summary: "Applies a filter expression to a field.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field to filter.",
            },
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "Filter expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetLoadFields",
        signature: "SetLoadFields([Field1: Any, ...]): Boolean",
        summary: "Replaces load fields set.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: Any",
            documentation: "First field to include.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetPosition",
        signature: "SetPosition(String: Text)",
        summary: "Sets record position from a previously captured position string.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "String: Text",
            documentation: "Position string from GetPosition.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetRange",
        signature: "SetRange(Field: Any [, FromValue: Any] [, ToValue: Any])",
        summary: "Applies a simple range filter.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field to filter.",
            },
            BuiltinMethodParam {
                label: "FromValue: Any",
                documentation: "Range start.",
            },
            BuiltinMethodParam {
                label: "ToValue: Any",
                documentation: "Range end.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetRecFilter",
        signature: "SetRecFilter()",
        summary: "Sets filters to match current record values.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetView",
        signature: "SetView(String: Text)",
        summary: "Applies key and filters from a view string.",
        docs_url: RECORD_DOCS,
        params: &[BuiltinMethodParam {
            label: "String: Text",
            documentation: "View text from GetView.",
        }],
    },
    BuiltinMethodDoc {
        name: "TableCaption",
        signature: "TableCaption(): Text",
        summary: "Returns caption of the table.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TableName",
        signature: "TableName(): Text",
        summary: "Returns name of the table.",
        docs_url: RECORD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TestField",
        signature: "TestField(Field: Any [, Value: Any])",
        summary: "Validates that a field has a value or matches an expected value.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field to test.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Optional expected value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "TransferFields",
        signature: "TransferFields(FromRecord: Record [, InitPrimaryKeyFields: Boolean])",
        summary: "Transfers matching fields from another record type.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "FromRecord: Record",
                documentation: "Source record.",
            },
            BuiltinMethodParam {
                label: "InitPrimaryKeyFields: Boolean",
                documentation: "Initialize primary keys when true.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Validate",
        signature: "Validate(Field: Any [, NewValue: Any])",
        summary: "Validates and assigns field value via OnValidate.",
        docs_url: RECORD_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field to validate.",
            },
            BuiltinMethodParam {
                label: "NewValue: Any",
                documentation: "Optional value to validate and assign.",
            },
        ],
    },
];

const CODEUNIT_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "Run",
    signature: "Run([Record: Record]): Boolean",
    summary: "Runs the codeunit and optionally passes a record.",
    docs_url: CODEUNIT_DOCS,
    params: &[BuiltinMethodParam {
        label: "Record: Record",
        documentation: "Optional record input/output parameter.",
    }],
}];

const PAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Run",
        signature: "Run()",
        summary: "Runs a page non-modally.",
        docs_url: PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RunModal",
        signature: "RunModal(): Action",
        summary: "Runs a page modally and returns closing action.",
        docs_url: PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetRecord",
        signature: "SetRecord(Record: Record)",
        summary: "Sets the current record for the page.",
        docs_url: PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Record: Record",
            documentation: "Record shown by the page.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRecord",
        signature: "GetRecord(var Record: Record)",
        summary: "Gets the current record from the page.",
        docs_url: PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Record: Record",
            documentation: "Output record variable.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetTableView",
        signature: "SetTableView(Record: Record)",
        summary: "Applies table view from a record variable.",
        docs_url: PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Record: Record",
            documentation: "Record carrying key/filter view.",
        }],
    },
    BuiltinMethodDoc {
        name: "LookupMode",
        signature: "LookupMode([NewLookupMode: Boolean]): Boolean",
        summary: "Gets or sets lookup mode for the page.",
        docs_url: PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewLookupMode: Boolean",
            documentation: "Optional new lookup mode.",
        }],
    },
    BuiltinMethodDoc {
        name: "Update",
        signature: "Update([SaveRecord: Boolean])",
        summary: "Refreshes page and optionally saves current record.",
        docs_url: PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "SaveRecord: Boolean",
            documentation: "Save current record before update.",
        }],
    },
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes the page.",
        docs_url: PAGE_DOCS,
        params: PARAM_NONE,
    },
];

const REPORT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Run",
        signature: "Run([RequestWindow: Boolean])",
        summary: "Runs the report.",
        docs_url: REPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "RequestWindow: Boolean",
            documentation: "Show request page when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "RunModal",
        signature: "RunModal([RequestWindow: Boolean]): Action",
        summary: "Runs the report modally.",
        docs_url: REPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "RequestWindow: Boolean",
            documentation: "Show request page when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "Print",
        signature: "Print()",
        summary: "Prints the report output.",
        docs_url: REPORT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SaveAs",
        signature: "SaveAs(Format: ReportFormat, OutStream: OutStream)",
        summary: "Saves report output to a stream.",
        docs_url: REPORT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Format: ReportFormat",
                documentation: "Output format.",
            },
            BuiltinMethodParam {
                label: "OutStream: OutStream",
                documentation: "Output stream.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Execute",
        signature: "Execute()",
        summary: "Executes report processing without rendering output.",
        docs_url: REPORT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetTableView",
        signature: "SetTableView(Record: Record)",
        summary: "Applies table view for report data item filtering.",
        docs_url: REPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Record: Record",
            documentation: "Record with filters/key.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetRecord",
        signature: "SetRecord(Record: Record)",
        summary: "Sets current record for a single-record report run.",
        docs_url: REPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Record: Record",
            documentation: "Record to use.",
        }],
    },
    BuiltinMethodDoc {
        name: "UseRequestPage",
        signature: "UseRequestPage([NewValue: Boolean]): Boolean",
        summary: "Gets or sets request-page usage.",
        docs_url: REPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewValue: Boolean",
            documentation: "Optional new setting.",
        }],
    },
];

const QUERY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Open",
        signature: "Open()",
        summary: "Opens a query dataset.",
        docs_url: QUERY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Read",
        signature: "Read(): Boolean",
        summary: "Reads the next row from the query.",
        docs_url: QUERY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes query dataset.",
        docs_url: QUERY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SaveAsXml",
        signature: "SaveAsXml(OutStream: OutStream)",
        summary: "Writes query results as XML.",
        docs_url: QUERY_DOCS,
        params: &[BuiltinMethodParam {
            label: "OutStream: OutStream",
            documentation: "Destination stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetFilter",
        signature: "SetFilter(Column: Any, String: Text [, Value: Any, ...])",
        summary: "Applies filter to query column.",
        docs_url: QUERY_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Column: Any",
                documentation: "Column to filter.",
            },
            BuiltinMethodParam {
                label: "String: Text",
                documentation: "Filter expression.",
            },
        ],
    },
];

const XMLPORT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Run",
        signature: "Run()",
        summary: "Runs the XMLport.",
        docs_url: XMLPORT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Import",
        signature: "Import()",
        summary: "Runs XMLport in import mode.",
        docs_url: XMLPORT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Export",
        signature: "Export()",
        summary: "Runs XMLport in export mode.",
        docs_url: XMLPORT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetSource",
        signature: "SetSource(InStream: InStream)",
        summary: "Sets source stream for import.",
        docs_url: XMLPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "InStream: InStream",
            documentation: "Input stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetDestination",
        signature: "SetDestination(OutStream: OutStream)",
        summary: "Sets destination stream for export.",
        docs_url: XMLPORT_DOCS,
        params: &[BuiltinMethodParam {
            label: "OutStream: OutStream",
            documentation: "Output stream.",
        }],
    },
];

const ENUM_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Names",
        signature: "Names(): List of [Text]",
        summary: "Gets enum value names.",
        docs_url: ENUM_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Ordinals",
        signature: "Ordinals(): List of [Integer]",
        summary: "Gets enum ordinals.",
        docs_url: ENUM_DOCS,
        params: PARAM_NONE,
    },
];

const LIST_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Value: T)",
        summary: "Appends an item.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: T",
            documentation: "Value to append.",
        }],
    },
    BuiltinMethodDoc {
        name: "AddRange",
        signature: "AddRange(Values: List of [T])",
        summary: "Appends values from another list.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Values: List of [T]",
            documentation: "Values to append.",
        }],
    },
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Value: T): Boolean",
        summary: "Checks whether value exists in the list.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: T",
            documentation: "Value to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of list items.",
        docs_url: LIST_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Index: Integer): T",
        summary: "Gets item by index.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Zero-based index.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRange",
        signature: "GetRange(Index: Integer, Count: Integer): List of [T]",
        summary: "Gets a slice of items.",
        docs_url: LIST_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "Start index.",
            },
            BuiltinMethodParam {
                label: "Count: Integer",
                documentation: "Number of items.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "IndexOf",
        signature: "IndexOf(Value: T): Integer",
        summary: "Finds index of value.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: T",
            documentation: "Value to search.",
        }],
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(Index: Integer, Value: T)",
        summary: "Inserts item at index.",
        docs_url: LIST_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "Insert position.",
            },
            BuiltinMethodParam {
                label: "Value: T",
                documentation: "Value to insert.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Value: T): Boolean",
        summary: "Removes first matching value.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: T",
            documentation: "Value to remove.",
        }],
    },
    BuiltinMethodDoc {
        name: "RemoveAt",
        signature: "RemoveAt(Index: Integer)",
        summary: "Removes value at index.",
        docs_url: LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Index to remove.",
        }],
    },
    BuiltinMethodDoc {
        name: "Set",
        signature: "Set(Index: Integer, Value: T)",
        summary: "Replaces value at index.",
        docs_url: LIST_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "Index to update.",
            },
            BuiltinMethodParam {
                label: "Value: T",
                documentation: "New value.",
            },
        ],
    },
];

const DICTIONARY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Key: K, Value: V)",
        summary: "Adds a key/value pair.",
        docs_url: DICTIONARY_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: K",
                documentation: "Dictionary key.",
            },
            BuiltinMethodParam {
                label: "Value: V",
                documentation: "Dictionary value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ContainsKey",
        signature: "ContainsKey(Key: K): Boolean",
        summary: "Checks whether key exists.",
        docs_url: DICTIONARY_DOCS,
        params: &[BuiltinMethodParam {
            label: "Key: K",
            documentation: "Key to check.",
        }],
    },
    BuiltinMethodDoc {
        name: "ContainsValue",
        signature: "ContainsValue(Value: V): Boolean",
        summary: "Checks whether value exists.",
        docs_url: DICTIONARY_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: V",
            documentation: "Value to check.",
        }],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of entries.",
        docs_url: DICTIONARY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Key: K): V",
        summary: "Gets value for key.",
        docs_url: DICTIONARY_DOCS,
        params: &[BuiltinMethodParam {
            label: "Key: K",
            documentation: "Lookup key.",
        }],
    },
    BuiltinMethodDoc {
        name: "Keys",
        signature: "Keys(): List of [K]",
        summary: "Gets keys as list.",
        docs_url: DICTIONARY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Key: K): Boolean",
        summary: "Removes key/value pair by key.",
        docs_url: DICTIONARY_DOCS,
        params: &[BuiltinMethodParam {
            label: "Key: K",
            documentation: "Key to remove.",
        }],
    },
    BuiltinMethodDoc {
        name: "Set",
        signature: "Set(Key: K, Value: V)",
        summary: "Assigns value for key.",
        docs_url: DICTIONARY_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: K",
                documentation: "Key to set.",
            },
            BuiltinMethodParam {
                label: "Value: V",
                documentation: "New value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Values",
        signature: "Values(): List of [V]",
        summary: "Gets values as list.",
        docs_url: DICTIONARY_DOCS,
        params: PARAM_NONE,
    },
];

const TEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Value: Text): Boolean",
        summary: "Checks whether the text contains a value.",
        docs_url: TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Text to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "StartsWith",
        signature: "StartsWith(Value: Text): Boolean",
        summary: "Checks whether the text starts with a value.",
        docs_url: TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Start text to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "EndsWith",
        signature: "EndsWith(Value: Text): Boolean",
        summary: "Checks whether the text ends with a value.",
        docs_url: TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "End text to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "IndexOf",
        signature: "IndexOf(Value: Text): Integer",
        summary: "Returns the zero-based index of a value in the text.",
        docs_url: TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Text to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "LastIndexOf",
        signature: "LastIndexOf(Value: Text): Integer",
        summary: "Returns the last zero-based index of a value in the text.",
        docs_url: TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Text to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): Integer",
        summary: "Returns the number of characters in the text.",
        docs_url: TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "MaxStrLen",
        signature: "MaxStrLen(): Integer",
        summary: "Returns the maximum length for a Text/Code value.",
        docs_url: TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SubString",
        signature: "SubString(StartIndex: Integer [, Length: Integer]): Text",
        summary: "Returns a substring from the specified start index.",
        docs_url: TEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "StartIndex: Integer",
                documentation: "Zero-based start index.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional number of characters.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Trim",
        signature: "Trim(): Text",
        summary: "Removes leading and trailing whitespace characters.",
        docs_url: TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ToLower",
        signature: "ToLower(): Text",
        summary: "Converts text to lowercase.",
        docs_url: TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ToUpper",
        signature: "ToUpper(): Text",
        summary: "Converts text to uppercase.",
        docs_url: TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Replace",
        signature: "Replace(OldValue: Text, NewValue: Text): Text",
        summary: "Replaces all occurrences of one value with another.",
        docs_url: TEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "OldValue: Text",
                documentation: "Value to replace.",
            },
            BuiltinMethodParam {
                label: "NewValue: Text",
                documentation: "Replacement value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Split",
        signature: "Split(Separator: Text): List of [Text]",
        summary: "Splits text into a list of values.",
        docs_url: TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Separator: Text",
            documentation: "Separator text.",
        }],
    },
    BuiltinMethodDoc {
        name: "PadLeft",
        signature: "PadLeft(TotalWidth: Integer [, PaddingChar: Char]): Text",
        summary: "Pads text on the left to a target width.",
        docs_url: TEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "TotalWidth: Integer",
                documentation: "Desired total width.",
            },
            BuiltinMethodParam {
                label: "PaddingChar: Char",
                documentation: "Optional padding character.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "PadRight",
        signature: "PadRight(TotalWidth: Integer [, PaddingChar: Char]): Text",
        summary: "Pads text on the right to a target width.",
        docs_url: TEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "TotalWidth: Integer",
                documentation: "Desired total width.",
            },
            BuiltinMethodParam {
                label: "PaddingChar: Char",
                documentation: "Optional padding character.",
            },
        ],
    },
];

const BIGTEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddText",
        signature: "AddText(NewText: Text [, Position: Integer])",
        summary: "Appends or inserts text into a BigText value.",
        docs_url: BIGTEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "NewText: Text",
                documentation: "Text to add.",
            },
            BuiltinMethodParam {
                label: "Position: Integer",
                documentation: "Optional insertion position.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear()",
        summary: "Clears the BigText value.",
        docs_url: BIGTEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetSubText",
        signature: "GetSubText(var Result: Text, Position: Integer [, Length: Integer])",
        summary: "Reads a segment of BigText into a Text variable.",
        docs_url: BIGTEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "var Result: Text",
                documentation: "Output text segment.",
            },
            BuiltinMethodParam {
                label: "Position: Integer",
                documentation: "Start position.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional number of characters.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): Integer",
        summary: "Returns character length of the BigText value.",
        docs_url: BIGTEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TextPos",
        signature: "TextPos(SubText: Text [, Position: Integer]): Integer",
        summary: "Finds position of subtext in BigText.",
        docs_url: BIGTEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "SubText: Text",
                documentation: "Text to search for.",
            },
            BuiltinMethodParam {
                label: "Position: Integer",
                documentation: "Optional search start position.",
            },
        ],
    },
];

const GUID_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CreateGuid",
        signature: "CreateGuid(): Guid",
        summary: "Creates a new GUID value.",
        docs_url: GUID_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsNullGuid",
        signature: "IsNullGuid(GuidValue: Guid): Boolean",
        summary: "Checks whether a GUID is all zeros.",
        docs_url: GUID_DOCS,
        params: &[BuiltinMethodParam {
            label: "GuidValue: Guid",
            documentation: "GUID to evaluate.",
        }],
    },
    BuiltinMethodDoc {
        name: "Parse",
        signature: "Parse(Value: Text): Guid",
        summary: "Parses a textual GUID into a Guid value.",
        docs_url: GUID_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "GUID text representation.",
        }],
    },
    BuiltinMethodDoc {
        name: "ToText",
        signature: "ToText(): Text",
        summary: "Converts a Guid value to text.",
        docs_url: GUID_DOCS,
        params: PARAM_NONE,
    },
];

const JSON_OBJECT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Key: Text, Value: JsonToken)",
        summary: "Adds a key/value pair to the JSON object.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Property name.",
            },
            BuiltinMethodParam {
                label: "Value: JsonToken",
                documentation: "JSON token value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Key: Text): Boolean",
        summary: "Checks whether a property exists.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Key: Text",
            documentation: "Property name.",
        }],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns the number of properties.",
        docs_url: JSON_OBJECT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Key: Text, var Result: JsonToken): Boolean",
        summary: "Gets a property token by key.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Property name.",
            },
            BuiltinMethodParam {
                label: "var Result: JsonToken",
                documentation: "Output token.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Keys",
        signature: "Keys(): List of [Text]",
        summary: "Returns the list of property names.",
        docs_url: JSON_OBJECT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadFrom",
        signature: "ReadFrom(JsonText: Text): Boolean",
        summary: "Reads a JSON object from text.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[BuiltinMethodParam {
            label: "JsonText: Text",
            documentation: "Serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(var JsonText: Text): Boolean",
        summary: "Writes the JSON object to text.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[BuiltinMethodParam {
            label: "var JsonText: Text",
            documentation: "Output serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "Replace",
        signature: "Replace(Key: Text, Value: JsonToken): Boolean",
        summary: "Replaces an existing property value.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Property name.",
            },
            BuiltinMethodParam {
                label: "Value: JsonToken",
                documentation: "Replacement value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Key: Text): Boolean",
        summary: "Removes a property by key.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Key: Text",
            documentation: "Property name.",
        }],
    },
    BuiltinMethodDoc {
        name: "SelectToken",
        signature: "SelectToken(Path: Text, var Result: JsonToken): Boolean",
        summary: "Selects a token using a JSON path expression.",
        docs_url: JSON_OBJECT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "JSON path expression.",
            },
            BuiltinMethodParam {
                label: "var Result: JsonToken",
                documentation: "Output token.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Clone",
        signature: "Clone(): JsonObject",
        summary: "Creates a deep copy of the JSON object.",
        docs_url: JSON_OBJECT_DOCS,
        params: PARAM_NONE,
    },
];

const JSON_ARRAY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Value: JsonToken)",
        summary: "Appends a value to the JSON array.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: JsonToken",
            documentation: "Token to append.",
        }],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns the number of items.",
        docs_url: JSON_ARRAY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Index: Integer, var Result: JsonToken): Boolean",
        summary: "Gets an item token by index.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "Zero-based index.",
            },
            BuiltinMethodParam {
                label: "var Result: JsonToken",
                documentation: "Output token.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(Index: Integer, Value: JsonToken)",
        summary: "Inserts a token at the specified index.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "Insertion index.",
            },
            BuiltinMethodParam {
                label: "Value: JsonToken",
                documentation: "Token to insert.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Index: Integer)",
        summary: "Removes an item at the specified index.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Index to remove.",
        }],
    },
    BuiltinMethodDoc {
        name: "Set",
        signature: "Set(Index: Integer, Value: JsonToken)",
        summary: "Replaces the token at an index.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Index: Integer",
                documentation: "Index to replace.",
            },
            BuiltinMethodParam {
                label: "Value: JsonToken",
                documentation: "Replacement token.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ReadFrom",
        signature: "ReadFrom(JsonText: Text): Boolean",
        summary: "Reads a JSON array from text.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[BuiltinMethodParam {
            label: "JsonText: Text",
            documentation: "Serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(var JsonText: Text): Boolean",
        summary: "Writes the JSON array to text.",
        docs_url: JSON_ARRAY_DOCS,
        params: &[BuiltinMethodParam {
            label: "var JsonText: Text",
            documentation: "Output serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "Clone",
        signature: "Clone(): JsonArray",
        summary: "Creates a deep copy of the JSON array.",
        docs_url: JSON_ARRAY_DOCS,
        params: PARAM_NONE,
    },
];

const JSON_TOKEN_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsObject",
        signature: "AsObject(): JsonObject",
        summary: "Casts token to JsonObject.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsArray",
        signature: "AsArray(): JsonArray",
        summary: "Casts token to JsonArray.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsValue",
        signature: "AsValue(): JsonValue",
        summary: "Casts token to JsonValue.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsObject",
        signature: "IsObject(): Boolean",
        summary: "Checks whether token is an object.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsArray",
        signature: "IsArray(): Boolean",
        summary: "Checks whether token is an array.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsValue",
        signature: "IsValue(): Boolean",
        summary: "Checks whether token is a value token.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Path",
        signature: "Path(): Text",
        summary: "Returns token path inside the JSON document.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SelectToken",
        signature: "SelectToken(Path: Text, var Result: JsonToken): Boolean",
        summary: "Selects a child token by JSON path.",
        docs_url: JSON_TOKEN_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Path: Text",
                documentation: "JSON path expression.",
            },
            BuiltinMethodParam {
                label: "var Result: JsonToken",
                documentation: "Output token.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ReadFrom",
        signature: "ReadFrom(JsonText: Text): Boolean",
        summary: "Reads a JSON token from text.",
        docs_url: JSON_TOKEN_DOCS,
        params: &[BuiltinMethodParam {
            label: "JsonText: Text",
            documentation: "Serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(var JsonText: Text): Boolean",
        summary: "Writes token JSON to text.",
        docs_url: JSON_TOKEN_DOCS,
        params: &[BuiltinMethodParam {
            label: "var JsonText: Text",
            documentation: "Output serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "Clone",
        signature: "Clone(): JsonToken",
        summary: "Creates a deep copy of the token.",
        docs_url: JSON_TOKEN_DOCS,
        params: PARAM_NONE,
    },
];

const JSON_VALUE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsText",
        signature: "AsText(): Text",
        summary: "Converts JSON value to text.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsBoolean",
        signature: "AsBoolean(): Boolean",
        summary: "Converts JSON value to Boolean.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsInteger",
        signature: "AsInteger(): Integer",
        summary: "Converts JSON value to Integer.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDecimal",
        signature: "AsDecimal(): Decimal",
        summary: "Converts JSON value to Decimal.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDate",
        signature: "AsDate(): Date",
        summary: "Converts JSON value to Date.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsTime",
        signature: "AsTime(): Time",
        summary: "Converts JSON value to Time.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDateTime",
        signature: "AsDateTime(): DateTime",
        summary: "Converts JSON value to DateTime.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsNull",
        signature: "IsNull(): Boolean",
        summary: "Checks whether value is JSON null.",
        docs_url: JSON_VALUE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadFrom",
        signature: "ReadFrom(JsonText: Text): Boolean",
        summary: "Reads a JSON value from text.",
        docs_url: JSON_VALUE_DOCS,
        params: &[BuiltinMethodParam {
            label: "JsonText: Text",
            documentation: "Serialized JSON text.",
        }],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(var JsonText: Text): Boolean",
        summary: "Writes JSON value to text.",
        docs_url: JSON_VALUE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var JsonText: Text",
            documentation: "Output serialized JSON text.",
        }],
    },
];

const HTTP_CLIENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(RequestUri: Text, var Response: HttpResponseMessage): Boolean",
        summary: "Sends an HTTP GET request.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "RequestUri: Text",
                documentation: "Request URI.",
            },
            BuiltinMethodParam {
                label: "var Response: HttpResponseMessage",
                documentation: "Response message output.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Post",
        signature: "Post(RequestUri: Text, Content: HttpContent, var Response: HttpResponseMessage): Boolean",
        summary: "Sends an HTTP POST request.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "RequestUri: Text",
                documentation: "Request URI.",
            },
            BuiltinMethodParam {
                label: "Content: HttpContent",
                documentation: "Request body content.",
            },
            BuiltinMethodParam {
                label: "var Response: HttpResponseMessage",
                documentation: "Response message output.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Put",
        signature: "Put(RequestUri: Text, Content: HttpContent, var Response: HttpResponseMessage): Boolean",
        summary: "Sends an HTTP PUT request.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "RequestUri: Text",
                documentation: "Request URI.",
            },
            BuiltinMethodParam {
                label: "Content: HttpContent",
                documentation: "Request body content.",
            },
            BuiltinMethodParam {
                label: "var Response: HttpResponseMessage",
                documentation: "Response message output.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Delete",
        signature: "Delete(RequestUri: Text, var Response: HttpResponseMessage): Boolean",
        summary: "Sends an HTTP DELETE request.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "RequestUri: Text",
                documentation: "Request URI.",
            },
            BuiltinMethodParam {
                label: "var Response: HttpResponseMessage",
                documentation: "Response message output.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Send",
        signature: "Send(Request: HttpRequestMessage, var Response: HttpResponseMessage): Boolean",
        summary: "Sends a prepared HTTP request message.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Request: HttpRequestMessage",
                documentation: "Request message.",
            },
            BuiltinMethodParam {
                label: "var Response: HttpResponseMessage",
                documentation: "Response message output.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DefaultRequestHeaders",
        signature: "DefaultRequestHeaders(): HttpHeaders",
        summary: "Gets default headers applied to outgoing requests.",
        docs_url: HTTP_CLIENT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "UseResponseCookies",
        signature: "UseResponseCookies([Enable: Boolean])",
        summary: "Gets or sets cookie handling for responses.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Enable: Boolean",
            documentation: "Enable cookie handling.",
        }],
    },
    BuiltinMethodDoc {
        name: "Timeout",
        signature: "Timeout([Duration: Duration]): Duration",
        summary: "Gets or sets request timeout.",
        docs_url: HTTP_CLIENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Duration: Duration",
            documentation: "Optional timeout value.",
        }],
    },
];

const HTTP_REQUEST_MESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "SetRequestUri",
        signature: "SetRequestUri(RequestUri: Text)",
        summary: "Sets the request URI.",
        docs_url: HTTP_REQUEST_MESSAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "RequestUri: Text",
            documentation: "Request URI.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRequestUri",
        signature: "GetRequestUri(): Text",
        summary: "Gets the request URI.",
        docs_url: HTTP_REQUEST_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Method",
        signature: "Method([Verb: Text]): Text",
        summary: "Gets or sets HTTP method verb.",
        docs_url: HTTP_REQUEST_MESSAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Verb: Text",
            documentation: "Optional HTTP verb.",
        }],
    },
    BuiltinMethodDoc {
        name: "Content",
        signature: "Content([HttpContent: HttpContent]): HttpContent",
        summary: "Gets or sets request content.",
        docs_url: HTTP_REQUEST_MESSAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "HttpContent: HttpContent",
            documentation: "Optional content value.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetHeaders",
        signature: "GetHeaders(var Headers: HttpHeaders): Boolean",
        summary: "Gets request headers.",
        docs_url: HTTP_REQUEST_MESSAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Headers: HttpHeaders",
            documentation: "Output headers.",
        }],
    },
];

const HTTP_RESPONSE_MESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "IsSuccessStatusCode",
        signature: "IsSuccessStatusCode(): Boolean",
        summary: "Checks whether status code indicates success.",
        docs_url: HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HttpStatusCode",
        signature: "HttpStatusCode(): Integer",
        summary: "Gets the HTTP status code.",
        docs_url: HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReasonPhrase",
        signature: "ReasonPhrase(): Text",
        summary: "Gets the reason phrase.",
        docs_url: HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Content",
        signature: "Content(): HttpContent",
        summary: "Gets the response content.",
        docs_url: HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetHeaders",
        signature: "GetHeaders(var Headers: HttpHeaders): Boolean",
        summary: "Gets response headers.",
        docs_url: HTTP_RESPONSE_MESSAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Headers: HttpHeaders",
            documentation: "Output headers.",
        }],
    },
];

const HTTP_CONTENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "WriteFrom",
        signature: "WriteFrom(ContentText: Text)",
        summary: "Sets HTTP content from text.",
        docs_url: HTTP_CONTENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "ContentText: Text",
            documentation: "Text body.",
        }],
    },
    BuiltinMethodDoc {
        name: "ReadAs",
        signature: "ReadAs(var ContentText: Text): Boolean",
        summary: "Reads HTTP content as text.",
        docs_url: HTTP_CONTENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "var ContentText: Text",
            documentation: "Output text body.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetHeaders",
        signature: "GetHeaders(var Headers: HttpHeaders): Boolean",
        summary: "Gets content headers.",
        docs_url: HTTP_CONTENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Headers: HttpHeaders",
            documentation: "Output headers.",
        }],
    },
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear()",
        summary: "Clears HTTP content.",
        docs_url: HTTP_CONTENT_DOCS,
        params: PARAM_NONE,
    },
];

const HTTP_HEADERS_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Name: Text, Value: Text): Boolean",
        summary: "Adds an HTTP header value.",
        docs_url: HTTP_HEADERS_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Header name.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "Header value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Name: Text): Boolean",
        summary: "Checks whether a header exists.",
        docs_url: HTTP_HEADERS_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Header name.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetValues",
        signature: "GetValues(Name: Text): List of [Text]",
        summary: "Gets header values by name.",
        docs_url: HTTP_HEADERS_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Header name.",
        }],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Name: Text): Boolean",
        summary: "Removes a header.",
        docs_url: HTTP_HEADERS_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Header name.",
        }],
    },
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear()",
        summary: "Clears all header values.",
        docs_url: HTTP_HEADERS_DOCS,
        params: PARAM_NONE,
    },
];

const XML_DOCUMENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(): XmlDocument",
        summary: "Creates a new XML document.",
        docs_url: XML_DOCUMENT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ReadFrom",
        signature: "ReadFrom(XmlText: Text): Boolean",
        summary: "Reads XML from text.",
        docs_url: XML_DOCUMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "XmlText: Text",
            documentation: "Serialized XML text.",
        }],
    },
    BuiltinMethodDoc {
        name: "WriteTo",
        signature: "WriteTo(var XmlText: Text): Boolean",
        summary: "Writes XML document to text.",
        docs_url: XML_DOCUMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "var XmlText: Text",
            documentation: "Output serialized XML text.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRoot",
        signature: "GetRoot(var Root: XmlElement): Boolean",
        summary: "Gets the root XML element.",
        docs_url: XML_DOCUMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Root: XmlElement",
            documentation: "Output root element.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetRoot",
        signature: "SetRoot(Root: XmlElement)",
        summary: "Sets the root XML element.",
        docs_url: XML_DOCUMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Root: XmlElement",
            documentation: "Root element.",
        }],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Result: XmlNode): Boolean",
        summary: "Selects a single node using XPath.",
        docs_url: XML_DOCUMENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "XPath expression.",
            },
            BuiltinMethodParam {
                label: "var Result: XmlNode",
                documentation: "Output node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var Result: XmlNodeList): Boolean",
        summary: "Selects multiple nodes using XPath.",
        docs_url: XML_DOCUMENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "XPath expression.",
            },
            BuiltinMethodParam {
                label: "var Result: XmlNodeList",
                documentation: "Output node list.",
            },
        ],
    },
];

const XML_ELEMENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Gets the element name.",
        docs_url: XML_ELEMENT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "InnerText",
        signature: "InnerText(): Text",
        summary: "Gets or sets inner text content.",
        docs_url: XML_ELEMENT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "InnerXml",
        signature: "InnerXml(): Text",
        summary: "Gets or sets inner XML content.",
        docs_url: XML_ELEMENT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetAttribute",
        signature: "SetAttribute(Name: Text, Value: Text)",
        summary: "Sets an attribute value.",
        docs_url: XML_ELEMENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Attribute name.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "Attribute value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetAttribute",
        signature: "GetAttribute(Name: Text): Text",
        summary: "Gets an attribute value by name.",
        docs_url: XML_ELEMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Attribute name.",
        }],
    },
    BuiltinMethodDoc {
        name: "AppendChild",
        signature: "AppendChild(Node: XmlNode): XmlNode",
        summary: "Appends a child node.",
        docs_url: XML_ELEMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Node: XmlNode",
            documentation: "Node to append.",
        }],
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Result: XmlNode): Boolean",
        summary: "Selects a single descendant node.",
        docs_url: XML_ELEMENT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "XPath expression.",
            },
            BuiltinMethodParam {
                label: "var Result: XmlNode",
                documentation: "Output node.",
            },
        ],
    },
];

const XML_NODE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AsXmlElement",
        signature: "AsXmlElement(): XmlElement",
        summary: "Casts node to XmlElement.",
        docs_url: XML_NODE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsXmlAttribute",
        signature: "AsXmlAttribute(): XmlAttribute",
        summary: "Casts node to XmlAttribute.",
        docs_url: XML_NODE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Gets node name.",
        docs_url: XML_NODE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value(): Text",
        summary: "Gets node value.",
        docs_url: XML_NODE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "InnerText",
        signature: "InnerText(): Text",
        summary: "Gets node text value.",
        docs_url: XML_NODE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SelectSingleNode",
        signature: "SelectSingleNode(XPath: Text, var Result: XmlNode): Boolean",
        summary: "Selects a single descendant node.",
        docs_url: XML_NODE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "XPath expression.",
            },
            BuiltinMethodParam {
                label: "var Result: XmlNode",
                documentation: "Output node.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SelectNodes",
        signature: "SelectNodes(XPath: Text, var Result: XmlNodeList): Boolean",
        summary: "Selects descendant nodes.",
        docs_url: XML_NODE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "XPath: Text",
                documentation: "XPath expression.",
            },
            BuiltinMethodParam {
                label: "var Result: XmlNodeList",
                documentation: "Output node list.",
            },
        ],
    },
];

const XML_NODE_LIST_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of nodes in the list.",
        docs_url: XML_NODE_LIST_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Index: Integer): XmlNode",
        summary: "Gets node by index.",
        docs_url: XML_NODE_LIST_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Zero-based index.",
        }],
    },
];

const XML_ATTRIBUTE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Gets attribute name.",
        docs_url: XML_ATTRIBUTE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value(): Text",
        summary: "Gets or sets attribute value.",
        docs_url: XML_ATTRIBUTE_DOCS,
        params: PARAM_NONE,
    },
];

const RECORDREF_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Open",
        signature: "Open(TableNo: Integer [, Temporary: Boolean])",
        summary: "Opens a table by ID.",
        docs_url: RECORDREF_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "TableNo: Integer",
                documentation: "Table ID.",
            },
            BuiltinMethodParam {
                label: "Temporary: Boolean",
                documentation: "Open as temporary record.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes the record reference.",
        docs_url: RECORDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FindFirst",
        signature: "FindFirst(): Boolean",
        summary: "Finds first record in current view.",
        docs_url: RECORDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FindSet",
        signature: "FindSet(): Boolean",
        summary: "Finds record set for iteration.",
        docs_url: RECORDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Next",
        signature: "Next([Steps: Integer]): Integer",
        summary: "Moves in the record set.",
        docs_url: RECORDREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "Steps: Integer",
            documentation: "Number of records to move.",
        }],
    },
    BuiltinMethodDoc {
        name: "Field",
        signature: "Field(FieldNo: Integer): FieldRef",
        summary: "Gets a field reference by field number.",
        docs_url: RECORDREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "FieldNo: Integer",
            documentation: "Field number.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetTable",
        signature: "SetTable(Record: Record)",
        summary: "Loads data from a typed record variable.",
        docs_url: RECORDREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "Record: Record",
            documentation: "Source record.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetTable",
        signature: "GetTable(var Record: Record)",
        summary: "Copies data into a typed record variable.",
        docs_url: RECORDREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Record: Record",
            documentation: "Destination record.",
        }],
    },
];

const FIELDREF_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Number",
        signature: "Number(): Integer",
        summary: "Gets field number.",
        docs_url: FIELDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Gets field name.",
        docs_url: FIELDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption(): Text",
        summary: "Gets field caption.",
        docs_url: FIELDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Type",
        signature: "Type(): FieldType",
        summary: "Gets field type.",
        docs_url: FIELDREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value([NewValue: Any]): Any",
        summary: "Gets or sets field value.",
        docs_url: FIELDREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewValue: Any",
            documentation: "Optional value to assign.",
        }],
    },
    BuiltinMethodDoc {
        name: "Validate",
        signature: "Validate([NewValue: Any])",
        summary: "Validates and assigns field value.",
        docs_url: FIELDREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewValue: Any",
            documentation: "Optional value to validate.",
        }],
    },
    BuiltinMethodDoc {
        name: "Record",
        signature: "Record(): RecordRef",
        summary: "Gets parent record reference.",
        docs_url: FIELDREF_DOCS,
        params: PARAM_NONE,
    },
];

const KEYREF_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "FieldCount",
        signature: "FieldCount(): Integer",
        summary: "Returns number of fields in the key.",
        docs_url: KEYREF_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "FieldIndex",
        signature: "FieldIndex(Index: Integer): FieldRef",
        summary: "Gets key field by index.",
        docs_url: KEYREF_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "1-based field index in key.",
        }],
    },
];

const DATE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ToText",
        signature: "ToText([IncludeXmlSchemaFormat: Boolean]): Text",
        summary: "Converts Date to text.",
        docs_url: DATE_DOCS,
        params: &[BuiltinMethodParam {
            label: "IncludeXmlSchemaFormat: Boolean",
            documentation: "Use XML schema date format when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "Day",
        signature: "Day(): Integer",
        summary: "Returns day of month.",
        docs_url: DATE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Month",
        signature: "Month(): Integer",
        summary: "Returns month number.",
        docs_url: DATE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Year",
        signature: "Year(): Integer",
        summary: "Returns year number.",
        docs_url: DATE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "WeekNo",
        signature: "WeekNo(): Integer",
        summary: "Returns ISO week number.",
        docs_url: DATE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DayOfWeek",
        signature: "DayOfWeek(): Integer",
        summary: "Returns day of week number.",
        docs_url: DATE_DOCS,
        params: PARAM_NONE,
    },
];

const TIME_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ToText",
        signature: "ToText([IncludeXmlSchemaFormat: Boolean]): Text",
        summary: "Converts Time to text.",
        docs_url: TIME_DOCS,
        params: &[BuiltinMethodParam {
            label: "IncludeXmlSchemaFormat: Boolean",
            documentation: "Use XML schema time format when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "Hour",
        signature: "Hour(): Integer",
        summary: "Returns hour component.",
        docs_url: TIME_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Minute",
        signature: "Minute(): Integer",
        summary: "Returns minute component.",
        docs_url: TIME_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Second",
        signature: "Second(): Integer",
        summary: "Returns second component.",
        docs_url: TIME_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Millisecond",
        signature: "Millisecond(): Integer",
        summary: "Returns millisecond component.",
        docs_url: TIME_DOCS,
        params: PARAM_NONE,
    },
];

const DATETIME_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ToText",
        signature: "ToText([IncludeXmlSchemaFormat: Boolean]): Text",
        summary: "Converts DateTime to text.",
        docs_url: DATETIME_DOCS,
        params: &[BuiltinMethodParam {
            label: "IncludeXmlSchemaFormat: Boolean",
            documentation: "Use XML schema date-time format when true.",
        }],
    },
    BuiltinMethodDoc {
        name: "Date",
        signature: "Date(): Date",
        summary: "Returns Date component.",
        docs_url: DATETIME_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Time",
        signature: "Time(): Time",
        summary: "Returns Time component.",
        docs_url: DATETIME_DOCS,
        params: PARAM_NONE,
    },
];

const DURATION_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "ToText",
    signature: "ToText(): Text",
    summary: "Converts Duration to text.",
    docs_url: DURATION_DOCS,
    params: PARAM_NONE,
}];

const INSTREAM_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Read",
        signature: "Read(var Value: Any [, Length: Integer]): Integer",
        summary: "Reads binary data from stream.",
        docs_url: INSTREAM_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "var Value: Any",
                documentation: "Output value.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional number of bytes to read.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ReadText",
        signature: "ReadText(var Value: Text [, Length: Integer]): Integer",
        summary: "Reads text from stream.",
        docs_url: INSTREAM_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "var Value: Text",
                documentation: "Output text value.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional maximum character count.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "EOS",
        signature: "EOS(): Boolean",
        summary: "Checks whether end of stream is reached.",
        docs_url: INSTREAM_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): BigInteger",
        summary: "Returns stream length in bytes.",
        docs_url: INSTREAM_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Position",
        signature: "Position([NewPosition: BigInteger]): BigInteger",
        summary: "Gets or sets current stream position.",
        docs_url: INSTREAM_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewPosition: BigInteger",
            documentation: "Optional new position.",
        }],
    },
    BuiltinMethodDoc {
        name: "ResetPosition",
        signature: "ResetPosition()",
        summary: "Resets stream position to start.",
        docs_url: INSTREAM_DOCS,
        params: PARAM_NONE,
    },
];

const OUTSTREAM_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Write",
        signature: "Write(Value: Any [, Length: Integer])",
        summary: "Writes binary data to stream.",
        docs_url: OUTSTREAM_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to write.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional number of bytes to write.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "WriteText",
        signature: "WriteText([Value: Text] [, Length: Integer])",
        summary: "Writes text to stream.",
        docs_url: OUTSTREAM_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "Optional text value to write.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional character count.",
            },
        ],
    },
];

const RECORDID_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "TableNo",
        signature: "TableNo(): Integer",
        summary: "Returns the table ID of the RecordId.",
        docs_url: RECORDID_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetRecord",
        signature: "GetRecord(): RecordRef",
        summary: "Gets a RecordRef for the RecordId value.",
        docs_url: RECORDID_DOCS,
        params: PARAM_NONE,
    },
];

const VARIANT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "IsNull",
        signature: "IsNull(): Boolean",
        summary: "Checks whether the Variant has no value.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsRecord",
        signature: "IsRecord(): Boolean",
        summary: "Checks whether the Variant contains a Record.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsRecordRef",
        signature: "IsRecordRef(): Boolean",
        summary: "Checks whether the Variant contains a RecordRef.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsRecordId",
        signature: "IsRecordId(): Boolean",
        summary: "Checks whether the Variant contains a RecordId.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsText",
        signature: "IsText(): Boolean",
        summary: "Checks whether the Variant contains a Text value.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsCode",
        signature: "IsCode(): Boolean",
        summary: "Checks whether the Variant contains a Code value.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsInteger",
        signature: "IsInteger(): Boolean",
        summary: "Checks whether the Variant contains an Integer.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDecimal",
        signature: "IsDecimal(): Boolean",
        summary: "Checks whether the Variant contains a Decimal.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsBoolean",
        signature: "IsBoolean(): Boolean",
        summary: "Checks whether the Variant contains a Boolean.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDate",
        signature: "IsDate(): Boolean",
        summary: "Checks whether the Variant contains a Date.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsTime",
        signature: "IsTime(): Boolean",
        summary: "Checks whether the Variant contains a Time.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDateTime",
        signature: "IsDateTime(): Boolean",
        summary: "Checks whether the Variant contains a DateTime.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDuration",
        signature: "IsDuration(): Boolean",
        summary: "Checks whether the Variant contains a Duration.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsGuid",
        signature: "IsGuid(): Boolean",
        summary: "Checks whether the Variant contains a Guid.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsOption",
        signature: "IsOption(): Boolean",
        summary: "Checks whether the Variant contains an Option value.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsEnum",
        signature: "IsEnum(): Boolean",
        summary: "Checks whether the Variant contains an Enum value.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsList",
        signature: "IsList(): Boolean",
        summary: "Checks whether the Variant contains a List.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsDictionary",
        signature: "IsDictionary(): Boolean",
        summary: "Checks whether the Variant contains a Dictionary.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsJsonObject",
        signature: "IsJsonObject(): Boolean",
        summary: "Checks whether the Variant contains a JsonObject.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsJsonArray",
        signature: "IsJsonArray(): Boolean",
        summary: "Checks whether the Variant contains a JsonArray.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsJsonToken",
        signature: "IsJsonToken(): Boolean",
        summary: "Checks whether the Variant contains a JsonToken.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsJsonValue",
        signature: "IsJsonValue(): Boolean",
        summary: "Checks whether the Variant contains a JsonValue.",
        docs_url: VARIANT_DOCS,
        params: PARAM_NONE,
    },
];

const SESSION_SETTINGS_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Init",
        signature: "Init()",
        summary: "Initializes SessionSettings with current session values.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Company",
        signature: "Company([CompanyName: Text]): Text",
        summary: "Gets or sets the company setting.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: &[BuiltinMethodParam {
            label: "CompanyName: Text",
            documentation: "Optional new company name.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetLanguageId",
        signature: "GetLanguageId(): Integer",
        summary: "Gets session language ID.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LanguageId",
        signature: "LanguageId([LanguageId: Integer])",
        summary: "Gets or sets session language ID.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: &[BuiltinMethodParam {
            label: "LanguageId: Integer",
            documentation: "Optional language ID to set.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetLocaleId",
        signature: "GetLocaleId(): Text",
        summary: "Gets session locale name.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "LocaleId",
        signature: "LocaleId([LocaleName: Text])",
        summary: "Gets or sets session locale name.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: &[BuiltinMethodParam {
            label: "LocaleName: Text",
            documentation: "Optional locale to set.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetTimeZone",
        signature: "GetTimeZone(): Text",
        summary: "Gets session time zone.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TimeZone",
        signature: "TimeZone([TimeZoneName: Text])",
        summary: "Gets or sets session time zone.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: &[BuiltinMethodParam {
            label: "TimeZoneName: Text",
            documentation: "Optional time zone to set.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetRequestSessionUpdate",
        signature: "GetRequestSessionUpdate(): Boolean",
        summary: "Gets whether session update is requested.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RequestSessionUpdate",
        signature: "RequestSessionUpdate([DoUpdate: Boolean])",
        summary: "Gets or sets whether session update is requested.",
        docs_url: SESSION_SETTINGS_DOCS,
        params: &[BuiltinMethodParam {
            label: "DoUpdate: Boolean",
            documentation: "Optional request update flag.",
        }],
    },
];

const NOTIFICATION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Id",
        signature: "Id([NotificationId: Guid]): Guid",
        summary: "Gets or sets notification identifier.",
        docs_url: NOTIFICATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "NotificationId: Guid",
            documentation: "Optional notification ID to assign.",
        }],
    },
    BuiltinMethodDoc {
        name: "Message",
        signature: "Message([Text: Text]): Text",
        summary: "Gets or sets notification message text.",
        docs_url: NOTIFICATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "Text: Text",
            documentation: "Optional message to assign.",
        }],
    },
    BuiltinMethodDoc {
        name: "Scope",
        signature: "Scope([Scope: NotificationScope]): NotificationScope",
        summary: "Gets or sets notification scope.",
        docs_url: NOTIFICATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "Scope: NotificationScope",
            documentation: "Optional notification scope to assign.",
        }],
    },
    BuiltinMethodDoc {
        name: "Send",
        signature: "Send()",
        summary: "Sends notification to the client.",
        docs_url: NOTIFICATION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Recall",
        signature: "Recall()",
        summary: "Recalls a previously sent notification.",
        docs_url: NOTIFICATION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AddAction",
        signature: "AddAction(Caption: Text, CodeunitId: Integer, FunctionName: Text)",
        summary: "Adds an action to the notification.",
        docs_url: NOTIFICATION_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Caption: Text",
                documentation: "Action caption.",
            },
            BuiltinMethodParam {
                label: "CodeunitId: Integer",
                documentation: "Codeunit to invoke.",
            },
            BuiltinMethodParam {
                label: "FunctionName: Text",
                documentation: "Codeunit function name.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "SetData",
        signature: "SetData(Key: Text, Value: Text)",
        summary: "Stores key/value data on the notification.",
        docs_url: NOTIFICATION_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Data key.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "Data value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetData",
        signature: "GetData(Key: Text): Text",
        summary: "Gets stored notification data by key.",
        docs_url: NOTIFICATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "Key: Text",
            documentation: "Data key.",
        }],
    },
];

const DIALOG_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Open",
        signature: "Open(String: Text [, Value1: Any, ...])",
        summary: "Opens a progress dialog.",
        docs_url: DIALOG_DOCS,
        params: &[BuiltinMethodParam {
            label: "String: Text",
            documentation: "Dialog text with placeholders.",
        }],
    },
    BuiltinMethodDoc {
        name: "Update",
        signature: "Update([Number: Integer] [, Value: Any])",
        summary: "Updates dialog text placeholders.",
        docs_url: DIALOG_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Number: Integer",
                documentation: "Placeholder index to update.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Replacement value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes an open dialog.",
        docs_url: DIALOG_DOCS,
        params: PARAM_NONE,
    },
];

const SESSION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "StartSession",
        signature: "StartSession(var SessionId: Integer, CodeunitId: Integer [, CompanyName: Text] [, var Record: Record]): Boolean",
        summary: "Starts a background session.",
        docs_url: SESSION_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "var SessionId: Integer",
                documentation: "Output session identifier.",
            },
            BuiltinMethodParam {
                label: "CodeunitId: Integer",
                documentation: "Codeunit to run.",
            },
            BuiltinMethodParam {
                label: "CompanyName: Text",
                documentation: "Optional company for the new session.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "StopSession",
        signature: "StopSession(SessionId: Integer): Boolean",
        summary: "Stops a running background session.",
        docs_url: SESSION_DOCS,
        params: &[BuiltinMethodParam {
            label: "SessionId: Integer",
            documentation: "Session identifier.",
        }],
    },
    BuiltinMethodDoc {
        name: "IsSessionAlive",
        signature: "IsSessionAlive(SessionId: Integer): Boolean",
        summary: "Checks whether a session is still running.",
        docs_url: SESSION_DOCS,
        params: &[BuiltinMethodParam {
            label: "SessionId: Integer",
            documentation: "Session identifier.",
        }],
    },
    BuiltinMethodDoc {
        name: "CurrentSessionId",
        signature: "CurrentSessionId(): Integer",
        summary: "Returns current session identifier.",
        docs_url: SESSION_DOCS,
        params: PARAM_NONE,
    },
];

const MODULE_INFO_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Id",
        signature: "Id(): Guid",
        summary: "Returns application ID.",
        docs_url: MODULE_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Returns application name.",
        docs_url: MODULE_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Publisher",
        signature: "Publisher(): Text",
        summary: "Returns application publisher.",
        docs_url: MODULE_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AppVersion",
        signature: "AppVersion(): Version",
        summary: "Returns application version.",
        docs_url: MODULE_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DataVersion",
        signature: "DataVersion(): Version",
        summary: "Returns application data version.",
        docs_url: MODULE_INFO_DOCS,
        params: PARAM_NONE,
    },
];

const SECRET_TEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "IsEmpty",
        signature: "IsEmpty(): Boolean",
        summary: "Checks whether SecretText is empty.",
        docs_url: SECRET_TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Unwrap",
        signature: "Unwrap(): Text",
        summary: "Returns SecretText value as plain text.",
        docs_url: SECRET_TEXT_DOCS,
        params: PARAM_NONE,
    },
];

const DATABASE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Commit",
        signature: "Commit()",
        summary: "Ends current write transaction and commits changes.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CompanyName",
        signature: "CompanyName(): Text",
        summary: "Returns current company name.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SessionId",
        signature: "SessionId(): Integer",
        summary: "Returns current session ID.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "UserId",
        signature: "UserId(): Text",
        summary: "Returns current user ID.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "UserSecurityId",
        signature: "UserSecurityId(): Guid",
        summary: "Returns current user security ID.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TenantId",
        signature: "TenantId(): Text",
        summary: "Returns current tenant ID.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SelectLatestVersion",
        signature: "SelectLatestVersion()",
        summary: "Refreshes record state with latest committed data.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsInWriteTransaction",
        signature: "IsInWriteTransaction(): Boolean",
        summary: "Checks whether code is inside a write transaction.",
        docs_url: DATABASE_DOCS,
        params: PARAM_NONE,
    },
];

const SYSTEM_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Abs",
        signature: "Abs(Number: Decimal): Decimal",
        summary: "Returns absolute value.",
        docs_url: SYSTEM_DOCS,
        params: &[BuiltinMethodParam {
            label: "Number: Decimal",
            documentation: "Input numeric value.",
        }],
    },
    BuiltinMethodDoc {
        name: "CalcDate",
        signature: "CalcDate(DateExpression: Text [, Date: Date]): Date",
        summary: "Calculates a date from a date formula expression.",
        docs_url: SYSTEM_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "DateExpression: Text",
                documentation: "Date formula text.",
            },
            BuiltinMethodParam {
                label: "Date: Date",
                documentation: "Optional base date.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Round",
        signature: "Round(Number: Decimal [, Precision: Decimal] [, Direction: Text]): Decimal",
        summary: "Rounds a decimal to given precision and direction.",
        docs_url: SYSTEM_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Number: Decimal",
                documentation: "Input number.",
            },
            BuiltinMethodParam {
                label: "Precision: Decimal",
                documentation: "Optional rounding precision.",
            },
            BuiltinMethodParam {
                label: "Direction: Text",
                documentation: "Optional direction ('='/'>'/'<').",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Format",
        signature: "Format(Value: Any [, Length: Integer] [, FormatNumber: Integer]): Text",
        summary: "Formats a value as text.",
        docs_url: SYSTEM_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Any",
            documentation: "Input value.",
        }],
    },
    BuiltinMethodDoc {
        name: "CurrentDateTime",
        signature: "CurrentDateTime(): DateTime",
        summary: "Returns current date and time.",
        docs_url: SYSTEM_DOCS,
        params: PARAM_NONE,
    },
];

const SESSION_INFORMATION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ClientType",
        signature: "ClientType(): ClientType",
        summary: "Returns current client type.",
        docs_url: SESSION_INFORMATION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SessionId",
        signature: "SessionId(): Integer",
        summary: "Returns session ID.",
        docs_url: SESSION_INFORMATION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "UserSecurityId",
        signature: "UserSecurityId(): Guid",
        summary: "Returns current user security ID.",
        docs_url: SESSION_INFORMATION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ServerInstanceId",
        signature: "ServerInstanceId(): Integer",
        summary: "Returns service tier instance ID.",
        docs_url: SESSION_INFORMATION_DOCS,
        params: PARAM_NONE,
    },
];

const TASK_SCHEDULER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CreateTask",
        signature: "CreateTask(CodeunitId: Integer [, RecordId: RecordId] [, NotBefore: DateTime] [, Name: Text]): Guid",
        summary: "Creates a scheduled task.",
        docs_url: TASK_SCHEDULER_DOCS,
        params: &[BuiltinMethodParam {
            label: "CodeunitId: Integer",
            documentation: "Codeunit to execute.",
        }],
    },
    BuiltinMethodDoc {
        name: "CancelTask",
        signature: "CancelTask(TaskId: Guid): Boolean",
        summary: "Cancels a scheduled task.",
        docs_url: TASK_SCHEDULER_DOCS,
        params: &[BuiltinMethodParam {
            label: "TaskId: Guid",
            documentation: "Task identifier.",
        }],
    },
    BuiltinMethodDoc {
        name: "TaskExists",
        signature: "TaskExists(TaskId: Guid): Boolean",
        summary: "Checks whether a task exists.",
        docs_url: TASK_SCHEDULER_DOCS,
        params: &[BuiltinMethodParam {
            label: "TaskId: Guid",
            documentation: "Task identifier.",
        }],
    },
    BuiltinMethodDoc {
        name: "CanCreateTask",
        signature: "CanCreateTask(): Boolean",
        summary: "Checks whether current context can create tasks.",
        docs_url: TASK_SCHEDULER_DOCS,
        params: PARAM_NONE,
    },
];

const FILTER_PAGE_BUILDER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddRecord",
        signature: "AddRecord(Name: Text, Record: Record)",
        summary: "Adds a record to filter page builder.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Filter page control name.",
            },
            BuiltinMethodParam {
                label: "Record: Record",
                documentation: "Record reference for filtering.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddField",
        signature: "AddField(Name: Text, Field: Any)",
        summary: "Adds a field to filter page.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Filter page control name.",
            },
            BuiltinMethodParam {
                label: "Field: Any",
                documentation: "Field reference.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "RunModal",
        signature: "RunModal(): Boolean",
        summary: "Runs filter page and returns true if user confirms.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetView",
        signature: "GetView(Name: Text): Text",
        summary: "Gets resulting table view for added record.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Filter page control name.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetView",
        signature: "SetView(Name: Text, View: Text)",
        summary: "Sets table view for added record.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Filter page control name.",
            },
            BuiltinMethodParam {
                label: "View: Text",
                documentation: "View string.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of controls in builder.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(Index: Integer): Text",
        summary: "Gets control name by index.",
        docs_url: FILTER_PAGE_BUILDER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "1-based index.",
        }],
    },
];

const BLOB_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CreateInStream",
        signature: "CreateInStream(var InStream: InStream)",
        summary: "Creates an InStream from Blob value.",
        docs_url: BLOB_DOCS,
        params: &[BuiltinMethodParam {
            label: "var InStream: InStream",
            documentation: "Output input stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "CreateOutStream",
        signature: "CreateOutStream(var OutStream: OutStream)",
        summary: "Creates an OutStream for Blob value.",
        docs_url: BLOB_DOCS,
        params: &[BuiltinMethodParam {
            label: "var OutStream: OutStream",
            documentation: "Output output stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "HasValue",
        signature: "HasValue(): Boolean",
        summary: "Checks whether Blob contains data.",
        docs_url: BLOB_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): Integer",
        summary: "Returns Blob length in bytes.",
        docs_url: BLOB_DOCS,
        params: PARAM_NONE,
    },
];

const FILE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Open",
        signature: "Open(Name: Text)",
        summary: "Opens a file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "File name/path.",
        }],
    },
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Name: Text)",
        summary: "Creates a file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "File name/path.",
        }],
    },
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes current file handle.",
        docs_url: FILE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "CreateInStream",
        signature: "CreateInStream(var InStream: InStream)",
        summary: "Creates InStream from file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var InStream: InStream",
            documentation: "Output stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "CreateOutStream",
        signature: "CreateOutStream(var OutStream: OutStream)",
        summary: "Creates OutStream for file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var OutStream: OutStream",
            documentation: "Output stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "Read",
        signature: "Read(var Value: Any): Integer",
        summary: "Reads data from file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Value: Any",
            documentation: "Output value.",
        }],
    },
    BuiltinMethodDoc {
        name: "Write",
        signature: "Write(Value: Any)",
        summary: "Writes data to file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Any",
            documentation: "Value to write.",
        }],
    },
    BuiltinMethodDoc {
        name: "Exists",
        signature: "Exists(Name: Text): Boolean",
        summary: "Checks whether file exists.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "File name/path.",
        }],
    },
    BuiltinMethodDoc {
        name: "Erase",
        signature: "Erase(Name: Text)",
        summary: "Deletes a file.",
        docs_url: FILE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "File name/path.",
        }],
    },
    BuiltinMethodDoc {
        name: "Rename",
        signature: "Rename(OldName: Text, NewName: Text)",
        summary: "Renames a file.",
        docs_url: FILE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "OldName: Text",
                documentation: "Current file name/path.",
            },
            BuiltinMethodParam {
                label: "NewName: Text",
                documentation: "New file name/path.",
            },
        ],
    },
];

const VERSION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(VersionText: Text): Version",
        summary: "Creates a Version from text.",
        docs_url: VERSION_DOCS,
        params: &[BuiltinMethodParam {
            label: "VersionText: Text",
            documentation: "Version text.",
        }],
    },
    BuiltinMethodDoc {
        name: "ToText",
        signature: "ToText(): Text",
        summary: "Converts Version to text.",
        docs_url: VERSION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Major",
        signature: "Major(): Integer",
        summary: "Returns major version component.",
        docs_url: VERSION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Minor",
        signature: "Minor(): Integer",
        summary: "Returns minor version component.",
        docs_url: VERSION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Build",
        signature: "Build(): Integer",
        summary: "Returns build version component.",
        docs_url: VERSION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Revision",
        signature: "Revision(): Integer",
        summary: "Returns revision version component.",
        docs_url: VERSION_DOCS,
        params: PARAM_NONE,
    },
];

const MODULE_DEPENDENCY_INFO_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Id",
        signature: "Id(): Guid",
        summary: "Returns dependency app ID.",
        docs_url: MODULE_DEPENDENCY_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Returns dependency app name.",
        docs_url: MODULE_DEPENDENCY_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Publisher",
        signature: "Publisher(): Text",
        summary: "Returns dependency app publisher.",
        docs_url: MODULE_DEPENDENCY_INFO_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AppVersion",
        signature: "AppVersion(): Version",
        summary: "Returns dependency app version.",
        docs_url: MODULE_DEPENDENCY_INFO_DOCS,
        params: PARAM_NONE,
    },
];

const NAV_APP_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "GetCurrentModuleInfo",
        signature: "GetCurrentModuleInfo(var ModuleInfo: ModuleInfo)",
        summary: "Gets information about current extension module.",
        docs_url: NAV_APP_DOCS,
        params: &[BuiltinMethodParam {
            label: "var ModuleInfo: ModuleInfo",
            documentation: "Output module information.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetModuleInfo",
        signature: "GetModuleInfo(AppId: Guid, var ModuleInfo: ModuleInfo): Boolean",
        summary: "Gets module info for a specific app.",
        docs_url: NAV_APP_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "AppId: Guid",
                documentation: "App identifier.",
            },
            BuiltinMethodParam {
                label: "var ModuleInfo: ModuleInfo",
                documentation: "Output module information.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetResourceAsStream",
        signature:
            "GetResourceAsStream(AppId: Guid, ResourceName: Text, var InStream: InStream): Boolean",
        summary: "Reads an app resource as stream.",
        docs_url: NAV_APP_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "AppId: Guid",
                documentation: "App identifier.",
            },
            BuiltinMethodParam {
                label: "ResourceName: Text",
                documentation: "Resource name.",
            },
            BuiltinMethodParam {
                label: "var InStream: InStream",
                documentation: "Output resource stream.",
            },
        ],
    },
];

const NUMBER_SEQUENCE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Insert",
        signature: "Insert(Code: Code[20], StartingValue: BigInteger [, IncrementBy: Integer])",
        summary: "Creates a number sequence.",
        docs_url: NUMBER_SEQUENCE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Code: Code[20]",
                documentation: "Sequence code.",
            },
            BuiltinMethodParam {
                label: "StartingValue: BigInteger",
                documentation: "Initial value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Next",
        signature: "Next(Code: Code[20]): BigInteger",
        summary: "Gets next sequence number.",
        docs_url: NUMBER_SEQUENCE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Code: Code[20]",
            documentation: "Sequence code.",
        }],
    },
    BuiltinMethodDoc {
        name: "Current",
        signature: "Current(Code: Code[20]): BigInteger",
        summary: "Gets current sequence value.",
        docs_url: NUMBER_SEQUENCE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Code: Code[20]",
            documentation: "Sequence code.",
        }],
    },
];

const TEXT_BUILDER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Append",
        signature: "Append(Value: Text): TextBuilder",
        summary: "Appends text to builder.",
        docs_url: TEXT_BUILDER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Text to append.",
        }],
    },
    BuiltinMethodDoc {
        name: "AppendLine",
        signature: "AppendLine([Value: Text]): TextBuilder",
        summary: "Appends text and line ending.",
        docs_url: TEXT_BUILDER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Optional text to append.",
        }],
    },
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear()",
        summary: "Clears builder content.",
        docs_url: TEXT_BUILDER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): Integer",
        summary: "Returns current content length.",
        docs_url: TEXT_BUILDER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ToText",
        signature: "ToText(): Text",
        summary: "Converts builder content to text.",
        docs_url: TEXT_BUILDER_DOCS,
        params: PARAM_NONE,
    },
];

const MEDIA_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ImportStream",
        signature: "ImportStream(InStream: InStream, Description: Text [, MimeType: Text])",
        summary: "Imports media content from a stream.",
        docs_url: MEDIA_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "Input content stream.",
            },
            BuiltinMethodParam {
                label: "Description: Text",
                documentation: "Media description.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ImportFile",
        signature: "ImportFile(FileName: Text [, Description: Text])",
        summary: "Imports media content from a file.",
        docs_url: MEDIA_DOCS,
        params: &[BuiltinMethodParam {
            label: "FileName: Text",
            documentation: "File path.",
        }],
    },
    BuiltinMethodDoc {
        name: "ExportStream",
        signature: "ExportStream(var OutStream: OutStream)",
        summary: "Exports media content to a stream.",
        docs_url: MEDIA_DOCS,
        params: &[BuiltinMethodParam {
            label: "var OutStream: OutStream",
            documentation: "Output stream.",
        }],
    },
    BuiltinMethodDoc {
        name: "ExportFile",
        signature: "ExportFile(FileName: Text)",
        summary: "Exports media content to a file.",
        docs_url: MEDIA_DOCS,
        params: &[BuiltinMethodParam {
            label: "FileName: Text",
            documentation: "File path.",
        }],
    },
    BuiltinMethodDoc {
        name: "HasValue",
        signature: "HasValue(): Boolean",
        summary: "Checks whether media has content.",
        docs_url: MEDIA_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "MediaId",
        signature: "MediaId(): Guid",
        summary: "Returns media identifier.",
        docs_url: MEDIA_DOCS,
        params: PARAM_NONE,
    },
];

const MEDIA_SET_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ImportStream",
        signature: "ImportStream(InStream: InStream, Description: Text [, MimeType: Text])",
        summary: "Appends media from stream to mediaset.",
        docs_url: MEDIA_SET_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "InStream: InStream",
                documentation: "Input content stream.",
            },
            BuiltinMethodParam {
                label: "Description: Text",
                documentation: "Media description.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ImportFile",
        signature: "ImportFile(FileName: Text [, Description: Text])",
        summary: "Appends media from file to mediaset.",
        docs_url: MEDIA_SET_DOCS,
        params: &[BuiltinMethodParam {
            label: "FileName: Text",
            documentation: "File path.",
        }],
    },
    BuiltinMethodDoc {
        name: "Item",
        signature: "Item(Index: Integer): Media",
        summary: "Gets media item by index.",
        docs_url: MEDIA_SET_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "1-based media index.",
        }],
    },
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of media items.",
        docs_url: MEDIA_SET_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Index: Integer)",
        summary: "Removes media item by index.",
        docs_url: MEDIA_SET_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "1-based media index.",
        }],
    },
    BuiltinMethodDoc {
        name: "Clear",
        signature: "Clear()",
        summary: "Clears all media items from mediaset.",
        docs_url: MEDIA_SET_DOCS,
        params: PARAM_NONE,
    },
];

const ERROR_INFO_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Message",
        signature: "Message([Text: Text]): Text",
        summary: "Gets or sets the top-level error message.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "Text: Text",
            documentation: "Optional message value.",
        }],
    },
    BuiltinMethodDoc {
        name: "DetailedMessage",
        signature: "DetailedMessage([Text: Text]): Text",
        summary: "Gets or sets detailed error message text.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "Text: Text",
            documentation: "Optional detailed message.",
        }],
    },
    BuiltinMethodDoc {
        name: "Verbosity",
        signature: "Verbosity([Verbosity: Verbosity]): Verbosity",
        summary: "Gets or sets error verbosity.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "Verbosity: Verbosity",
            documentation: "Optional verbosity level.",
        }],
    },
    BuiltinMethodDoc {
        name: "RecordId",
        signature: "RecordId([RecordId: RecordId]): RecordId",
        summary: "Gets or sets related RecordId.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "RecordId: RecordId",
            documentation: "Optional related record id.",
        }],
    },
    BuiltinMethodDoc {
        name: "FieldNo",
        signature: "FieldNo([FieldNo: Integer]): Integer",
        summary: "Gets or sets related field number.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "FieldNo: Integer",
            documentation: "Optional related field number.",
        }],
    },
    BuiltinMethodDoc {
        name: "PageNo",
        signature: "PageNo([PageNo: Integer]): Integer",
        summary: "Gets or sets related page number.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "PageNo: Integer",
            documentation: "Optional related page number.",
        }],
    },
    BuiltinMethodDoc {
        name: "Collectible",
        signature: "Collectible([Collectible: Boolean]): Boolean",
        summary: "Gets or sets collectible flag.",
        docs_url: ERROR_INFO_DOCS,
        params: &[BuiltinMethodParam {
            label: "Collectible: Boolean",
            documentation: "Optional collectible flag.",
        }],
    },
    BuiltinMethodDoc {
        name: "AddNavigationAction",
        signature: "AddNavigationAction(Caption: Text, CodeunitId: Integer, FunctionName: Text)",
        summary: "Adds navigation action to error info.",
        docs_url: ERROR_INFO_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Caption: Text",
                documentation: "Action caption.",
            },
            BuiltinMethodParam {
                label: "CodeunitId: Integer",
                documentation: "Target codeunit id.",
            },
            BuiltinMethodParam {
                label: "FunctionName: Text",
                documentation: "Function to invoke.",
            },
        ],
    },
];

const TEST_PAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Cancel",
        signature: "Cancel(): TestAction",
        summary: "Gets the Cancel system action.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption(): Text",
        summary: "Gets the caption of the test page.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Close",
        signature: "Close()",
        summary: "Closes an open test page.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Edit",
        signature: "Edit(): TestAction",
        summary: "Gets the Edit system action.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Editable",
        signature: "Editable(): Boolean",
        summary: "Gets the runtime value of the Editable property on a test page.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Expand",
        signature: "Expand(SetExpand: Boolean)",
        summary: "Expands rows on a test page.",
        docs_url: TEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "SetExpand: Boolean",
            documentation: "true to expand rows; false to collapse rows.",
        }],
    },
    BuiltinMethodDoc {
        name: "FindFirstField",
        signature: "FindFirstField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the first matching field value in the current data set.",
        docs_url: TEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindNextField",
        signature: "FindNextField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the next matching field value in the current data set.",
        docs_url: TEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindPreviousField",
        signature: "FindPreviousField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the previous matching field value in the current data set.",
        docs_url: TEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "First",
        signature: "First(): Boolean",
        summary: "Sets the current row as the first row in the data set.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetField",
        signature: "GetField(Id: Integer): TestField",
        summary: "Gets a field on a test page by control identifier.",
        docs_url: TEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Id: Integer",
            documentation: "Control identifier for the field.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetValidationError",
        signature: "GetValidationError([Index: Integer]): Text",
        summary: "Gets validation error text that occurred on the test page.",
        docs_url: TEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Optional validation error index.",
        }],
    },
    BuiltinMethodDoc {
        name: "GoToKey",
        signature: "GoToKey([Value1: Any, ...]): Boolean",
        summary: "Finds the row identified by the specified key values.",
        docs_url: TEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value1: Any",
            documentation: "First key value to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "GoToRecord",
        signature: "GoToRecord(Rec: Record): Boolean",
        summary: "Finds the specified record in the test page data set.",
        docs_url: TEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Rec: Record",
            documentation: "Record to locate.",
        }],
    },
    BuiltinMethodDoc {
        name: "IsExpanded",
        signature: "IsExpanded(): Boolean",
        summary: "Specifies whether rows are currently expanded.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Last",
        signature: "Last(): Boolean",
        summary: "Sets the current row as the last row in the data set.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "New",
        signature: "New(): Boolean",
        summary: "Sets the current row to an empty row in the data set.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OpenView",
        signature: "OpenView()",
        summary: "Opens a test page in view mode.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OpenEdit",
        signature: "OpenEdit()",
        summary: "Opens a test page in edit mode.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OpenNew",
        signature: "OpenNew()",
        summary: "Opens a blank test page in edit mode.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Next",
        signature: "Next(): Boolean",
        summary: "Sets the current row as the next row in the data set.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "No",
        signature: "No(): TestAction",
        summary: "Gets the No system action.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OK",
        signature: "OK(): TestAction",
        summary: "Gets the OK system action.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Prev",
        signature: "Prev(): Boolean",
        summary: "Sets the current row as the previous row in the data set.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Previous",
        signature: "Previous(): Boolean",
        summary: "Sets the current row as the previous row in the data set.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RunPageBackgroundTask",
        signature:
            "RunPageBackgroundTask(CodeunitId: Integer [, var Parameters: Dictionary of [Text, Text]] [, RunCompletionTriggers: Boolean])",
        summary: "Runs a page background task codeunit in a child session.",
        docs_url: TEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "CodeunitId: Integer",
            documentation: "Codeunit ID to run as the page background task.",
        }],
    },
    BuiltinMethodDoc {
        name: "Trap",
        signature: "Trap()",
        summary: "Traps the next page instance for test interaction.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ValidationErrorCount",
        signature: "ValidationErrorCount(): Integer",
        summary: "Gets the number of validation errors on the test page.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "View",
        signature: "View(): TestAction",
        summary: "Gets the View system action.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Yes",
        signature: "Yes(): TestAction",
        summary: "Gets the Yes system action.",
        docs_url: TEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
];

const TEST_FIELD_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Activate",
        signature: "Activate()",
        summary: "Activates a field on a test page.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsBoolean",
        signature: "AsBoolean(): Boolean",
        summary: "Converts the field value to Boolean.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDate",
        signature: "AsDate(): Date",
        summary: "Converts the field value to Date.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDateTime",
        signature: "AsDateTime(): DateTime",
        summary: "Converts the field value to DateTime.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsDecimal",
        signature: "AsDecimal(): Decimal",
        summary: "Converts the field value to Decimal.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsInteger",
        signature: "AsInteger(): Integer",
        summary: "Converts the field value to Integer.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AsTime",
        signature: "AsTime(): Time",
        summary: "Converts the field value to Time.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "AssertEquals",
        signature: "AssertEquals(Expected: Any)",
        summary: "Asserts that the field value equals the expected value.",
        docs_url: TEST_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Expected: Any",
            documentation: "Expected field value.",
        }],
    },
    BuiltinMethodDoc {
        name: "AssistEdit",
        signature: "AssistEdit()",
        summary: "Runs assist-edit for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption(): Text",
        summary: "Gets the current field caption.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "DrillDown",
        signature: "DrillDown()",
        summary: "Runs drill-down behavior for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Editable",
        signature: "Editable(): Boolean",
        summary: "Gets the editable state for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Enabled",
        signature: "Enabled(): Boolean",
        summary: "Gets the enabled state for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetOption",
        signature: "GetOption([Index: Integer]): Text",
        summary: "Gets an option caption for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Optional option index.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetValidationError",
        signature: "GetValidationError([Index: Integer]): Text",
        summary: "Gets validation error text for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Optional validation error index.",
        }],
    },
    BuiltinMethodDoc {
        name: "HideValue",
        signature: "HideValue(): Boolean",
        summary: "Gets the hide-value state for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Invoke",
        signature: "Invoke()",
        summary: "Invokes the default action on the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Lookup",
        signature: "Lookup()",
        summary: "Opens lookup for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Lookup",
        signature: "Lookup(RecRef: RecordRef)",
        summary: "Runs lookup and returns the selected record reference.",
        docs_url: TEST_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "RecRef: RecordRef",
            documentation: "Record reference for the selected record.",
        }],
    },
    BuiltinMethodDoc {
        name: "OptionCount",
        signature: "OptionCount(): Integer",
        summary: "Gets the number of options available for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetValue",
        signature: "SetValue(Value: Any)",
        summary: "Sets the field value in the test page.",
        docs_url: TEST_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Any",
            documentation: "Value to assign.",
        }],
    },
    BuiltinMethodDoc {
        name: "ShowMandatory",
        signature: "ShowMandatory(): Boolean",
        summary: "Gets the ShowMandatory state for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ValidationErrorCount",
        signature: "ValidationErrorCount(): Integer",
        summary: "Gets the number of validation errors for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value([Value: Text]): Text",
        summary: "Gets or sets the field value.",
        docs_url: TEST_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Optional value to assign before reading.",
        }],
    },
    BuiltinMethodDoc {
        name: "Visible",
        signature: "Visible(): Boolean",
        summary: "Gets the visible state for the field.",
        docs_url: TEST_FIELD_DOCS,
        params: PARAM_NONE,
    },
];

const TEST_ACTION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Invoke",
        signature: "Invoke()",
        summary: "Invokes test page action.",
        docs_url: TEST_ACTION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Visible",
        signature: "Visible(): Boolean",
        summary: "Checks whether action is visible.",
        docs_url: TEST_ACTION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Enabled",
        signature: "Enabled(): Boolean",
        summary: "Checks whether action is enabled.",
        docs_url: TEST_ACTION_DOCS,
        params: PARAM_NONE,
    },
];

const TEST_REQUEST_PAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Cancel",
        signature: "Cancel(): TestAction",
        summary: "Gets a TestAction for the Cancel action.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption(): Text",
        summary: "Gets the caption of the test request page.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Editable",
        signature: "Editable(): Boolean",
        summary: "Gets the runtime value of the Editable property.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Expand",
        signature: "Expand(SetExpand: Boolean)",
        summary: "Expands rows on a test request page.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "SetExpand: Boolean",
            documentation: "true to expand rows; false to collapse rows.",
        }],
    },
    BuiltinMethodDoc {
        name: "FindFirstField",
        signature: "FindFirstField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the first matching field value.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindNextField",
        signature: "FindNextField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the next matching field value.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindPreviousField",
        signature: "FindPreviousField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the previous matching field value.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "First",
        signature: "First(): Boolean",
        summary: "Sets the current row as the first row in the data set.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetValidationError",
        signature: "GetValidationError([Index: Integer]): Text",
        summary: "Gets validation error text that occurred on the test request page.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Optional validation error index.",
        }],
    },
    BuiltinMethodDoc {
        name: "GoToKey",
        signature: "GoToKey([Value1: Any, ...]): Boolean",
        summary: "Finds the row identified by the specified key values.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value1: Any",
            documentation: "First key value to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "GoToRecord",
        signature: "GoToRecord(Rec: Record): Boolean",
        summary: "Finds the specified record in the request page data set.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Rec: Record",
            documentation: "Record to locate.",
        }],
    },
    BuiltinMethodDoc {
        name: "IsExpanded",
        signature: "IsExpanded(): Boolean",
        summary: "Specifies whether rows are currently expanded.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Last",
        signature: "Last(): Boolean",
        summary: "Sets the current row as the last row in the data set.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "New",
        signature: "New(): Boolean",
        summary: "Sets the current row to an empty row in the data set.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Next",
        signature: "Next(): Boolean",
        summary: "Sets the current row as the next row in the data set.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OK",
        signature: "OK(): TestAction",
        summary: "Gets a TestAction for the OK action.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Preview",
        signature: "Preview(): TestAction",
        summary: "Gets a TestAction for the Preview action.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Previous",
        signature: "Previous(): Boolean",
        summary: "Sets the current row as the previous row in the data set.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Print",
        signature: "Print(): TestAction",
        summary: "Gets a TestAction for the Print action.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SaveAsExcel",
        signature: "SaveAsExcel(FileName: Text)",
        summary: "Saves the report output as an Excel file.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "FileName: Text",
            documentation: "Output file path.",
        }],
    },
    BuiltinMethodDoc {
        name: "SaveAsPdf",
        signature: "SaveAsPdf(FileName: Text)",
        summary: "Saves the report output as a PDF file.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "FileName: Text",
            documentation: "Output file path.",
        }],
    },
    BuiltinMethodDoc {
        name: "SaveAsWord",
        signature: "SaveAsWord(FileName: Text)",
        summary: "Saves the report output as a Word file.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "FileName: Text",
            documentation: "Output file path.",
        }],
    },
    BuiltinMethodDoc {
        name: "SaveAsXml",
        signature: "SaveAsXml(DataSetFileName: Text, LabelsFileName: Text)",
        summary: "Saves report data set and labels as XML files.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "DataSetFileName: Text",
                documentation: "Output file path for dataset XML.",
            },
            BuiltinMethodParam {
                label: "LabelsFileName: Text",
                documentation: "Output file path for labels XML.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Schedule",
        signature: "Schedule(): TestAction",
        summary: "Gets a TestAction for the Schedule action.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ValidationErrorCount",
        signature: "ValidationErrorCount(): Integer",
        summary: "Gets the number of validation errors on the request page.",
        docs_url: TEST_REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
];

const TEST_PART_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption(): Text",
        summary: "Gets the caption of the part.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Editable",
        signature: "Editable(): Boolean",
        summary: "Gets the editable state for the part.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Enabled",
        signature: "Enabled(): Boolean",
        summary: "Gets the enabled state for the part.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Expand",
        signature: "Expand(SetExpand: Boolean)",
        summary: "Expands rows in the part.",
        docs_url: TEST_PART_DOCS,
        params: &[BuiltinMethodParam {
            label: "SetExpand: Boolean",
            documentation: "true to expand rows; false to collapse rows.",
        }],
    },
    BuiltinMethodDoc {
        name: "FindFirstField",
        signature: "FindFirstField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the first matching field value.",
        docs_url: TEST_PART_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindNextField",
        signature: "FindNextField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the next matching field value.",
        docs_url: TEST_PART_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "FindPreviousField",
        signature: "FindPreviousField(Field: TestField, Value: Any): Boolean",
        summary: "Finds the previous matching field value.",
        docs_url: TEST_PART_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestField",
                documentation: "Field to search.",
            },
            BuiltinMethodParam {
                label: "Value: Any",
                documentation: "Value to match.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "First",
        signature: "First(): Boolean",
        summary: "Sets the current row as the first row in the data set.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetField",
        signature: "GetField(Id: Integer): TestField",
        summary: "Gets a field on the part by control identifier.",
        docs_url: TEST_PART_DOCS,
        params: &[BuiltinMethodParam {
            label: "Id: Integer",
            documentation: "Control identifier for the field.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetValidationError",
        signature: "GetValidationError([Index: Integer]): Text",
        summary: "Gets validation error text for the part.",
        docs_url: TEST_PART_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Optional validation error index.",
        }],
    },
    BuiltinMethodDoc {
        name: "GoToKey",
        signature: "GoToKey([Value1: Any, ...]): Boolean",
        summary: "Finds the row identified by the specified key values.",
        docs_url: TEST_PART_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value1: Any",
            documentation: "First key value to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "GoToRecord",
        signature: "GoToRecord(Rec: Record): Boolean",
        summary: "Finds the specified record in the part data set.",
        docs_url: TEST_PART_DOCS,
        params: &[BuiltinMethodParam {
            label: "Rec: Record",
            documentation: "Record to locate.",
        }],
    },
    BuiltinMethodDoc {
        name: "IsExpanded",
        signature: "IsExpanded(): Boolean",
        summary: "Specifies whether rows are currently expanded.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Last",
        signature: "Last(): Boolean",
        summary: "Sets the current row as the last row in the data set.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "New",
        signature: "New(): Boolean",
        summary: "Sets the current row to an empty row in the data set.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Next",
        signature: "Next(): Boolean",
        summary: "Sets the current row as the next row in the data set.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Prev",
        signature: "Prev(): Boolean",
        summary: "Sets the current row as the previous row in the data set.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Previous",
        signature: "Previous(): Boolean",
        summary: "Sets the current row as the previous row in the data set.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ValidationErrorCount",
        signature: "ValidationErrorCount(): Integer",
        summary: "Gets the number of validation errors for the part.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Visible",
        signature: "Visible(): Boolean",
        summary: "Gets the visible state for the part.",
        docs_url: TEST_PART_DOCS,
        params: PARAM_NONE,
    },
];

const TEST_FILTER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Ascending",
        signature: "Ascending([Value: Boolean]): Boolean",
        summary: "Gets or sets ascending order for dataset traversal.",
        docs_url: TEST_FILTER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Boolean",
            documentation: "Optional value for ascending sort mode.",
        }],
    },
    BuiltinMethodDoc {
        name: "CurrentKey",
        signature: "CurrentKey(): Text",
        summary: "Gets the current key of the filtered dataset.",
        docs_url: TEST_FILTER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetFilter",
        signature: "GetFilter(Field: TestFilterField): Text",
        summary: "Gets the filter applied to a specified filter field.",
        docs_url: TEST_FILTER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field: TestFilterField",
            documentation: "Filter field identifier.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetCurrentKey",
        signature: "SetCurrentKey(Field1: TestFilterField [, Field2: TestFilterField, ...])",
        summary: "Sets current key fields for dataset traversal.",
        docs_url: TEST_FILTER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Field1: TestFilterField",
            documentation: "First key field.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetFilter",
        signature: "SetFilter(Field: TestFilterField, Filter: Text)",
        summary: "Applies a filter to the specified filter field.",
        docs_url: TEST_FILTER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Field: TestFilterField",
                documentation: "Filter field identifier.",
            },
            BuiltinMethodParam {
                label: "Filter: Text",
                documentation: "Filter expression.",
            },
        ],
    },
];

const ANY_METHODS: &[BuiltinMethodDoc] = &[];

const BIG_INTEGER_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "ToText",
    signature: "ToText([Length: Integer] [, FormatNumber: Integer]): Text",
    summary: "Converts the BigInteger value to formatted text.",
    docs_url: BIG_INTEGER_DOCS,
    params: &[
        BuiltinMethodParam {
            label: "Length: Integer",
            documentation: "Optional minimum output length.",
        },
        BuiltinMethodParam {
            label: "FormatNumber: Integer",
            documentation: "Optional format number.",
        },
    ],
}];

const BOOLEAN_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "ToText",
    signature: "ToText([Length: Integer] [, FormatNumber: Integer]): Text",
    summary: "Converts the Boolean value to formatted text.",
    docs_url: BOOLEAN_DOCS,
    params: &[
        BuiltinMethodParam {
            label: "Length: Integer",
            documentation: "Optional minimum output length.",
        },
        BuiltinMethodParam {
            label: "FormatNumber: Integer",
            documentation: "Optional format number.",
        },
    ],
}];

const BYTE_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "ToText",
    signature: "ToText([Length: Integer] [, FormatNumber: Integer]): Text",
    summary: "Converts the Byte value to formatted text.",
    docs_url: BYTE_DOCS,
    params: &[
        BuiltinMethodParam {
            label: "Length: Integer",
            documentation: "Optional minimum output length.",
        },
        BuiltinMethodParam {
            label: "FormatNumber: Integer",
            documentation: "Optional format number.",
        },
    ],
}];

const CHAR_METHODS: &[BuiltinMethodDoc] = &[];

const COMPANY_PROPERTY_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "DisplayName",
        signature: "DisplayName(): Text",
        summary: "Returns the current company display name.",
        docs_url: COMPANY_PROPERTY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Id",
        signature: "Id(): Guid",
        summary: "Returns the current company identifier.",
        docs_url: COMPANY_PROPERTY_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Returns the current company name.",
        docs_url: COMPANY_PROPERTY_DOCS,
        params: PARAM_NONE,
    },
];

const COOKIE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Domain",
        signature: "Domain(): Text",
        summary: "Returns the cookie domain.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Expires",
        signature: "Expires(): DateTime",
        summary: "Returns cookie expiration date and time.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HttpOnly",
        signature: "HttpOnly(): Boolean",
        summary: "Returns whether HttpOnly is enabled.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Returns the cookie name.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Path",
        signature: "Path(): Text",
        summary: "Returns the cookie path.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Secure",
        signature: "Secure(): Boolean",
        summary: "Returns whether Secure is enabled.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value(): Text",
        summary: "Returns the cookie value.",
        docs_url: COOKIE_DOCS,
        params: PARAM_NONE,
    },
];

const DATA_TRANSFER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddConstantValue",
        signature: "AddConstantValue(SourceFieldNo: Integer, Value: Text)",
        summary: "Adds a constant source value used during copy.",
        docs_url: DATA_TRANSFER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "SourceFieldNo: Integer",
                documentation: "Source field number.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "Constant value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddFieldValue",
        signature: "AddFieldValue(SourceFieldNo: Integer, DestinationFieldNo: Integer)",
        summary: "Maps source and destination fields for transfer.",
        docs_url: DATA_TRANSFER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "SourceFieldNo: Integer",
                documentation: "Source field number.",
            },
            BuiltinMethodParam {
                label: "DestinationFieldNo: Integer",
                documentation: "Destination field number.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddJoin",
        signature: "AddJoin(SourceFieldNo: Integer, DestinationFieldNo: Integer)",
        summary: "Adds a join between source and destination fields.",
        docs_url: DATA_TRANSFER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "SourceFieldNo: Integer",
                documentation: "Source field number.",
            },
            BuiltinMethodParam {
                label: "DestinationFieldNo: Integer",
                documentation: "Destination field number.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AddSourceFilter",
        signature: "AddSourceFilter(SourceFieldNo: Integer, Filter: Text)",
        summary: "Adds a source filter expression.",
        docs_url: DATA_TRANSFER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "SourceFieldNo: Integer",
                documentation: "Source field number.",
            },
            BuiltinMethodParam {
                label: "Filter: Text",
                documentation: "Filter expression.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "CopyFields",
        signature: "CopyFields()",
        summary: "Copies configured source fields to destination.",
        docs_url: DATA_TRANSFER_DOCS,
        params: PARAM_NONE,
    },
];

const DATE_FORMULA_METHODS: &[BuiltinMethodDoc] = &[];

const DEBUGGER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Break",
        signature: "Break([Condition: Boolean])",
        summary: "Requests a debugger break when condition is true.",
        docs_url: DEBUGGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Condition: Boolean",
            documentation: "Optional break condition.",
        }],
    },
    BuiltinMethodDoc {
        name: "IsAttached",
        signature: "IsAttached(): Boolean",
        summary: "Returns whether a debugger is attached.",
        docs_url: DEBUGGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "BreakOnError",
        signature: "BreakOnError([Enabled: Boolean])",
        summary: "Enables or disables break-on-error.",
        docs_url: DEBUGGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Enabled: Boolean",
            documentation: "Optional enabled flag.",
        }],
    },
    BuiltinMethodDoc {
        name: "BreakOnNext",
        signature: "BreakOnNext()",
        summary: "Breaks on the next AL statement.",
        docs_url: DEBUGGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "BreakOnNextSession",
        signature: "BreakOnNextSession()",
        summary: "Breaks when the next session starts.",
        docs_url: DEBUGGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "BreakOnRecordWrite",
        signature: "BreakOnRecordWrite(TableNo: Integer)",
        summary: "Breaks on write operations for a table.",
        docs_url: DEBUGGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "TableNo: Integer",
            documentation: "Table number to watch.",
        }],
    },
    BuiltinMethodDoc {
        name: "BreakOnRecordWriteWithTrigger",
        signature: "BreakOnRecordWriteWithTrigger(TableNo: Integer)",
        summary: "Breaks on writes that run table triggers.",
        docs_url: DEBUGGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "TableNo: Integer",
            documentation: "Table number to watch.",
        }],
    },
    BuiltinMethodDoc {
        name: "BreakOnRecordWriteWithoutTrigger",
        signature: "BreakOnRecordWriteWithoutTrigger(TableNo: Integer)",
        summary: "Breaks on writes that skip table triggers.",
        docs_url: DEBUGGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "TableNo: Integer",
            documentation: "Table number to watch.",
        }],
    },
    BuiltinMethodDoc {
        name: "GetLastBreakRecordId",
        signature: "GetLastBreakRecordId(): RecordId",
        summary: "Returns RecordId for last record-write break.",
        docs_url: DEBUGGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetLastBreakTableId",
        signature: "GetLastBreakTableId(): Integer",
        summary: "Returns table id for last record-write break.",
        docs_url: DEBUGGER_DOCS,
        params: PARAM_NONE,
    },
];

const DECIMAL_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "ToText",
    signature: "ToText([Length: Integer] [, FormatNumber: Integer]): Text",
    summary: "Converts the Decimal value to formatted text.",
    docs_url: DECIMAL_DOCS,
    params: &[
        BuiltinMethodParam {
            label: "Length: Integer",
            documentation: "Optional minimum output length.",
        },
        BuiltinMethodParam {
            label: "FormatNumber: Integer",
            documentation: "Optional format number.",
        },
    ],
}];

const DOTNET_METHODS: &[BuiltinMethodDoc] = &[];

const FILE_UPLOAD_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "CreateInStream",
        signature: "CreateInStream([Encoding: TextEncoding]): InStream",
        summary: "Creates an input stream over uploaded file data.",
        docs_url: FILE_UPLOAD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Encoding: TextEncoding",
            documentation: "Optional text encoding.",
        }],
    },
    BuiltinMethodDoc {
        name: "CreateOutStream",
        signature: "CreateOutStream([Encoding: TextEncoding]): OutStream",
        summary: "Creates an output stream for file data.",
        docs_url: FILE_UPLOAD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Encoding: TextEncoding",
            documentation: "Optional text encoding.",
        }],
    },
    BuiltinMethodDoc {
        name: "FileName",
        signature: "FileName(): Text",
        summary: "Returns uploaded file name.",
        docs_url: FILE_UPLOAD_DOCS,
        params: PARAM_NONE,
    },
];

const INTEGER_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "ToText",
    signature: "ToText([Length: Integer] [, FormatNumber: Integer]): Text",
    summary: "Converts the Integer value to formatted text.",
    docs_url: INTEGER_DOCS,
    params: &[
        BuiltinMethodParam {
            label: "Length: Integer",
            documentation: "Optional minimum output length.",
        },
        BuiltinMethodParam {
            label: "FormatNumber: Integer",
            documentation: "Optional format number.",
        },
    ],
}];

const ISOLATED_STORAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Key: Text [, Scope: DataScope]): Boolean",
        summary: "Checks whether a key exists in isolated storage.",
        docs_url: ISOLATED_STORAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Storage key.",
            },
            BuiltinMethodParam {
                label: "Scope: DataScope",
                documentation: "Optional storage scope.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Delete",
        signature: "Delete(Key: Text [, Scope: DataScope]): Boolean",
        summary: "Deletes a key from isolated storage.",
        docs_url: ISOLATED_STORAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Storage key.",
            },
            BuiltinMethodParam {
                label: "Scope: DataScope",
                documentation: "Optional storage scope.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Key: Text, var Value: Text [, Scope: DataScope]): Boolean",
        summary: "Gets a value from isolated storage.",
        docs_url: ISOLATED_STORAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Storage key.",
            },
            BuiltinMethodParam {
                label: "var Value: Text",
                documentation: "Output value.",
            },
            BuiltinMethodParam {
                label: "Scope: DataScope",
                documentation: "Optional storage scope.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Set",
        signature:
            "Set(Key: Text, Value: Text [, Scope: DataScope] [, UseEncryption: Boolean]): Boolean",
        summary: "Stores a value in isolated storage.",
        docs_url: ISOLATED_STORAGE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Key: Text",
                documentation: "Storage key.",
            },
            BuiltinMethodParam {
                label: "Value: Text",
                documentation: "Value to store.",
            },
            BuiltinMethodParam {
                label: "Scope: DataScope",
                documentation: "Optional storage scope.",
            },
            BuiltinMethodParam {
                label: "UseEncryption: Boolean",
                documentation: "Encrypt stored value when true.",
            },
        ],
    },
];

const LABEL_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Contains",
        signature: "Contains(Value: Text): Boolean",
        summary: "Checks whether a substring exists.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Substring to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "EndsWith",
        signature: "EndsWith(Value: Text): Boolean",
        summary: "Checks whether value ends with a substring.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Suffix to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "IndexOf",
        signature: "IndexOf(Value: Text): Integer",
        summary: "Returns first index of substring.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Substring to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "LastIndexOf",
        signature: "LastIndexOf(Value: Text): Integer",
        summary: "Returns last index of substring.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Substring to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "Length",
        signature: "Length(): Integer",
        summary: "Returns label length.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "MaxStrLen",
        signature: "MaxStrLen(): Integer",
        summary: "Returns max label length.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "PadLeft",
        signature: "PadLeft(TotalWidth: Integer [, PaddingChar: Char]): Text",
        summary: "Left-pads the value to total width.",
        docs_url: LABEL_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "TotalWidth: Integer",
                documentation: "Final width.",
            },
            BuiltinMethodParam {
                label: "PaddingChar: Char",
                documentation: "Optional padding character.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "PadRight",
        signature: "PadRight(TotalWidth: Integer [, PaddingChar: Char]): Text",
        summary: "Right-pads the value to total width.",
        docs_url: LABEL_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "TotalWidth: Integer",
                documentation: "Final width.",
            },
            BuiltinMethodParam {
                label: "PaddingChar: Char",
                documentation: "Optional padding character.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(StartIndex: Integer [, Count: Integer]): Text",
        summary: "Removes characters from the value.",
        docs_url: LABEL_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "StartIndex: Integer",
                documentation: "Start index.",
            },
            BuiltinMethodParam {
                label: "Count: Integer",
                documentation: "Optional number of characters.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Replace",
        signature: "Replace(OldValue: Text, NewValue: Text): Text",
        summary: "Replaces all occurrences of a substring.",
        docs_url: LABEL_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "OldValue: Text",
                documentation: "Substring to replace.",
            },
            BuiltinMethodParam {
                label: "NewValue: Text",
                documentation: "Replacement text.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Split",
        signature: "Split(Separator: Text): List of [Text]",
        summary: "Splits text by separator.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "Separator: Text",
            documentation: "Separator text.",
        }],
    },
    BuiltinMethodDoc {
        name: "StartsWith",
        signature: "StartsWith(Value: Text): Boolean",
        summary: "Checks whether value starts with a substring.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Prefix to match.",
        }],
    },
    BuiltinMethodDoc {
        name: "StrPos",
        signature: "StrPos(SubString: Text): Integer",
        summary: "Returns first position of substring.",
        docs_url: LABEL_DOCS,
        params: &[BuiltinMethodParam {
            label: "SubString: Text",
            documentation: "Substring to find.",
        }],
    },
    BuiltinMethodDoc {
        name: "Substring",
        signature: "Substring(StartIndex: Integer [, Length: Integer]): Text",
        summary: "Returns substring from text.",
        docs_url: LABEL_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "StartIndex: Integer",
                documentation: "Start index.",
            },
            BuiltinMethodParam {
                label: "Length: Integer",
                documentation: "Optional length.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "ToLower",
        signature: "ToLower(): Text",
        summary: "Converts text to lowercase.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "ToUpper",
        signature: "ToUpper(): Text",
        summary: "Converts text to uppercase.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Trim",
        signature: "Trim(): Text",
        summary: "Trims whitespace from both ends.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TrimEnd",
        signature: "TrimEnd(): Text",
        summary: "Trims whitespace from the end.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "TrimStart",
        signature: "TrimStart(): Text",
        summary: "Trims whitespace from the start.",
        docs_url: LABEL_DOCS,
        params: PARAM_NONE,
    },
];

const OPTION_METHODS: &[BuiltinMethodDoc] = &[];

const PRODUCT_NAME_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Full",
        signature: "Full(): Text",
        summary: "Returns the full product name.",
        docs_url: PRODUCT_NAME_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "GetShortName",
        signature: "GetShortName(): Text",
        summary: "Returns the short product name.",
        docs_url: PRODUCT_NAME_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Short",
        signature: "Short(): Text",
        summary: "Returns the short product name.",
        docs_url: PRODUCT_NAME_DOCS,
        params: PARAM_NONE,
    },
];

const REQUEST_PAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Caption",
        signature: "Caption([NewCaption: Text]): Text",
        summary: "Gets or sets request page caption.",
        docs_url: REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewCaption: Text",
            documentation: "Optional new caption.",
        }],
    },
    BuiltinMethodDoc {
        name: "Editable",
        signature: "Editable([NewEditable: Boolean]): Boolean",
        summary: "Gets or sets editable state.",
        docs_url: REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewEditable: Boolean",
            documentation: "Optional new editable value.",
        }],
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Returns request page name.",
        docs_url: REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "OptionCount",
        signature: "OptionCount(): Integer",
        summary: "Returns option count on request page.",
        docs_url: REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetSelectionFilter",
        signature: "SetSelectionFilter(var Record: Record)",
        summary: "Applies selected filter to the record variable.",
        docs_url: REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "var Record: Record",
            documentation: "Record receiving selected filters.",
        }],
    },
    BuiltinMethodDoc {
        name: "Update",
        signature: "Update([SaveRecord: Boolean])",
        summary: "Refreshes request page values.",
        docs_url: REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "SaveRecord: Boolean",
            documentation: "Optional save before update.",
        }],
    },
    BuiltinMethodDoc {
        name: "Visible",
        signature: "Visible([NewVisible: Boolean]): Boolean",
        summary: "Gets or sets visible state.",
        docs_url: REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewVisible: Boolean",
            documentation: "Optional new visible value.",
        }],
    },
    BuiltinMethodDoc {
        name: "ValidationErrorCount",
        signature: "ValidationErrorCount(): Integer",
        summary: "Returns number of validation errors.",
        docs_url: REQUEST_PAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value([Name: Text]): Text",
        summary: "Gets a control value by name.",
        docs_url: REQUEST_PAGE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Optional control name.",
        }],
    },
];

const TEST_FILTER_FIELD_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "ActAsFilter",
        signature: "ActAsFilter([Value: Boolean]): Boolean",
        summary: "Gets or sets whether field acts as a filter field.",
        docs_url: TEST_FILTER_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Boolean",
            documentation: "Optional new setting.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetFilter",
        signature: "SetFilter(Filter: Text)",
        summary: "Sets a filter on the test filter field.",
        docs_url: TEST_FILTER_FIELD_DOCS,
        params: &[BuiltinMethodParam {
            label: "Filter: Text",
            documentation: "Filter expression.",
        }],
    },
];

const TEST_HTTP_REQUEST_MESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Body",
        signature: "Body(): Text",
        summary: "Returns request body text.",
        docs_url: TEST_HTTP_REQUEST_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Headers",
        signature: "Headers(): HttpHeaders",
        summary: "Returns request headers.",
        docs_url: TEST_HTTP_REQUEST_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Method",
        signature: "Method(): Text",
        summary: "Returns HTTP method.",
        docs_url: TEST_HTTP_REQUEST_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Path",
        signature: "Path(): Text",
        summary: "Returns request path.",
        docs_url: TEST_HTTP_REQUEST_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
];

const TEST_HTTP_RESPONSE_MESSAGE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Content",
        signature: "Content(): HttpContent",
        summary: "Returns response content.",
        docs_url: TEST_HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Headers",
        signature: "Headers(): HttpHeaders",
        summary: "Returns response headers.",
        docs_url: TEST_HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HttpStatusCode",
        signature: "HttpStatusCode(): Integer",
        summary: "Returns HTTP status code.",
        docs_url: TEST_HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "IsSuccessStatusCode",
        signature: "IsSuccessStatusCode(): Boolean",
        summary: "Returns whether status code is successful.",
        docs_url: TEST_HTTP_RESPONSE_MESSAGE_DOCS,
        params: PARAM_NONE,
    },
];

const TEXT_CONST_METHODS: &[BuiltinMethodDoc] = &[];

const WEB_SERVICE_ACTION_CONTEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "AddEntityKey",
        signature: "AddEntityKey(KeyName: Text, KeyValue: Any)",
        summary: "Adds an entity key entry to the action context.",
        docs_url: WEB_SERVICE_ACTION_CONTEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "KeyName: Text",
                documentation: "Entity key name.",
            },
            BuiltinMethodParam {
                label: "KeyValue: Any",
                documentation: "Entity key value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "GetObjectType",
        signature: "GetObjectType(): WebServiceActionObjectType",
        summary: "Returns action object type.",
        docs_url: WEB_SERVICE_ACTION_CONTEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SetObjectId",
        signature: "SetObjectId(ObjectId: Integer)",
        summary: "Sets action object id.",
        docs_url: WEB_SERVICE_ACTION_CONTEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "ObjectId: Integer",
            documentation: "Object id.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetObjectType",
        signature: "SetObjectType(ObjectType: WebServiceActionObjectType)",
        summary: "Sets action object type.",
        docs_url: WEB_SERVICE_ACTION_CONTEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "ObjectType: WebServiceActionObjectType",
            documentation: "Object type.",
        }],
    },
    BuiltinMethodDoc {
        name: "SetResultCode",
        signature: "SetResultCode(ResultCode: WebServiceActionResultCode [, ResultMessage: Text])",
        summary: "Sets action result code and optional message.",
        docs_url: WEB_SERVICE_ACTION_CONTEXT_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "ResultCode: WebServiceActionResultCode",
                documentation: "Result code.",
            },
            BuiltinMethodParam {
                label: "ResultMessage: Text",
                documentation: "Optional result message.",
            },
        ],
    },
];

const XML_ATTRIBUTE_COLLECTION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Count",
        signature: "Count(): Integer",
        summary: "Returns number of attributes in the collection.",
        docs_url: XML_ATTRIBUTE_COLLECTION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Name: Text, var Attribute: XmlAttribute): Boolean",
        summary: "Gets an attribute by name.",
        docs_url: XML_ATTRIBUTE_COLLECTION_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Attribute name.",
            },
            BuiltinMethodParam {
                label: "var Attribute: XmlAttribute",
                documentation: "Output attribute.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "Item",
        signature: "Item(Index: Integer): XmlAttribute",
        summary: "Gets attribute by index.",
        docs_url: XML_ATTRIBUTE_COLLECTION_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Zero-based index.",
        }],
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(Index: Integer): Text",
        summary: "Gets attribute name by index.",
        docs_url: XML_ATTRIBUTE_COLLECTION_DOCS,
        params: &[BuiltinMethodParam {
            label: "Index: Integer",
            documentation: "Zero-based index.",
        }],
    },
    BuiltinMethodDoc {
        name: "Nodes",
        signature: "Nodes(): XmlNodeList",
        summary: "Returns attributes as a node list.",
        docs_url: XML_ATTRIBUTE_COLLECTION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Remove",
        signature: "Remove(Name: Text): Boolean",
        summary: "Removes an attribute by name.",
        docs_url: XML_ATTRIBUTE_COLLECTION_DOCS,
        params: &[BuiltinMethodParam {
            label: "Name: Text",
            documentation: "Attribute name.",
        }],
    },
];

const XML_CDATA_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Value: Text): XmlCData",
        summary: "Creates a CDATA node.",
        docs_url: XML_CDATA_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "CDATA content.",
        }],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Casts CDATA to XmlNode.",
        docs_url: XML_CDATA_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value([NewValue: Text]): Text",
        summary: "Gets or sets CDATA value.",
        docs_url: XML_CDATA_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewValue: Text",
            documentation: "Optional new value.",
        }],
    },
];

const XML_COMMENT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Value: Text): XmlComment",
        summary: "Creates a comment node.",
        docs_url: XML_COMMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Comment text.",
        }],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Casts comment to XmlNode.",
        docs_url: XML_COMMENT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value([NewValue: Text]): Text",
        summary: "Gets or sets comment value.",
        docs_url: XML_COMMENT_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewValue: Text",
            documentation: "Optional new value.",
        }],
    },
];

const XML_DECLARATION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Version: Text, Encoding: Text, Standalone: Text): XmlDeclaration",
        summary: "Creates an XML declaration node.",
        docs_url: XML_DECLARATION_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Version: Text",
                documentation: "XML version.",
            },
            BuiltinMethodParam {
                label: "Encoding: Text",
                documentation: "Encoding name.",
            },
            BuiltinMethodParam {
                label: "Standalone: Text",
                documentation: "Standalone value.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Casts declaration to XmlNode.",
        docs_url: XML_DECLARATION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Encoding",
        signature: "Encoding([NewEncoding: Text]): Text",
        summary: "Gets or sets declaration encoding.",
        docs_url: XML_DECLARATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewEncoding: Text",
            documentation: "Optional new encoding.",
        }],
    },
    BuiltinMethodDoc {
        name: "Standalone",
        signature: "Standalone([NewStandalone: Text]): Text",
        summary: "Gets or sets standalone value.",
        docs_url: XML_DECLARATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewStandalone: Text",
            documentation: "Optional new standalone value.",
        }],
    },
    BuiltinMethodDoc {
        name: "Version",
        signature: "Version([NewVersion: Text]): Text",
        summary: "Gets or sets XML version.",
        docs_url: XML_DECLARATION_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewVersion: Text",
            documentation: "Optional new version.",
        }],
    },
];

const XML_DOCUMENT_TYPE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Name: Text, PublicId: Text, SystemId: Text, InternalSubset: Text): XmlDocumentType",
        summary: "Creates a document type node.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Name: Text",
                documentation: "Document type name.",
            },
            BuiltinMethodParam {
                label: "PublicId: Text",
                documentation: "Public identifier.",
            },
            BuiltinMethodParam {
                label: "SystemId: Text",
                documentation: "System identifier.",
            },
            BuiltinMethodParam {
                label: "InternalSubset: Text",
                documentation: "Internal subset declaration.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Casts document type to XmlNode.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Entities",
        signature: "Entities(): XmlNamedNodeMap",
        summary: "Returns declared entities.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "InternalSubset",
        signature: "InternalSubset(): Text",
        summary: "Returns internal subset text.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Name",
        signature: "Name(): Text",
        summary: "Returns document type name.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Notations",
        signature: "Notations(): XmlNamedNodeMap",
        summary: "Returns declared notations.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "PublicId",
        signature: "PublicId(): Text",
        summary: "Returns public identifier.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "SystemId",
        signature: "SystemId(): Text",
        summary: "Returns system identifier.",
        docs_url: XML_DOCUMENT_TYPE_DOCS,
        params: PARAM_NONE,
    },
];

const XML_NAMESPACE_MANAGER_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(XmlNameTable: XmlNameTable): XmlNamespaceManager",
        summary: "Creates a namespace manager from name table.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "XmlNameTable: XmlNameTable",
            documentation: "Name table to use.",
        }],
    },
    BuiltinMethodDoc {
        name: "AddNamespace",
        signature: "AddNamespace(Prefix: Text, NamespaceUri: Text)",
        summary: "Adds a namespace mapping.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Prefix: Text",
                documentation: "Namespace prefix.",
            },
            BuiltinMethodParam {
                label: "NamespaceUri: Text",
                documentation: "Namespace URI.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "DefaultNamespace",
        signature: "DefaultNamespace(): Text",
        summary: "Returns default namespace URI.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "HasNamespace",
        signature: "HasNamespace(Prefix: Text): Boolean",
        summary: "Checks whether prefix exists.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Prefix: Text",
            documentation: "Namespace prefix.",
        }],
    },
    BuiltinMethodDoc {
        name: "LookupNamespace",
        signature: "LookupNamespace(Prefix: Text): Text",
        summary: "Gets URI for prefix.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "Prefix: Text",
            documentation: "Namespace prefix.",
        }],
    },
    BuiltinMethodDoc {
        name: "LookupPrefix",
        signature: "LookupPrefix(NamespaceUri: Text): Text",
        summary: "Gets prefix for URI.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: &[BuiltinMethodParam {
            label: "NamespaceUri: Text",
            documentation: "Namespace URI.",
        }],
    },
    BuiltinMethodDoc {
        name: "NameTable",
        signature: "NameTable(): XmlNameTable",
        summary: "Returns the associated name table.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "RemoveNamespace",
        signature: "RemoveNamespace(Prefix: Text, NamespaceUri: Text)",
        summary: "Removes namespace mapping.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Prefix: Text",
                documentation: "Namespace prefix.",
            },
            BuiltinMethodParam {
                label: "NamespaceUri: Text",
                documentation: "Namespace URI.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "PushScope",
        signature: "PushScope()",
        summary: "Pushes a namespace scope.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "PopScope",
        signature: "PopScope(): Boolean",
        summary: "Pops a namespace scope.",
        docs_url: XML_NAMESPACE_MANAGER_DOCS,
        params: PARAM_NONE,
    },
];

const XML_NAME_TABLE_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Add",
        signature: "Add(Value: Text): Text",
        summary: "Adds a string to the name table.",
        docs_url: XML_NAME_TABLE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "String to add.",
        }],
    },
    BuiltinMethodDoc {
        name: "Get",
        signature: "Get(Value: Text): Text",
        summary: "Gets a string from the name table.",
        docs_url: XML_NAME_TABLE_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "String to get.",
        }],
    },
];

const XML_PROCESSING_INSTRUCTION_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Target: Text, Data: Text): XmlProcessingInstruction",
        summary: "Creates a processing instruction.",
        docs_url: XML_PROCESSING_INSTRUCTION_DOCS,
        params: &[
            BuiltinMethodParam {
                label: "Target: Text",
                documentation: "Instruction target.",
            },
            BuiltinMethodParam {
                label: "Data: Text",
                documentation: "Instruction data.",
            },
        ],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Casts processing instruction to XmlNode.",
        docs_url: XML_PROCESSING_INSTRUCTION_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Data",
        signature: "Data([NewData: Text]): Text",
        summary: "Gets or sets instruction data.",
        docs_url: XML_PROCESSING_INSTRUCTION_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewData: Text",
            documentation: "Optional new data.",
        }],
    },
    BuiltinMethodDoc {
        name: "Target",
        signature: "Target([NewTarget: Text]): Text",
        summary: "Gets or sets instruction target.",
        docs_url: XML_PROCESSING_INSTRUCTION_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewTarget: Text",
            documentation: "Optional new target.",
        }],
    },
];

const XML_READ_OPTIONS_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "PreserveWhitespace",
    signature: "PreserveWhitespace([NewValue: Boolean]): Boolean",
    summary: "Gets or sets preserve-whitespace mode when reading.",
    docs_url: XML_READ_OPTIONS_DOCS,
    params: &[BuiltinMethodParam {
        label: "NewValue: Boolean",
        documentation: "Optional new value.",
    }],
}];

const XML_TEXT_METHODS: &[BuiltinMethodDoc] = &[
    BuiltinMethodDoc {
        name: "Create",
        signature: "Create(Value: Text): XmlText",
        summary: "Creates a text node.",
        docs_url: XML_TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "Value: Text",
            documentation: "Text node value.",
        }],
    },
    BuiltinMethodDoc {
        name: "AsXmlNode",
        signature: "AsXmlNode(): XmlNode",
        summary: "Casts text node to XmlNode.",
        docs_url: XML_TEXT_DOCS,
        params: PARAM_NONE,
    },
    BuiltinMethodDoc {
        name: "Value",
        signature: "Value([NewValue: Text]): Text",
        summary: "Gets or sets text node value.",
        docs_url: XML_TEXT_DOCS,
        params: &[BuiltinMethodParam {
            label: "NewValue: Text",
            documentation: "Optional new value.",
        }],
    },
];

const XML_WRITE_OPTIONS_METHODS: &[BuiltinMethodDoc] = &[BuiltinMethodDoc {
    name: "PreserveWhitespace",
    signature: "PreserveWhitespace([NewValue: Boolean]): Boolean",
    summary: "Gets or sets preserve-whitespace mode when writing.",
    docs_url: XML_WRITE_OPTIONS_DOCS,
    params: &[BuiltinMethodParam {
        label: "NewValue: Boolean",
        documentation: "Optional new value.",
    }],
}];

const EMPTY_METHODS: &[BuiltinMethodDoc] = &[];

include!("builtins_missing_generated.rs");

const BOOLEAN_VALUES: &[&str] = &["true", "false"];

const TABLE_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "DataCaptionFields",
        summary: "Specifies fields shown in data captions.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "DataClassification",
        summary: "Classifies customer/system data handling.",
        literal_values: &[
            "ToBeClassified",
            "CustomerContent",
            "EndUserIdentifiableInformation",
            "AccountData",
            "EndUserPseudonymousIdentifiers",
            "OrganizationIdentifiableInformation",
            "SystemMetadata",
        ],
    },
    BuiltinPropertyDoc {
        name: "DataPerCompany",
        summary: "Stores table data per company when true.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "DrillDownPageID",
        summary: "Default drilldown page for records.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "LookupPageID",
        summary: "Default lookup page for records.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "PasteIsValid",
        summary: "Allows pasting into page based on table.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "Permissions",
        summary: "Declares indirect permissions required.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ReplicateData",
        summary: "Marks table for data replication.",
        literal_values: BOOLEAN_VALUES,
    },
];

const CODEUNIT_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "EventSubscriberInstance",
        summary: "Subscriber instance lifecycle.",
        literal_values: &["StaticAutomatic", "Manual"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Permissions",
        summary: "Declares indirect permissions required.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "SingleInstance",
        summary: "Keeps one codeunit instance per session.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "Subtype",
        summary: "Codeunit subtype behavior.",
        literal_values: &["Normal", "Install", "Upgrade", "Test", "TestRunner"],
    },
    BuiltinPropertyDoc {
        name: "TableNo",
        summary: "Associates source table for processing.",
        literal_values: &[],
    },
];

const PAGE_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "ApplicationArea",
        summary: "Controls UI visibility by application area.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "DeleteAllowed",
        summary: "Allows deleting records in page UI.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "Editable",
        summary: "Controls editability of page or control.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "InsertAllowed",
        summary: "Allows inserting records in page UI.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "LinksAllowed",
        summary: "Allows navigation links in page UI.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "ModifyAllowed",
        summary: "Allows modifying records in page UI.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "PageType",
        summary: "Defines page layout and behavior type.",
        literal_values: &[
            "Card",
            "CardPart",
            "List",
            "ListPart",
            "Document",
            "Worksheet",
            "RoleCenter",
            "StandardDialog",
            "ConfirmationDialog",
            "NavigatePage",
            "API",
            "HeadlinePart",
        ],
    },
    BuiltinPropertyDoc {
        name: "SourceTable",
        summary: "Specifies source table for page.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "UsageCategory",
        summary: "Categorizes page in search and role explorer.",
        literal_values: &[
            "None",
            "Lists",
            "Tasks",
            "ReportsAndAnalysis",
            "Documents",
            "History",
            "Administration",
        ],
    },
];

const FIELD_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "ApplicationArea",
        summary: "Controls UI visibility by application area.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "AutoFormatType",
        summary: "Applies numeric formatting profile.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "BlankZero",
        summary: "Shows zero values as blank.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "CalcFormula",
        summary: "Defines FlowField calculation formula.",
        literal_values: &["Lookup", "Sum", "Average", "Count", "Exist", "Min", "Max"],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "DataClassification",
        summary: "Classifies customer/system data handling.",
        literal_values: &[
            "ToBeClassified",
            "CustomerContent",
            "EndUserIdentifiableInformation",
            "AccountData",
            "EndUserPseudonymousIdentifiers",
            "OrganizationIdentifiableInformation",
            "SystemMetadata",
        ],
    },
    BuiltinPropertyDoc {
        name: "Editable",
        summary: "Controls field editability.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "FieldClass",
        summary: "Defines normal/flowfield/flowfilter behavior.",
        literal_values: &["Normal", "FlowField", "FlowFilter"],
    },
    BuiltinPropertyDoc {
        name: "InitValue",
        summary: "Default value on initialization.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "NotBlank",
        summary: "Requires non-empty field value.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "OptionCaption",
        summary: "Captions for Option members.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "OptionMembers",
        summary: "Member names for Option fields.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "TableRelation",
        summary: "Defines related table/field and lookup filtering.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ToolTip",
        summary: "Tooltip displayed in client UI.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ValidateTableRelation",
        summary: "Validates entered value against related table.",
        literal_values: BOOLEAN_VALUES,
    },
];

const KEY_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Clustered",
        summary: "Marks SQL key as clustered.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "Enabled",
        summary: "Enables or disables the key.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "MaintainSQLIndex",
        summary: "Maintains SQL index for the key.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "SumIndexFields",
        summary: "Defines SIFT aggregate fields.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Unique",
        summary: "Enforces uniqueness on key values.",
        literal_values: BOOLEAN_VALUES,
    },
];

const ENUM_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "AssignmentCompatibility",
        summary: "Allows assignment compatibility with options.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Extensible",
        summary: "Allows extending the enum.",
        literal_values: BOOLEAN_VALUES,
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
];

const ENUM_VALUE_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Implementation",
        summary: "Maps enum value to interface implementation.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
];

const REPORT_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "ApplicationArea",
        summary: "Controls UI visibility by application area.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "DefaultLayout",
        summary: "Default rendering layout type.",
        literal_values: &["None", "RDLC", "Word", "Excel"],
    },
    BuiltinPropertyDoc {
        name: "ExcelLayout",
        summary: "Path to Excel layout file.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "RDLCLayout",
        summary: "Path to RDLC layout file.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "UsageCategory",
        summary: "Categorizes report in search and role explorer.",
        literal_values: &[
            "None",
            "Lists",
            "Tasks",
            "ReportsAndAnalysis",
            "Documents",
            "History",
            "Administration",
        ],
    },
    BuiltinPropertyDoc {
        name: "WordLayout",
        summary: "Path to Word layout file.",
        literal_values: &[],
    },
];

const QUERY_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
];

const XMLPORT_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "Direction",
        summary: "Allowed XMLport direction.",
        literal_values: &["Import", "Export", "Both"],
    },
    BuiltinPropertyDoc {
        name: "Encoding",
        summary: "Text encoding used by XMLport.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "FormatEvaluate",
        summary: "Controls format-evaluation behavior.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
];

const INTERFACE_PROPERTIES: &[BuiltinPropertyDoc] = &[
    BuiltinPropertyDoc {
        name: "Access",
        summary: "Controls accessibility of the object.",
        literal_values: &["Public", "Internal"],
    },
    BuiltinPropertyDoc {
        name: "Caption",
        summary: "Defines localized display caption.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteReason",
        summary: "Explains why element is obsolete.",
        literal_values: &[],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteState",
        summary: "Obsolescence lifecycle state.",
        literal_values: &["No", "Pending", "Removed"],
    },
    BuiltinPropertyDoc {
        name: "ObsoleteTag",
        summary: "Version tag for obsolescence.",
        literal_values: &[],
    },
];

const EMPTY_PROPERTIES: &[BuiltinPropertyDoc] = &[];

fn normalize_object_kind(object_kind: &str) -> &str {
    if object_kind.eq_ignore_ascii_case("page_builtin") {
        "page"
    } else if object_kind.eq_ignore_ascii_case("report_builtin") {
        "report"
    } else if object_kind.eq_ignore_ascii_case("query_builtin") {
        "query"
    } else if object_kind.eq_ignore_ascii_case("xmlport_builtin") {
        "xmlport"
    } else if object_kind.eq_ignore_ascii_case("enum_builtin") {
        "enum"
    } else if object_kind.eq_ignore_ascii_case("codeunit_builtin") {
        "codeunit"
    } else if object_kind.eq_ignore_ascii_case("code") {
        "text"
    } else {
        object_kind
    }
}

pub fn methods_for_object_kind(object_kind: &str) -> &'static [BuiltinMethodDoc] {
    match normalize_object_kind(object_kind) {
        "any" => ANY_METHODS,
        "table" => RECORD_METHODS,
        "biginteger" => BIG_INTEGER_METHODS,
        "boolean" => BOOLEAN_METHODS,
        "byte" => BYTE_METHODS,
        "char" => CHAR_METHODS,
        "codeunit" => CODEUNIT_METHODS,
        "companyproperty" => COMPANY_PROPERTY_METHODS,
        "cookie" => COOKIE_METHODS,
        "datatransfer" => DATA_TRANSFER_METHODS,
        "dateformula" => DATE_FORMULA_METHODS,
        "debugger" => DEBUGGER_METHODS,
        "decimal" => DECIMAL_METHODS,
        "dotnet" => DOTNET_METHODS,
        "page" => PAGE_METHODS,
        "report" => REPORT_METHODS,
        "query" => QUERY_METHODS,
        "xmlport" => XMLPORT_METHODS,
        "enum" => ENUM_METHODS,
        "fileupload" => FILE_UPLOAD_METHODS,
        "integer" => INTEGER_METHODS,
        "isolatedstorage" => ISOLATED_STORAGE_METHODS,
        "label" => LABEL_METHODS,
        "list" => LIST_METHODS,
        "option" => OPTION_METHODS,
        "productname" => PRODUCT_NAME_METHODS,
        "requestpage" => REQUEST_PAGE_METHODS,
        "dictionary" => DICTIONARY_METHODS,
        "text" => TEXT_METHODS,
        "textconst" => TEXT_CONST_METHODS,
        "bigtext" => BIGTEXT_METHODS,
        "guid" => GUID_METHODS,
        "jsonobject" => JSON_OBJECT_METHODS,
        "jsonarray" => JSON_ARRAY_METHODS,
        "jsontoken" => JSON_TOKEN_METHODS,
        "jsonvalue" => JSON_VALUE_METHODS,
        "httpclient" => HTTP_CLIENT_METHODS,
        "httprequestmessage" => HTTP_REQUEST_MESSAGE_METHODS,
        "httpresponsemessage" => HTTP_RESPONSE_MESSAGE_METHODS,
        "httpcontent" => HTTP_CONTENT_METHODS,
        "httpheaders" => HTTP_HEADERS_METHODS,
        "xmldocument" => XML_DOCUMENT_METHODS,
        "xmlelement" => XML_ELEMENT_METHODS,
        "xmlnode" => XML_NODE_METHODS,
        "xmlnodelist" => XML_NODE_LIST_METHODS,
        "xmlattribute" => XML_ATTRIBUTE_METHODS,
        "xmlattributecollection" => XML_ATTRIBUTE_COLLECTION_METHODS,
        "xmlcdata" => XML_CDATA_METHODS,
        "xmlcomment" => XML_COMMENT_METHODS,
        "xmldeclaration" => XML_DECLARATION_METHODS,
        "xmldocumenttype" => XML_DOCUMENT_TYPE_METHODS,
        "xmlnamespacemanager" => XML_NAMESPACE_MANAGER_METHODS,
        "xmlnametable" => XML_NAME_TABLE_METHODS,
        "xmlprocessinginstruction" => XML_PROCESSING_INSTRUCTION_METHODS,
        "xmlreadoptions" => XML_READ_OPTIONS_METHODS,
        "xmltext" => XML_TEXT_METHODS,
        "xmlwriteoptions" => XML_WRITE_OPTIONS_METHODS,
        "recordref" => RECORDREF_METHODS,
        "fieldref" => FIELDREF_METHODS,
        "keyref" => KEYREF_METHODS,
        "date" => DATE_METHODS,
        "time" => TIME_METHODS,
        "datetime" => DATETIME_METHODS,
        "duration" => DURATION_METHODS,
        "instream" => INSTREAM_METHODS,
        "outstream" => OUTSTREAM_METHODS,
        "recordid" => RECORDID_METHODS,
        "variant" => VARIANT_METHODS,
        "sessionsettings" => SESSION_SETTINGS_METHODS,
        "notification" => NOTIFICATION_METHODS,
        "dialog" => DIALOG_METHODS,
        "session" => SESSION_METHODS,
        "moduleinfo" => MODULE_INFO_METHODS,
        "secrettext" => SECRET_TEXT_METHODS,
        "database" => DATABASE_METHODS,
        "system" => SYSTEM_METHODS,
        "sessioninformation" => SESSION_INFORMATION_METHODS,
        "taskscheduler" => TASK_SCHEDULER_METHODS,
        "filterpagebuilder" => FILTER_PAGE_BUILDER_METHODS,
        "blob" => BLOB_METHODS,
        "file" => FILE_METHODS,
        "version" => VERSION_METHODS,
        "moduledependencyinfo" => MODULE_DEPENDENCY_INFO_METHODS,
        "navapp" => NAV_APP_METHODS,
        "numbersequence" => NUMBER_SEQUENCE_METHODS,
        "textbuilder" => TEXT_BUILDER_METHODS,
        "media" => MEDIA_METHODS,
        "mediaset" => MEDIA_SET_METHODS,
        "errorinfo" => ERROR_INFO_METHODS,
        "testpage" => TEST_PAGE_METHODS,
        "testfield" => TEST_FIELD_METHODS,
        "testaction" => TEST_ACTION_METHODS,
        "testrequestpage" => TEST_REQUEST_PAGE_METHODS,
        "testpart" => TEST_PART_METHODS,
        "testfilter" => TEST_FILTER_METHODS,
        "testfilterfield" => TEST_FILTER_FIELD_METHODS,
        "testhttprequestmessage" => TEST_HTTP_REQUEST_MESSAGE_METHODS,
        "testhttpresponsemessage" => TEST_HTTP_RESPONSE_MESSAGE_METHODS,
        "webserviceactioncontext" => WEB_SERVICE_ACTION_CONTEXT_METHODS,
        _ => EMPTY_METHODS,
    }
}

pub fn find_builtin_method(
    object_kind: &str,
    method_name: &str,
) -> Option<&'static BuiltinMethodDoc> {
    let kind = normalize_object_kind(object_kind);
    methods_for_object_kind(kind)
        .iter()
        .chain(missing_methods_for_object_kind(kind).iter())
        .find(|m| m.name.eq_ignore_ascii_case(method_name))
}

pub fn is_known_builtin_method(object_kind: &str, method_name: &str) -> bool {
    find_builtin_method(object_kind, method_name).is_some()
}

pub fn builtin_method_return_type(object_kind: &str, method_name: &str) -> Option<&'static str> {
    let method = find_builtin_method(object_kind, method_name)?;
    let separator = method.signature.rfind("):")?;
    let return_type = method.signature[separator + 2..].trim();
    if return_type.is_empty() {
        None
    } else {
        Some(return_type)
    }
}

pub fn builtin_object_kind_from_name(object_name: &str) -> Option<&'static str> {
    match object_name.to_ascii_lowercase().as_str() {
        "any" => Some("any"),
        "biginteger" => Some("biginteger"),
        "boolean" => Some("boolean"),
        "byte" => Some("byte"),
        "char" => Some("char"),
        "companyproperty" => Some("companyproperty"),
        "cookie" => Some("cookie"),
        "codeunit" => Some("codeunit_builtin"),
        "datatransfer" => Some("datatransfer"),
        "dateformula" => Some("dateformula"),
        "debugger" => Some("debugger"),
        "decimal" => Some("decimal"),
        "dotnet" => Some("dotnet"),
        "page" => Some("page_builtin"),
        "report" => Some("report_builtin"),
        "xmlport" => Some("xmlport_builtin"),
        "query" => Some("query_builtin"),
        "fileupload" => Some("fileupload"),
        "guid" => Some("guid"),
        "integer" => Some("integer"),
        "isolatedstorage" => Some("isolatedstorage"),
        "label" => Some("label"),
        "text" => Some("text"),
        "code" => Some("text"),
        "option" => Some("option"),
        "productname" => Some("productname"),
        "requestpage" => Some("requestpage"),
        "bigtext" => Some("bigtext"),
        "secrettext" => Some("secrettext"),
        "textconst" => Some("textconst"),
        "jsonobject" => Some("jsonobject"),
        "jsonarray" => Some("jsonarray"),
        "jsontoken" => Some("jsontoken"),
        "jsonvalue" => Some("jsonvalue"),
        "httpclient" => Some("httpclient"),
        "httprequestmessage" => Some("httprequestmessage"),
        "httpresponsemessage" => Some("httpresponsemessage"),
        "httpcontent" => Some("httpcontent"),
        "httpheaders" => Some("httpheaders"),
        "xmldocument" => Some("xmldocument"),
        "xmlelement" => Some("xmlelement"),
        "xmlnode" => Some("xmlnode"),
        "xmlnodelist" => Some("xmlnodelist"),
        "xmlattribute" => Some("xmlattribute"),
        "xmlattributecollection" => Some("xmlattributecollection"),
        "xmlcdata" => Some("xmlcdata"),
        "xmlcomment" => Some("xmlcomment"),
        "xmldeclaration" => Some("xmldeclaration"),
        "xmldocumenttype" => Some("xmldocumenttype"),
        "xmlnamespacemanager" => Some("xmlnamespacemanager"),
        "xmlnametable" => Some("xmlnametable"),
        "xmlprocessinginstruction" => Some("xmlprocessinginstruction"),
        "xmlreadoptions" => Some("xmlreadoptions"),
        "xmltext" => Some("xmltext"),
        "xmlwriteoptions" => Some("xmlwriteoptions"),
        "recordref" => Some("recordref"),
        "fieldref" => Some("fieldref"),
        "keyref" => Some("keyref"),
        "date" => Some("date"),
        "time" => Some("time"),
        "datetime" => Some("datetime"),
        "duration" => Some("duration"),
        "instream" => Some("instream"),
        "outstream" => Some("outstream"),
        "recordid" => Some("recordid"),
        "variant" => Some("variant"),
        "sessionsettings" => Some("sessionsettings"),
        "notification" => Some("notification"),
        "dialog" => Some("dialog"),
        "session" => Some("session"),
        "moduleinfo" => Some("moduleinfo"),
        "database" => Some("database"),
        "system" => Some("system"),
        "sessioninformation" => Some("sessioninformation"),
        "taskscheduler" => Some("taskscheduler"),
        "filterpagebuilder" => Some("filterpagebuilder"),
        "blob" => Some("blob"),
        "file" => Some("file"),
        "version" => Some("version"),
        "moduledependencyinfo" => Some("moduledependencyinfo"),
        "navapp" => Some("navapp"),
        "numbersequence" => Some("numbersequence"),
        "textbuilder" => Some("textbuilder"),
        "media" => Some("media"),
        "mediaset" => Some("mediaset"),
        "errorinfo" => Some("errorinfo"),
        "testpage" => Some("testpage"),
        "testfield" => Some("testfield"),
        "testaction" => Some("testaction"),
        "testrequestpage" => Some("testrequestpage"),
        "testpart" => Some("testpart"),
        "testfilter" => Some("testfilter"),
        "testfilterfield" => Some("testfilterfield"),
        "testhttprequestmessage" => Some("testhttprequestmessage"),
        "testhttpresponsemessage" => Some("testhttpresponsemessage"),
        "webserviceactioncontext" => Some("webserviceactioncontext"),
        _ => None,
    }
}

pub fn properties_for_scope(scope: &str) -> &'static [BuiltinPropertyDoc] {
    match scope.to_ascii_lowercase().as_str() {
        "table" => TABLE_PROPERTIES,
        "codeunit" => CODEUNIT_PROPERTIES,
        "page" => PAGE_PROPERTIES,
        "field" => FIELD_PROPERTIES,
        "key" => KEY_PROPERTIES,
        "enum" => ENUM_PROPERTIES,
        "enumvalue" => ENUM_VALUE_PROPERTIES,
        "report" => REPORT_PROPERTIES,
        "query" => QUERY_PROPERTIES,
        "xmlport" => XMLPORT_PROPERTIES,
        "interface" => INTERFACE_PROPERTIES,
        _ => EMPTY_PROPERTIES,
    }
}

pub fn literal_values_for_property(property_name: &str) -> Option<&'static [&'static str]> {
    [
        TABLE_PROPERTIES,
        CODEUNIT_PROPERTIES,
        PAGE_PROPERTIES,
        FIELD_PROPERTIES,
        KEY_PROPERTIES,
        ENUM_PROPERTIES,
        ENUM_VALUE_PROPERTIES,
        REPORT_PROPERTIES,
        QUERY_PROPERTIES,
        XMLPORT_PROPERTIES,
        INTERFACE_PROPERTIES,
    ]
    .iter()
    .flat_map(|props| props.iter())
    .find(|prop| prop.name.eq_ignore_ascii_case(property_name))
    .map(|prop| prop.literal_values)
}
