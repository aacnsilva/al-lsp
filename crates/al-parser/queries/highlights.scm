; Keywords
"begin" @keyword
"end" @keyword
"if" @keyword
"then" @keyword
"else" @keyword
"for" @keyword
"to" @keyword
"downto" @keyword
"do" @keyword
"while" @keyword
"repeat" @keyword
"until" @keyword
"case" @keyword
"of" @keyword
"exit" @keyword
"with" @keyword
"var" @keyword
"local" @keyword
"internal" @keyword
"protected" @keyword
"procedure" @keyword
"trigger" @keyword
"extends" @keyword
"and" @keyword.operator
"or" @keyword.operator
"not" @keyword.operator
"xor" @keyword.operator
"mod" @keyword.operator
"div" @keyword.operator

; Object types
"table" @keyword.type
"tableextension" @keyword.type
"page" @keyword.type
"pageextension" @keyword.type
"codeunit" @keyword.type
"report" @keyword.type
"enum" @keyword.type
"enumextension" @keyword.type
"xmlport" @keyword.type
"query" @keyword.type
"interface" @keyword.type
"permissionset" @keyword.type
"controladdin" @keyword.type

; Declarations
(procedure_declaration name: (identifier) @function)
(procedure_declaration name: (quoted_identifier) @function)
(trigger_declaration name: (identifier) @function)
(trigger_declaration name: (quoted_identifier) @function)
(variable_declaration name: (identifier) @variable)
(variable_declaration name: (quoted_identifier) @variable)
(parameter name: (identifier) @variable.parameter)
(parameter name: (quoted_identifier) @variable.parameter)
(field_declaration name: (identifier) @property)
(field_declaration name: (quoted_identifier) @property)

; Types
(simple_type (identifier) @type)
(sized_type base: (identifier) @type)
(record_type table: (identifier) @type)
(record_type table: (quoted_identifier) @type)
"record" @keyword.type
"array" @keyword.type
"list" @keyword.type

; Function calls
(function_call function: (identifier) @function.call)
(method_call method: (identifier) @function.method)

; Literals
(string_literal) @string
(integer_literal) @number
(decimal_literal) @number
(boolean_literal) @constant.builtin
(quoted_identifier) @string.special

; Comments
(line_comment) @comment
(block_comment) @comment

; Operators
":=" @operator
"=" @operator
"<>" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator

; Punctuation
";" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter
"(" @punctuation.bracket
")" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
