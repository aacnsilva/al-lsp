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

  extras: ($) => [/\s/, $.line_comment, $.block_comment, $.region_directive, $.pragma_directive],

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
      seq(
        kw("enum"),
        $.integer_literal,
        $._object_name,
        optional($.implements_clause),
        "{",
        optional($._enum_body),
        "}"
      ),

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
          $.requestpage_section,
          $.labels_section,
          $._named_section
        )
      ),

    _enum_body: ($) =>
      repeat1(choice($.property, $.enum_value_declaration)),

    _interface_body: ($) =>
      repeat1(
        choice(
          $.property,
          $.interface_method
        )
      ),

    // Method signature without a body — used in interfaces.
    interface_method: ($) =>
      seq(
        repeat($.attribute),
        optional(field("access", choice(kw("local"), kw("internal"), kw("protected")))),
        kw("procedure"),
        field("name", choice($.identifier, $.quoted_identifier)),
        "(",
        field("parameters", optional($.parameter_list)),
        ")",
        optional(field("return_type", $.interface_return_type)),
        optional(";")
      ),

    // ─── Property ────────────────────────────────────────────

    property: ($) =>
      seq(
        field("name", $.property_name),
        "=",
        field("value", $._property_value),
        ";"
      ),

    property_name: ($) =>
      choice(
        $.identifier,
        $.quoted_identifier,
        // Handle cases where keywords are tokenized as a prefix of property names
        // (e.g. `TableRelation` -> `Table` + `Relation`).
        seq(kw("table"), $.identifier),
        // Handle `ToolTip` tokenized as `to` + `olTip`.
        seq(kw("to"), $.identifier)
      ),

    _property_value: ($) =>
      choice(
        $.table_relation_expression,
        $.calc_formula_expression,
        $.table_view_expression,
        $.dataitem_link_expression,
        $.property_option_value,
        $.string_literal,
        $.integer_literal,
        $.decimal_literal,
        $.temporal_literal,
        $.boolean_literal,
        $.identifier,
        $.quoted_identifier,
        // Enum-like: Sorting::Ascending
        seq($.identifier, "::", $.identifier),
        // Property list: field1, field2
        commaSep1(choice($.identifier, $.quoted_identifier))
      ),

    table_view_expression: ($) =>
      seq($.table_view_clause, repeat($.table_view_clause)),

    table_view_clause: ($) =>
      choice(
        seq(kw("sorting"), "(", commaSep1(choice($.identifier, $.quoted_identifier)), ")"),
        seq(kw("order"), "(", choice(kw("ascending"), kw("descending"), $.identifier, $.quoted_identifier), ")"),
        $.where_clause
      ),

    dataitem_link_expression: ($) =>
      commaSep1($.dataitem_link_condition),

    dataitem_link_condition: ($) =>
      seq(
        field("field", choice($.identifier, $.quoted_identifier, $.member_access)),
        "=",
        field(
          "value",
          choice(
            $.field_reference,
            $.const_reference,
            $.filter_reference,
            $.identifier,
            $.quoted_identifier,
            $.string_literal,
            $.integer_literal,
            $.decimal_literal,
            $.temporal_literal,
            $.boolean_literal
          )
        )
      ),

    property_option_value: ($) =>
      seq($.string_literal, repeat1(seq(",", $.property_option_assignment))),

    property_option_assignment: ($) =>
      seq(
        choice($.identifier, $.quoted_identifier),
        "=",
        choice(
          $.string_literal,
          $.integer_literal,
          $.decimal_literal,
          $.temporal_literal,
          $.boolean_literal,
          $.identifier,
          $.quoted_identifier
        )
      ),

    table_relation_expression: ($) =>
      choice(
        $.table_relation_if_expression,
        $.table_relation_target_expression
      ),

    table_relation_target_expression: ($) =>
      choice(
        seq(
          field("relation", $.member_access),
          optional(field("where", $.where_clause))
        ),
        seq(
          field("relation", choice($.identifier, $.quoted_identifier)),
          field("where", $.where_clause)
        )
      ),

    table_relation_if_expression: ($) =>
      prec.right(
        seq(
          kw("if"),
          "(",
          $.table_relation_condition,
          ")",
          $.table_relation_target_expression,
          optional(
            seq(
              kw("else"),
              choice(
                $.table_relation_if_expression,
                $.table_relation_target_expression
              )
            )
          )
        )
      ),

    table_relation_condition: ($) =>
      $.where_condition,

    calc_formula_expression: ($) =>
      seq(
        field(
          "method",
          choice(
            kw("lookup"),
            kw("sum"),
            kw("average"),
            kw("count"),
            kw("exist"),
            kw("min"),
            kw("max")
          )
        ),
        "(",
        field("source", $._relation_target),
        optional(field("where", $.where_clause)),
        ")"
      ),

    _relation_target: ($) =>
      choice($.member_access, $.identifier, $.quoted_identifier),

    where_clause: ($) =>
      seq(kw("where"), "(", commaSep1($.where_condition), ")"),

    where_condition: ($) =>
      seq(
        field("field", choice($.identifier, $.quoted_identifier)),
        "=",
        field("value", $.where_value_expression)
      ),

    where_value_expression: ($) =>
      choice($.field_reference, $.const_reference, $.filter_reference),

    field_reference: ($) =>
      seq(
        kw("field"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ")"
      ),

    const_reference: ($) =>
      seq(
        kw("const"),
        "(",
        field("value", $._const_value),
        ")"
      ),

    _const_value: ($) =>
      choice(
        $.string_literal,
        $.integer_literal,
        $.decimal_literal,
        $.temporal_literal,
        $.boolean_literal,
        $.identifier,
        $.quoted_identifier
      ),

    filter_reference: ($) =>
      seq(
        kw("filter"),
        "(",
        field("pattern", $.filter_pattern),
        ")"
      ),

    filter_pattern: ($) => token(prec(1, /([^)]|'[^']*')+/)),

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
      seq(kw("dataset"), "{", repeat(choice($._named_section, $.dataitem_declaration)), "}"),

    requestpage_section: ($) =>
      seq(
        kw("requestpage"),
        "{",
        repeat(
          choice(
            $.property,
            $.layout_section,
            $.actions_section,
            $.trigger_declaration,
            $.procedure_declaration,
            $.var_section,
            $._named_section
          )
        ),
        "}"
      ),

    labels_section: ($) =>
      seq(kw("labels"), "{", repeat($.property), "}"),

    _named_section: ($) =>
      choice(
        $.area_section,
        $.group_section,
        $.page_field,
        $.page_action,
        $.part_section,
        $.repeater_section,
        $.usercontrol_section
      ),

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

    dataitem_declaration: ($) =>
      seq(
        kw("dataitem"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        field("source", choice($.identifier, $.quoted_identifier, $.member_access)),
        ")",
        "{",
        repeat(
          choice(
            $.property,
            $.column_declaration,
            $.trigger_declaration,
            $.procedure_declaration,
            $.var_section,
            $.dataitem_declaration
          )
        ),
        "}"
      ),

    column_declaration: ($) =>
      seq(
        kw("column"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        field("expression", $._expression),
        ")",
        "{",
        repeat(choice($.property, $.trigger_declaration)),
        "}"
      ),

    usercontrol_section: ($) =>
      seq(
        kw("usercontrol"),
        "(",
        field("name", choice($.identifier, $.quoted_identifier)),
        ";",
        field("addin", choice($.identifier, $.quoted_identifier)),
        ")",
        "{",
        repeat(
          choice(
            $.property,
            $.trigger_declaration,
            $.procedure_declaration,
            $.var_section
          )
        ),
        "}"
      ),

    // ─── Attributes ─────────────────────────────────────────

    attribute: ($) =>
      seq("[", $.identifier, optional(seq("(", $.attribute_arguments, ")")), "]"),

    attribute_arguments: ($) =>
      commaSep1($._expression),

    // ─── Procedures & triggers ───────────────────────────────

    procedure_declaration: ($) =>
      seq(
        repeat($.attribute),
        optional(field("access", choice(kw("local"), kw("internal"), kw("protected")))),
        kw("procedure"),
        field("name", choice($.identifier, $.quoted_identifier)),
        "(",
        field("parameters", optional($.parameter_list)),
        ")",
        optional(field("return_type", $.return_type)),
        optional(";"),
        optional(field("vars", $.var_section)),
        field("body", $.block)
      ),

    trigger_declaration: ($) =>
      seq(
        repeat($.attribute),
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

    return_type: ($) =>
      choice(
        seq(":", field("type", $._type_reference)),
        seq(
          field("name", choice($.identifier, $.quoted_identifier)),
          ":",
          field("type", $._type_reference)
        )
      ),

    // Interface signatures support both `: Type` and `ReturnName: Type`.
    interface_return_type: ($) =>
      choice(
        seq(":", field("type", $._type_reference)),
        seq(
          field("name", $.named_return_header),
          field("type", $._type_reference)
        )
      ),

    // ─── Variable declarations ───────────────────────────────

    var_section: ($) =>
      seq(kw("var"), repeat($.variable_declaration)),

    variable_declaration: ($) =>
      seq(
        repeat($.attribute),
        commaSep1(field("name", choice($.identifier, $.quoted_identifier))),
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
        $.compound_assignment_statement,
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

    compound_assignment_statement: ($) =>
      seq(
        field("target", $._expression),
        field("operator", choice("+=", "-=", "*=", "/=")),
        field("value", $._expression)
      ),

    expression_statement: ($) => prec(-1, $._expression),

    if_statement: ($) =>
      prec.right(
        seq(
          kw("if"),
          field("condition", $._expression),
          kw("then"),
          optional(field("consequence", $._statement)),
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

    _expression: ($) => $.conditional_expression,

    conditional_expression: ($) =>
      prec.right(
        seq(
          field("condition", $.or_expression),
          optional(
            seq(
              "?",
              field("consequence", $._expression),
              ":",
              field("alternative", $._expression)
            )
          )
        )
      ),

    or_expression: ($) =>
      choice(
        $.and_expression,
        prec.left(
          1,
          seq(
            field("left", $.or_expression),
            field("operator", choice(kw("or"), kw("xor"))),
            field("right", $.and_expression)
          )
        )
      ),

    and_expression: ($) =>
      choice(
        $.comparison_expression,
        prec.left(
          2,
          seq(
            field("left", $.and_expression),
            field("operator", kw("and")),
            field("right", $.comparison_expression)
          )
        )
      ),

    comparison_expression: ($) =>
      choice(
        $.range_expression,
        prec.left(
          3,
          seq(
            field("left", $.range_expression),
            field("operator", choice("=", "<>", "<", ">", "<=", ">=", kw("in"))),
            field("right", $.range_expression)
          )
        )
      ),

    range_expression: ($) =>
      choice(
        $.additive_expression,
        prec.left(
          4,
          seq(
            field("left", $.range_expression),
            field("operator", ".."),
            field("right", $.additive_expression)
          )
        )
      ),

    additive_expression: ($) =>
      choice(
        $.multiplicative_expression,
        prec.left(
          5,
          seq(
            field("left", $.additive_expression),
            field("operator", choice("+", "-")),
            field("right", $.multiplicative_expression)
          )
        )
      ),

    multiplicative_expression: ($) =>
      choice(
        $._unary_expression,
        prec.left(
          6,
          seq(
            field("left", $.multiplicative_expression),
            field("operator", choice("*", "/", kw("mod"), kw("div"))),
            field("right", $._unary_expression)
          )
        )
      ),

    _unary_expression: ($) =>
      choice(
        $.not_expression,
        $.negation_expression,
        $._postfix_expression
      ),

    not_expression: ($) =>
      prec.right(7, seq(kw("not"), $._unary_expression)),

    negation_expression: ($) =>
      prec.right(7, seq("-", $._unary_expression)),

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
        $.builtin_object_reference,
        $.string_literal,
        $.integer_literal,
        $.decimal_literal,
        $.temporal_literal,
        $.boolean_literal,
        $.list_literal,
        $.parenthesized_expression,
        $.function_call,
        $.qualified_enum_value,
        $.qualified_object_reference
      ),

    list_literal: ($) =>
      seq("[", commaSep1($._expression), "]"),

    builtin_object_reference: ($) =>
      choice(
        kw("codeunit"),
        kw("page"),
        kw("report"),
        kw("query"),
        kw("xmlport")
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
      choice(
        prec.left(
          8,
          seq(
            field(
              "enum",
              choice(
                $.identifier,
                $.quoted_identifier,
                $.member_access,
                $.enum_qualified_name
              )
            ),
            "::",
            field("value", choice($.identifier, $.quoted_identifier))
          )
        ),
        prec.left(
          7,
          seq(
            field(
              "enum",
              choice(
                $.identifier,
                $.quoted_identifier,
                $.member_access,
                $.enum_qualified_name
              )
            ),
            "::"
          )
        )
      ),

    qualified_object_reference: ($) =>
      prec.left(
        7,
        seq(
          field(
            "kind",
            choice(
              kw("enum"),
              kw("codeunit"),
              kw("table"),
              kw("page"),
              kw("report"),
              kw("query"),
              kw("xmlport"),
              kw("interface")
            )
          ),
          "::",
          field("name", choice($.identifier, $.quoted_identifier))
        )
      ),

    enum_qualified_name: ($) =>
      seq(
        kw("enum"),
        "::",
        field("name", choice($.identifier, $.quoted_identifier))
      ),

    argument_list: ($) => commaSep1($._expression),

    // ─── Type references ─────────────────────────────────────

    _type_reference: ($) =>
      choice(
        $.sized_type,
        $.simple_type,
        $.record_type,
        $.codeunit_type,
        $.page_type,
        $.testpage_type,
        $.report_type,
        $.testrequestpage_type,
        $.query_type,
        $.xmlport_type,
        $.enum_type,
        $.interface_type,
        $.dictionary_type,
        $.list_type,
        $.array_type,
        $.option_type,
        $.label_type,
        $.dotnet_type
      ),

    // Type references allowed inside generic container declarations such as
    // `List of [T]` and `Dictionary of [K, V]`. Excludes comma-heavy forms
    // (`option` and `label`) to avoid ambiguity with generic separators.
    _generic_type_reference: ($) =>
      choice(
        $.sized_type,
        $.simple_type,
        $.record_type,
        $.codeunit_type,
        $.page_type,
        $.testpage_type,
        $.report_type,
        $.testrequestpage_type,
        $.query_type,
        $.xmlport_type,
        $.enum_type,
        $.interface_type,
        $.dictionary_type,
        $.list_type,
        $.dotnet_type
      ),

    simple_type: ($) => prec(-1, choice($.identifier, $.quoted_identifier)),

    sized_type: ($) =>
      prec(1, seq(
        field("base", $.identifier),
        "[",
        field("size", $.integer_literal),
        "]"
      )),

    record_type: ($) =>
      seq(
        kw("record"),
        field("table", choice($.identifier, $.quoted_identifier)),
        optional(kw("temporary"))
      ),

    codeunit_type: ($) =>
      seq(kw("codeunit"), field("name", choice($.identifier, $.quoted_identifier))),

    page_type: ($) =>
      seq(kw("page"), field("name", choice($.identifier, $.quoted_identifier))),

    testpage_type: ($) =>
      seq(kw("testpage"), field("name", choice($.identifier, $.quoted_identifier))),

    report_type: ($) =>
      seq(kw("report"), field("name", choice($.identifier, $.quoted_identifier))),

    testrequestpage_type: ($) =>
      seq(kw("testrequestpage"), field("name", choice($.identifier, $.quoted_identifier))),

    query_type: ($) =>
      seq(kw("query"), field("name", choice($.identifier, $.quoted_identifier))),

    xmlport_type: ($) =>
      seq(kw("xmlport"), field("name", choice($.identifier, $.quoted_identifier))),

    enum_type: ($) =>
      seq(kw("enum"), field("name", choice($.identifier, $.quoted_identifier))),

    interface_type: ($) =>
      seq(kw("interface"), field("name", choice($.identifier, $.quoted_identifier))),

    dictionary_type: ($) =>
      seq(
        kw("dictionary"),
        kw("of"),
        "[",
        field("key_type", $._generic_type_reference),
        ",",
        field("value_type", $._generic_type_reference),
        "]"
      ),

    list_type: ($) =>
      seq(kw("list"), kw("of"), "[", field("element_type", $._generic_type_reference), "]"),

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
      prec.right(
        seq(
          kw("option"),
          optional(
            seq(
              choice($.identifier, $.quoted_identifier),
              repeat(seq(",", choice($.identifier, $.quoted_identifier)))
            )
          )
        )
      ),

    label_type: ($) =>
      seq(kw("label"), $.string_literal, repeat(seq(",", $.label_option))),

    label_option: ($) =>
      seq(
        choice($.identifier, $.quoted_identifier),
        "=",
        choice(
          $.string_literal,
          $.integer_literal,
          $.decimal_literal,
          $.temporal_literal,
          $.boolean_literal,
          $.identifier,
          $.quoted_identifier
        )
      ),

    dotnet_type: ($) =>
      seq(kw("dotnet"), $.string_literal),

    // ─── Terminals ───────────────────────────────────────────

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    quoted_identifier: ($) => /"[^"]*"/,

    // Used in interface method signatures with named return values:
    // `procedure Foo() ReturnVar: Code[20]`
    named_return_header: ($) => token(/[a-zA-Z_][a-zA-Z0-9_]*\s*:/),

    // AL strings escape single quote as doubled single quote: ''.
    string_literal: ($) => /'([^']|'')*'/,

    integer_literal: ($) => /\d+/,

    decimal_literal: ($) => /\d+\.\d+/,

    // AL temporal constants such as 0D, 0T, 0DT.
    temporal_literal: ($) => token(prec(2, /\d+([dD][tT]|[dD]|[tT])/)),

    // Use token(prec()) to ensure these win over identifier at the lexer level
    boolean_literal: ($) => token(prec(1, choice(
      /[tT][rR][uU][eE]/,
      /[fF][aA][lL][sS][eE]/
    ))),

    line_comment: ($) => token(seq("//", /.*/)),

    block_comment: ($) => token(seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")),

    region_directive: ($) =>
      token(
        choice(
          seq(/#[rR][eE][gG][iI][oO][nN]/, /.*/),
          seq(/#[eE][nN][dD][rR][eE][gG][iI][oO][nN]/, /.*/)
        )
      ),

    pragma_directive: ($) =>
      token(seq(/#[pP][rR][aA][gG][mM][aA]/, /.*/)),
  },
});
