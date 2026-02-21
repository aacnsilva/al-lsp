use tree_sitter::{Node, Tree};

/// Options for document formatting.
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub tab_size: usize,
    pub insert_spaces: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions {
            tab_size: 4,
            insert_spaces: true,
        }
    }
}

/// Format an AL document using the parse tree to determine structure.
///
/// Walks the CST to compute proper indentation for each line, applies basic
/// spacing rules (around `:=`, after `,`, etc.), and preserves the user's
/// line break decisions where possible.
pub fn format_document(tree: &Tree, source: &str, options: &FormatOptions) -> String {
    let indent_str = if options.insert_spaces {
        " ".repeat(options.tab_size)
    } else {
        "\t".to_string()
    };

    // Build a map: line_number -> indent_level.
    // We use Option to track which lines have been explicitly set.
    let line_count = source.lines().count().max(1);
    let mut indent_levels: Vec<Option<i32>> = vec![None; line_count + 1];

    // Track lines whose content should be joined to the previous line
    // (e.g., `begin` after `for...do`)
    let mut join_to_prev: Vec<bool> = vec![false; line_count + 1];

    // Track lines that should be removed (blank lines between var and begin)
    let mut remove_line: Vec<bool> = vec![false; line_count + 1];

    // Walk the tree to assign indent levels and join markers
    assign_indentation(
        tree.root_node(),
        source,
        &mut indent_levels,
        &mut join_to_prev,
        &mut remove_line,
        0,
    );

    // Apply indentation, line joining, and basic spacing to each line
    let mut result = String::with_capacity(source.len());
    let source_lines: Vec<&str> = source.lines().collect();
    let mut line_idx = 0;
    while line_idx < source_lines.len() {
        let line = source_lines[line_idx];
        let trimmed = line.trim();

        // Skip removed lines
        if remove_line.get(line_idx).copied().unwrap_or(false) {
            line_idx += 1;
            continue;
        }

        if trimmed.is_empty() {
            // Preserve blank lines
            result.push('\n');
            line_idx += 1;
            continue;
        }

        // Check if this line should be joined to the previous line
        if join_to_prev.get(line_idx).copied().unwrap_or(false) && !result.is_empty() {
            // Remove the trailing newline from the previous line
            if result.ends_with('\n') {
                result.pop();
            }
            result.push(' ');
            let formatted_line = apply_spacing(trimmed);
            result.push_str(&formatted_line);
            result.push('\n');
            line_idx += 1;
            continue;
        }

        let level = indent_levels[line_idx].unwrap_or(0).max(0) as usize;
        let indent = indent_str.repeat(level);

        // Apply basic spacing rules to the trimmed line content
        let formatted_line = apply_spacing(trimmed);

        result.push_str(&indent);
        result.push_str(&formatted_line);
        result.push('\n');
        line_idx += 1;
    }

    // Ensure file ends with exactly one newline
    while result.ends_with("\n\n") {
        result.pop();
    }
    if !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

/// Set the indent level for `line` only if it hasn't been set yet.
/// The first token to claim a line wins (leftmost on that line).
fn set_line_indent(levels: &mut [Option<i32>], line: usize, depth: i32) {
    if line < levels.len() && levels[line].is_none() {
        levels[line] = Some(depth);
    }
}

/// Recursively walk the CST and assign indentation to each line.
///
/// `depth` is the indentation level of this node itself.
/// The node decides what depth to pass to each of its children.
fn assign_indentation(
    node: Node,
    source: &str,
    levels: &mut [Option<i32>],
    join_to_prev: &mut [bool],
    remove_line: &mut [bool],
    depth: i32,
) {
    let kind = node.kind();
    let start_line = node.start_position().row;
    let end_line = node.end_position().row;

    // Leaf nodes: set the indent for their line
    if node.child_count() == 0 {
        set_line_indent(levels, start_line, depth);
        return;
    }

    // --- Special handling for nodes with implicit keywords ---

    // `block` = begin <stmts> end [;]
    // `begin` and `end` are NOT child nodes — they're implicit in the block span.
    // We must set begin (start_line) and end (end_line) explicitly.
    if kind == "block" {
        // `begin` line at this depth
        set_line_indent(levels, start_line, depth);
        // `end` line at this depth
        set_line_indent(levels, end_line, depth);
        // All children (statements, semicolons) are indented inside the block
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
        }
        return;
    }

    // `var_section` = var <declarations>
    // `var` keyword is NOT a child node — it's implicit at the start of the span.
    if kind == "var_section" {
        // `var` line at this depth
        set_line_indent(levels, start_line, depth);
        // Variable declarations are indented
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
        }
        return;
    }

    // --- Object declarations ---
    // The object keyword (codeunit, table, etc.) is NOT a child node.
    // Children on the declaration line (id, name) should be at parent_depth.
    // `{`/`}` at parent_depth, body members at parent_depth + 1.
    if is_object_declaration(kind) {
        // Set the object declaration's own start line (the keyword line)
        set_line_indent(levels, start_line, depth);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_line = child.start_position().row;
            let child_kind = child.kind();
            // Children on the same line as the object keyword (id, name, implements_clause)
            if child_line == start_line {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            } else if child_kind == "{" || child_kind == "}" {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            } else {
                // Body members: procedures, triggers, properties, sections
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            }
        }
        return;
    }

    // --- Braced sections: fields, keys, layout, actions, etc. ---
    // Section keyword is implicit at start_line.
    if is_braced_section(kind) {
        set_line_indent(levels, start_line, depth);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            if child_kind == "{" || child_kind == "}" {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            }
        }
        return;
    }

    // --- Field/key/enum_value declarations with optional brace body ---
    if is_field_like_declaration(kind) {
        set_line_indent(levels, start_line, depth);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_line = child.start_position().row;
            let child_kind = child.kind();
            if child_line == start_line {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            } else if child_kind == "{" || child_kind == "}" {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            }
        }
        return;
    }

    // --- Procedure / Trigger declarations ---
    // `procedure` keyword is implicit. All direct children at same depth.
    // Also: remove blank lines between var_section and block.
    if matches!(kind, "procedure_declaration" | "trigger_declaration") {
        set_line_indent(levels, start_line, depth);
        let mut prev_child_end: Option<usize> = Some(start_line);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            let child_start = child.start_position().row;

            // Remove blank lines between any parts of the procedure/trigger
            // (signature → var, var → begin, signature → begin).
            if matches!(child_kind, "var_section" | "block") {
                if let Some(prev_end) = prev_child_end {
                    for blank_line in (prev_end + 1)..child_start {
                        let line_text = source.lines().nth(blank_line).unwrap_or("");
                        if line_text.trim().is_empty() && blank_line < remove_line.len() {
                            remove_line[blank_line] = true;
                        }
                    }
                }
            }

            prev_child_end = Some(child.end_position().row);
            assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
        }
        return;
    }

    // --- if_statement ---
    // `if`, `then`, `else` keywords are implicit (not child nodes).
    // Children: condition expression, consequence block, [alternative block]
    if kind == "if_statement" {
        set_line_indent(levels, start_line, depth);
        let mut first_block_end: Option<usize> = None;
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            if child_kind == "block" {
                if let Some(_) = first_block_end {
                    // This is the alternative (else) block.
                    // The `else` keyword is on a line between the first block's end
                    // and this block's start. Set it at the if-statement's depth.
                    let alt_start = child.start_position().row;
                    if alt_start > 0 {
                        for else_line in (0..alt_start).rev() {
                            let line_text = source.lines().nth(else_line).unwrap_or("");
                            let trimmed = line_text.trim().to_lowercase();
                            if trimmed.starts_with("else") {
                                set_line_indent(levels, else_line, depth);
                                break;
                            }
                            if !trimmed.is_empty() {
                                break;
                            }
                        }
                    }
                }
                first_block_end = Some(child.end_position().row);
                // consequence/alternative blocks indented
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            }
        }
        return;
    }

    // --- for/while/with statements ---
    // When the body is a `block`, join `begin` to the statement line,
    // put `end` at the statement's depth, body at depth + 2.
    if matches!(kind, "for_statement" | "while_statement" | "with_statement") {
        set_line_indent(levels, start_line, depth);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            if child_kind == "block" {
                let block_start = child.start_position().row;
                let block_end = child.end_position().row;

                // If `begin` is on a different line from the statement, join it
                if block_start != start_line && block_start < join_to_prev.len() {
                    join_to_prev[block_start] = true;
                }

                // `end` at the for-statement's depth
                set_line_indent(levels, block_end, depth);

                // Body statements at depth + 2
                // (begin is joined to for-line at depth, body needs extra indent)
                let mut block_cursor = child.walk();
                for block_child in child.children(&mut block_cursor) {
                    assign_indentation(
                        block_child,
                        source,
                        levels,
                        join_to_prev,
                        remove_line,
                        depth + 2,
                    );
                }
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            }
        }
        return;
    }

    // --- repeat_statement ---
    // `repeat_statement` = repeat <stmts> until <condition>
    // `repeat` and `until` keywords are NOT child nodes.
    // `repeat` (start_line) and `until` (end_line) at this depth,
    // body statements indented one level.
    if kind == "repeat_statement" {
        set_line_indent(levels, start_line, depth);
        set_line_indent(levels, end_line, depth);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_start = child.start_position().row;
            if child_start == end_line {
                // The condition expression is on the `until` line — same depth
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            }
        }
        return;
    }

    // --- case_statement ---
    if kind == "case_statement" {
        set_line_indent(levels, start_line, depth);
        set_line_indent(levels, end_line, depth); // `end` line
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            if child_kind == "case_branch" {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            }
        }
        return;
    }

    // --- case_branch ---
    if kind == "case_branch" {
        set_line_indent(levels, start_line, depth);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let child_kind = child.kind();
            if child_kind == "block" {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth + 1);
            } else {
                assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
            }
        }
        return;
    }

    // --- Default: pass depth through to children ---
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        assign_indentation(child, source, levels, join_to_prev, remove_line, depth);
    }
}

fn is_field_like_declaration(kind: &str) -> bool {
    matches!(
        kind,
        "field_declaration"
            | "key_declaration"
            | "enum_value_declaration"
            | "page_field"
            | "page_action"
    )
}

/// Apply basic spacing rules to a trimmed line of AL code.
fn apply_spacing(line: &str) -> String {
    if line.is_empty() {
        return String::new();
    }

    // Don't modify comment lines
    if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
        return line.to_string();
    }

    let mut result = String::with_capacity(line.len() + 16);
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Inside string literals — preserve as-is
        if ch == '\'' {
            result.push(ch);
            i += 1;
            while i < len && chars[i] != '\'' {
                result.push(chars[i]);
                i += 1;
            }
            if i < len {
                result.push(chars[i]); // closing quote
                i += 1;
            }
            continue;
        }

        // Assignment operator :=
        if ch == ':' && i + 1 < len && chars[i + 1] == '=' {
            // Ensure space before :=
            if !result.ends_with(' ') && !result.is_empty() {
                result.push(' ');
            }
            result.push_str(":=");
            i += 2;
            // Ensure space after :=
            if i < len && chars[i] != ' ' {
                result.push(' ');
            }
            continue;
        }

        // Compound assignment operators: +=, -=, *=, /=
        if (ch == '+' || ch == '-' || ch == '*' || ch == '/') && i + 1 < len && chars[i + 1] == '='
        {
            // Ensure space before the operator
            if !result.ends_with(' ') && !result.is_empty() {
                result.push(' ');
            }
            result.push(ch);
            result.push('=');
            i += 2;
            // Ensure space after the operator
            if i < len && chars[i] != ' ' {
                result.push(' ');
            }
            continue;
        }

        // Comparison operators: <>, <=, >=
        if (ch == '<' || ch == '>') && i + 1 < len {
            let next = chars[i + 1];
            if next == '>' || next == '=' {
                if !result.ends_with(' ') && !result.is_empty() {
                    result.push(' ');
                }
                result.push(ch);
                result.push(next);
                i += 2;
                if i < len && chars[i] != ' ' {
                    result.push(' ');
                }
                continue;
            }
        }

        // Single comparison operators: <, >
        if (ch == '<' || ch == '>')
            && (i + 1 >= len || (chars[i + 1] != '>' && chars[i + 1] != '='))
        {
            if !result.ends_with(' ') && !result.is_empty() {
                result.push(' ');
            }
            result.push(ch);
            i += 1;
            if i < len && chars[i] != ' ' {
                result.push(' ');
            }
            continue;
        }

        // Comma: ensure space after, no space before
        if ch == ',' {
            while result.ends_with(' ') {
                result.pop();
            }
            result.push(',');
            i += 1;
            if i < len && chars[i] != ' ' {
                result.push(' ');
            }
            continue;
        }

        // Semicolon: no space before
        if ch == ';' {
            while result.ends_with(' ') {
                result.pop();
            }
            result.push(';');
            i += 1;
            if i < len && chars[i] != ' ' && chars[i] != '\n' {
                result.push(' ');
            }
            continue;
        }

        // Collapse multiple spaces into one
        if ch == ' ' && result.ends_with(' ') {
            i += 1;
            continue;
        }

        result.push(ch);
        i += 1;
    }

    result.trim_end().to_string()
}

// --- Helper predicates ---

fn is_object_declaration(kind: &str) -> bool {
    matches!(
        kind,
        "codeunit_declaration"
            | "table_declaration"
            | "table_extension_declaration"
            | "page_declaration"
            | "page_extension_declaration"
            | "report_declaration"
            | "enum_declaration"
            | "enum_extension_declaration"
            | "xmlport_declaration"
            | "query_declaration"
            | "interface_declaration"
            | "permissionset_declaration"
            | "controladdin_declaration"
    )
}

fn is_braced_section(kind: &str) -> bool {
    matches!(
        kind,
        "fields_section"
            | "keys_section"
            | "fieldgroups_section"
            | "layout_section"
            | "actions_section"
            | "dataset_section"
            | "requestpage_section"
            | "area_section"
            | "group_section"
            | "repeater_section"
            | "part_section"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format(source: &str) -> String {
        let tree = al_parser::parse(source).expect("failed to parse");
        let options = FormatOptions::default();
        format_document(&tree, source, &options)
    }

    #[test]
    fn test_basic_codeunit_indentation() {
        let input = r#"codeunit 50100 Test
{
procedure Hello()
begin
end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure Hello()
    begin
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_procedure_with_var_and_body() {
        let input = r#"codeunit 50100 Test
{
procedure DoWork()
var
MyVar: Integer;
begin
MyVar := 42;
Message(MyVar);
end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        MyVar: Integer;
    begin
        MyVar := 42;
        Message(MyVar);
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_if_statement() {
        let input = r#"codeunit 50100 Test
{
procedure DoWork()
begin
if x > 0 then
begin
y := 1;
end
else
begin
y := 2;
end;
end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure DoWork()
    begin
        if x > 0 then
            begin
                y := 1;
            end
        else
            begin
                y := 2;
            end;
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_table_with_fields_and_keys() {
        let input = r#"table 50100 Customer
{
fields
{
field(1; Name; Text[100])
{
Caption = 'Name';
}
}
keys
{
key(PK; Name)
{
Clustered = true;
}
}
}"#;
        let result = format(input);
        let expected = r#"table 50100 Customer
{
    fields
    {
        field(1; Name; Text[100])
        {
            Caption = 'Name';
        }
    }
    keys
    {
        key(PK; Name)
        {
            Clustered = true;
        }
    }
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_interface_indentation() {
        let input = r#"interface IAddressProvider
{
procedure GetAddress(): Text;
}"#;
        let result = format(input);
        let expected = r#"interface IAddressProvider
{
    procedure GetAddress(): Text;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_preserves_blank_lines() {
        let input = r#"codeunit 50100 Test
{

    procedure Hello()
    begin
    end;

    procedure World()
    begin
    end;

}"#;
        let result = format(input);
        assert!(result.contains("\n\n"), "blank lines should be preserved");
    }

    #[test]
    fn test_spacing_around_assignment() {
        let input = r#"codeunit 50100 Test
{
procedure DoWork()
var
X: Integer;
begin
X:=42;
end;
}"#;
        let result = format(input);
        assert!(
            result.contains("X := 42;"),
            "expected spaces around :=, got:\n{result}"
        );
    }

    #[test]
    fn test_already_formatted_unchanged() {
        let input = r#"codeunit 50100 "Hello World"
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
"#;
        let result = format(input);
        assert_eq!(result, input, "already-formatted code should not change");
    }

    #[test]
    fn test_spacing_after_comma() {
        let input = r#"codeunit 50100 Test
{
procedure DoWork(A: Integer;B: Text)
begin
Foo(1,2,3);
end;
}"#;
        let result = format(input);
        assert!(
            result.contains("Foo(1, 2, 3);"),
            "expected spaces after commas, got:\n{result}"
        );
    }

    #[test]
    fn test_for_loop_begin_joins_do_line() {
        let input = r#"codeunit 50100 Test
{
    procedure HelloWorld()
    var
        Counter: Integer;
    begin
        for Counter := 1 to 5 do
            begin
                Counter += 1;
            end;
    end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure HelloWorld()
    var
        Counter: Integer;
    begin
        for Counter := 1 to 5 do begin
                Counter += 1;
        end;
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_blank_lines_between_var_and_begin() {
        let input = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;

    begin
        X := 1;
    end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X := 1;
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_blank_lines_between_procedure_and_var() {
        let input = r#"codeunit 50100 Test
{
    procedure HelloWorld()

    var
        Counter: Integer;
        IAddressProvider: Interface IAddressProvider;
    begin
        Counter := 1;
    end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure HelloWorld()
    var
        Counter: Integer;
        IAddressProvider: Interface IAddressProvider;
    begin
        Counter := 1;
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_repeat_until_indentation() {
        let input = r#"codeunit 50100 Test
{
procedure DoWork()
var
Counter: Integer;
begin
repeat
Counter += 1;
until Counter = 0;
end;
}"#;
        let result = format(input);
        let expected = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        Counter: Integer;
    begin
        repeat
            Counter += 1;
        until Counter = 0;
    end;
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compound_operator_spacing() {
        let input = r#"codeunit 50100 Test
{
    procedure DoWork()
    var
        X: Integer;
    begin
        X+=1;
        X-=2;
        X*=3;
        X/=4;
    end;
}"#;
        let result = format(input);
        assert!(
            result.contains("X += 1;"),
            "expected spaces around +=, got:\n{result}"
        );
        assert!(
            result.contains("X -= 2;"),
            "expected spaces around -=, got:\n{result}"
        );
        assert!(
            result.contains("X *= 3;"),
            "expected spaces around *=, got:\n{result}"
        );
        assert!(
            result.contains("X /= 4;"),
            "expected spaces around /=, got:\n{result}"
        );
    }
}
