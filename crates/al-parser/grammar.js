/// <reference types="tree-sitter-cli/dsl" />

/**
 * Tree-sitter grammar for AL (Microsoft Dynamics 365 Business Central).
 *
 * AL is a Pascal-like language with case-insensitive keywords,
 * double-quoted identifiers, and single-quoted strings.
 */

// Case-insensitive keyword helper: "begin" → /[bB][eE][gG][iI][nN]/
function kw(word) {
  return new RegExp(
    word
      .split("")
      .map((c) =>
        /[a-zA-Z]/.test(c) ? `[${c.toLowerCase()}${c.toUpperCase()}]` : c
      )
      .join("")
  );
}

// Comma-separated list (1 or more)
function commaSep1(rule) {
  return seq(rule, repeat(seq(",", rule)));
}

// Semicolon-separated list (1 or more)
function semiSep1(rule) {
  return seq(rule, repeat(seq(";", rule)));
}

module.exports = grammar({
  name: "al",

  extras: ($) => [/\s/, $.line_comment, $.block_comment],

  // NOTE: We do NOT use the `word` property because our keywords are
  // case-insensitive regexes (kw()), and tree-sitter's keyword extraction
  // only works with string literals. Instead we rely on token precedence.

  conflicts: ($) => [
    [$.var_section],
  ],

  rules: {
    // ─── Top-level ───────────────────────────────────────────

    source_file: ($) => repeat($._object_declaration),

    _object_declaration: ($) =>
      choice(
        $.table_declaration,
        $.table_extension_declaration,
        $.page_declaration,
        $.page_extension_declaration,
        $.codeunit_declaration,
        $.report_declaration,
        $.enum_declaration,
        $.enum_extension_declaration,
        $.xmlport_declaration,
        $.query_declaration,
        $.interface_declaration,
        $.permissionset_declaration,
        $.controladdin_declaration
      ),

    // ─── Object declarations ─────────────────────────────────

    table_declaration: ($) =>
      seq(kw("table"), $.integer_literal, $._object_name, "{", optional($._table_body), "}"),

    table_extension_declaration: ($) =>
      seq(
        kw("tableextension"),
        $.integer_literal,
        $._object_name,
        kw("extends"),
        $._object_name,
        "{",
        optional($._table_body),
        "}"
      ),

    page_declaration: ($) =>
      seq(kw("page"), $.integer_literal, $._object_name, "{", optional($._object_body), "}"),

    page_extension_declaration: ($) =>
      seq(
        kw("pageextension"),
        $.integer_literal,
        $._object_name,
        kw("extends"),
        $._object_name,
        "{",
        optional($._object_body),
        "}"
      ),

    codeunit_declaration: ($) =>
      seq(kw("codeunit"), $.integer_literal, $._object_name, optional($.implements_clause), "{", optional($._object_body), "}"),

    report_declaration: ($) =>
      seq(kw("report"), $.integer_literal, $._object_name, "{", optional($._object_body), "}"),

    enum_declaration: ($) =>
      seq(kw("enum"), $.integer_literal, $._object_name, "{", optional($._enum_body), "}"),

    enum_extension_declaration: ($) =>
      seq(
        kw("enumextension"),
        $.integer_literal,
        $._object_name,
        kw("extends"),
        $._object_name,
        "{",
        optional($._enum_body),
        "}"
      ),

    xmlport_declaration: ($) =>
      seq(kw("xmlport"), $.integer_literal, $._object_name, "{", optional($._object_body), "}"),

    query_declaration: ($) =>
      seq(kw("query"), $.integer_literal, $._object_name, "{", optional($._object_body), "}"),

    interface_declaration: ($) =>
      seq(kw("interface"), $._object_name, "{", optional($._interface_body), "}"),

    permissionset_declaration: ($) =>
      seq(kw("permissionset"), $.integer_literal, $._object_name, "{", optional($._object_body), "}"),

    controladdin_declaration: ($) =>
      seq(kw("controladdin"), $._object_name, "{", optional($._object_body), "}"),

    _object_name: ($) => choice($.identifier, $.quoted_identifier),

    // implements IFoo, IBar
    implements_clause: ($) =>
      seq(kw("implements"), commaSep1($._object_name)),

    // ─── Object bodies ───────────────────────────────────────

    _table_body: ($) =>
      repeat1(
        choice(
          $.property,
          $.fields_section,
          $.keys_section,
          $.fieldgroups_section,
          $.trigger_declaration,
          $.procedure_declaration,
          $.var_section
        )
      ),

    _object_body: ($) =>
      repeat1(
        choice(
          $.property,
          $.trigger_declaration,
          $.procedure_declaration,
          $.var_section,
          $.fields_section,
          $.keys_section,
          $.fieldgroups_section,
          $.layout_section,
          $.actions_section,
          $.dataset_section,
          $._named_section
        )
      ),

    _enum_body: ($) =>
      repeat1(choice($.property, $.enum_value_declaration)),

    _interface_body: ($) =>
      repeat1(
        choice(
          $.property,
          $.procedure_declaration,
          $.interface_method
        )
      ),

    // Method signature without a body — used in interfaces.
    interface_method: ($) =>
      seq(
        optional(field("access", choice(kw("local"), kw("internal"), kw("protected")))),
        kw("procedure"),
        field("name", choice($.identifier, $.quoted_identifier)),
        "(",
        field("parameters", optional($.parameter_list)),
        ")",
        optional(field("return_type", $.return_type)),
        optional(";")
      ),

    // ─── Property ────────────────────────────────────────────

    property: ($) =>
      seq(
        field("name", choice($.identifier, $.quoted_identifier)),
        "=",
        field("value", $._property_value),
        ";"
      ),

    _property_value: ($) =>
      choice(
        $.string_literal,
        $.integer_literal,
        $.decimal_literal,
        $.boolean_literal,
        $.identifier,
        $.quoted_identifier,
        // Enum-like: Sorting::Ascending
        seq($.identifier, "::", $.identifier),
        // Property list: field1, field2
        commaSep1(choice($.identifier, $.quoted_identifier))
      ),

    // ─── Fields section (tables) ─────────────────────────────

    fields_section: ($) =>
      seq(kw("fields"), "{", repeat($.field_declaration), "}"),

    field_declaration: ($) =>
      seq(
        kw("field"),
        "(",
        field("id", $.integer_literal),
        ";",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        field("type", $._type_reference),
        ")",
        "{",
        repeat(choice($.property, $.trigger_declaration)),
        "}"
      ),

    // ─── Keys section (tables) ───────────────────────────────

    keys_section: ($) =>
      seq(kw("keys"), "{", repeat($.key_declaration), "}"),

    key_declaration: ($) =>
      seq(
        kw("key"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        commaSep1(choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        repeat($.property),
        "}"
      ),

    // ─── FieldGroups section ─────────────────────────────────

    fieldgroups_section: ($) =>
      seq(kw("fieldgroups"), "{", repeat($.fieldgroup_declaration), "}"),

    fieldgroup_declaration: ($) =>
      seq(
        kw("fieldgroup"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        commaSep1(choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        "}"
      ),

    // ─── Enum values ─────────────────────────────────────────

    enum_value_declaration: ($) =>
      seq(
        kw("value"),
        "(",
        field("id", $.integer_literal),
        ";",
        field("name", choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        repeat($.property),
        "}"
      ),

    // ─── Layout / actions / dataset (pages, reports) ─────────

    layout_section: ($) =>
      seq(kw("layout"), "{", repeat($._named_section), "}"),

    actions_section: ($) =>
      seq(kw("actions"), "{", repeat($._named_section), "}"),

    dataset_section: ($) =>
      seq(kw("dataset"), "{", repeat($._named_section), "}"),

    _named_section: ($) =>
      choice($.area_section, $.group_section, $.page_field, $.page_action, $.part_section, $.repeater_section),

    area_section: ($) =>
      seq(
        kw("area"),
        "(",
        field("name", $.identifier),
        ")",
        "{",
        repeat($._named_section),
        "}"
      ),

    group_section: ($) =>
      seq(
        kw("group"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        repeat(choice($._named_section, $.property)),
        "}"
      ),

    repeater_section: ($) =>
      seq(
        kw("repeater"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        repeat(choice($._named_section, $.property)),
        "}"
      ),

    part_section: ($) =>
      seq(
        kw("part"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        $._expression,
        ")",
        "{",
        repeat(choice($.property, $.trigger_declaration)),
        "}"
      ),

    page_field: ($) =>
      seq(
        kw("field"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        field("expression", $._expression),
        ")",
        "{",
        repeat(choice($.property, $.trigger_declaration)),
        "}"
      ),

    page_action: ($) =>
      seq(
        kw("action"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        repeat(choice($.property, $.trigger_declaration)),
        "}"
      ),

    // ─── Procedures & triggers ───────────────────────────────

    procedure_declaration: ($) =>
      seq(
        optional(field("access", choice(kw("local"), kw("internal"), kw("protected")))),
        kw("procedure"),
        field("name", choice($.identifier, $.quoted_identifier)),
        "(",
        field("parameters", optional($.parameter_list)),
        ")",
        optional(field("return_type", $.return_type)),
        optional(field("vars", $.var_section)),
        field("body", $.block)
      ),

    trigger_declaration: ($) =>
      seq(
        kw("trigger"),
        field("name", choice($.identifier, $.quoted_identifier)),
        "(",
        field("parameters", optional($.parameter_list)),
        ")",
        optional(field("vars", $.var_section)),
        field("body", $.block)
      ),

    // AL uses semicolons to separate parameters
    parameter_list: ($) => semiSep1($.parameter),

    parameter: ($) =>
      seq(
        optional(kw("var")),
        field("name", choice($.identifier, $.quoted_identifier)),
        ":",
        field("type", $._type_reference)
      ),

    return_type: ($) => seq(":", $._type_reference),

    // ─── Variable declarations ───────────────────────────────

    var_section: ($) =>
      seq(kw("var"), repeat1($.variable_declaration)),

    variable_declaration: ($) =>
      seq(
        field("name", choice($.identifier, $.quoted_identifier)),
        ":",
        field("type", $._type_reference),
        ";"
      ),

    // ─── Block ───────────────────────────────────────────────
    // Pascal-style: semicolons are separators between statements.
    // We use optional ";" after each statement for robustness.

    block: ($) =>
      prec.right(seq(
        kw("begin"),
        repeat(seq($._statement, optional(";"))),
        kw("end"),
        optional(";")
      )),

    // ─── Statements ──────────────────────────────────────────
    // Statements do NOT include trailing semicolons — the block handles that.

    _statement: ($) =>
      choice(
        $.assignment_statement,
        $.expression_statement,
        $.if_statement,
        $.case_statement,
        $.for_statement,
        $.while_statement,
        $.repeat_statement,
        $.exit_statement,
        $.with_statement,
        $.block
      ),

    assignment_statement: ($) =>
      seq(field("target", $._expression), ":=", field("value", $._expression)),

    expression_statement: ($) => prec(-1, $._expression),

    if_statement: ($) =>
      prec.right(
        seq(
          kw("if"),
          field("condition", $._expression),
          kw("then"),
          field("consequence", $._statement),
          optional(seq(kw("else"), field("alternative", $._statement)))
        )
      ),

    case_statement: ($) =>
      seq(
        kw("case"),
        field("expression", $._expression),
        kw("of"),
        repeat(seq($.case_branch, optional(";"))),
        optional(seq(kw("else"), repeat(seq($._statement, optional(";"))))),
        kw("end")
      ),

    case_branch: ($) =>
      seq(commaSep1($._expression), ":", $._statement),

    for_statement: ($) =>
      seq(
        kw("for"),
        field("variable", $._expression),
        ":=",
        field("from", $._expression),
        choice(kw("to"), kw("downto")),
        field("to", $._expression),
        kw("do"),
        field("body", $._statement)
      ),

    while_statement: ($) =>
      seq(
        kw("while"),
        field("condition", $._expression),
        kw("do"),
        field("body", $._statement)
      ),

    repeat_statement: ($) =>
      seq(
        kw("repeat"),
        repeat(seq($._statement, optional(";"))),
        kw("until"),
        field("condition", $._expression)
      ),

    exit_statement: ($) =>
      prec.right(seq(kw("exit"), optional(seq("(", optional($._expression), ")")))),

    with_statement: ($) =>
      seq(kw("with"), $._expression, kw("do"), $._statement),

    // ─── Expressions ─────────────────────────────────────────

    _expression: ($) =>
      choice(
        $.or_expression,
        $._unary_expression
      ),

    or_expression: ($) =>
      prec.left(
        1,
        seq(
          field("left", $._expression),
          field("operator", choice(kw("or"), kw("xor"))),
          field("right", $._expression)
        )
      ),

    and_expression: ($) =>
      prec.left(
        2,
        seq(
          field("left", $._expression),
          field("operator", kw("and")),
          field("right", $._expression)
        )
      ),

    comparison_expression: ($) =>
      prec.left(
        3,
        seq(
          field("left", $._expression),
          field("operator", choice("=", "<>", "<", ">", "<=", ">=")),
          field("right", $._expression)
        )
      ),

    additive_expression: ($) =>
      prec.left(
        4,
        seq(
          field("left", $._expression),
          field("operator", choice("+", "-")),
          field("right", $._expression)
        )
      ),

    multiplicative_expression: ($) =>
      prec.left(
        5,
        seq(
          field("left", $._expression),
          field("operator", choice("*", "/", kw("mod"), kw("div"))),
          field("right", $._expression)
        )
      ),

    _unary_expression: ($) =>
      choice(
        $.not_expression,
        $.negation_expression,
        $.and_expression,
        $.comparison_expression,
        $.additive_expression,
        $.multiplicative_expression,
        $._postfix_expression
      ),

    not_expression: ($) =>
      prec(6, seq(kw("not"), $._expression)),

    negation_expression: ($) =>
      prec(6, seq("-", $._postfix_expression)),

    _postfix_expression: ($) =>
      choice(
        $.method_call,
        $.member_access,
        $.array_access,
        $.primary_expression
      ),

    method_call: ($) =>
      prec.left(
        9,
        seq(
          field("object", $._postfix_expression),
          ".",
          field("method", choice($.identifier, $.quoted_identifier)),
          "(",
          field("arguments", optional($.argument_list)),
          ")"
        )
      ),

    member_access: ($) =>
      prec.left(
        8,
        seq(
          field("object", $._postfix_expression),
          ".",
          field("member", choice($.identifier, $.quoted_identifier))
        )
      ),

    array_access: ($) =>
      prec.left(
        8,
        seq(field("object", $._postfix_expression), "[", $._expression, "]")
      ),

    primary_expression: ($) =>
      choice(
        $.identifier,
        $.quoted_identifier,
        $.string_literal,
        $.integer_literal,
        $.decimal_literal,
        $.boolean_literal,
        $.parenthesized_expression,
        $.function_call,
        $.qualified_enum_value
      ),

    parenthesized_expression: ($) => seq("(", $._expression, ")"),

    function_call: ($) =>
      prec(
        7,
        seq(
          field("function", choice($.identifier, $.quoted_identifier)),
          "(",
          field("arguments", optional($.argument_list)),
          ")"
        )
      ),

    qualified_enum_value: ($) =>
      prec.left(7, seq($.identifier, "::", $.identifier)),

    argument_list: ($) => commaSep1($._expression),

    // ─── Type references ─────────────────────────────────────

    _type_reference: ($) =>
      choice(
        $.simple_type,
        $.sized_type,
        $.record_type,
        $.enum_type,
        $.list_type,
        $.array_type,
        $.option_type,
        $.label_type,
        $.dotnet_type
      ),

    simple_type: ($) => choice($.identifier, $.quoted_identifier),

    sized_type: ($) =>
      seq(
        field("base", $.identifier),
        "[",
        field("size", $.integer_literal),
        "]"
      ),

    record_type: ($) =>
      seq(
        kw("record"),
        field("table", choice($.identifier, $.quoted_identifier)),
        optional(kw("temporary"))
      ),

    enum_type: ($) =>
      seq(kw("enum"), field("name", choice($.identifier, $.quoted_identifier))),

    list_type: ($) =>
      seq(kw("list"), kw("of"), "[", $._type_reference, "]"),

    array_type: ($) =>
      seq(
        kw("array"),
        "[",
        commaSep1($.integer_literal),
        "]",
        kw("of"),
        $._type_reference
      ),

    option_type: ($) =>
      seq(kw("option"), choice($.identifier, $.quoted_identifier), repeat(seq(",", choice($.identifier, $.quoted_identifier)))),

    label_type: ($) =>
      seq(kw("label"), $.string_literal),

    dotnet_type: ($) =>
      seq(kw("dotnet"), $.string_literal),

    // ─── Terminals ───────────────────────────────────────────

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    quoted_identifier: ($) => /"[^"]*"/,

    string_literal: ($) => /'[^']*'/,

    integer_literal: ($) => /\d+/,

    decimal_literal: ($) => /\d+\.\d+/,

    // Use token(prec()) to ensure these win over identifier at the lexer level
    boolean_literal: ($) => token(prec(1, choice(
      /[tT][rR][uU][eE]/,
      /[fF][aA][lL][sS][eE]/
    ))),

    line_comment: ($) => token(seq("//", /.*/)),

    block_comment: ($) => token(seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")),
  },
});
