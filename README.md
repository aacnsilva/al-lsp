# al-lsp

A Language Server Protocol (LSP) implementation for the AL programming language (Microsoft Dynamics 365 Business Central), written in Rust with a custom tree-sitter grammar.

## Features

| Feature | Description |
|---|---|
| **Go to Definition** | Navigate to symbol declarations. Supports cross-document object/type/member navigation, implementation procedure -> interface method, qualified enum values, `TableRelation` targets (table + related field), and page `usercontrol` calls into `controladdin` procedures. |
| **Go to Implementation** | From an interface method to all implementing codeunit procedures across open documents. |
| **Go to Type Definition** | Resolves variable types to their object declarations (Record, Codeunit, Page, etc.). |
| **Find References** | Scope-aware, cross-document. Supports interface methods, implementation procedures, and interface-typed method calls. Respects shadowing. |
| **Hover** | Displays symbol kind/name/type, including qualified enum values, inferred record field types, inline `Option` variable members, and built-in method signatures/summaries with Microsoft Learn links (covering the full AL runtime data-type method library list). |
| **Completion** | Triggered by `.` and `::`. Includes scoped symbols, enum values (`Enum::Value` and `Rec."Enum Field"::Value`), inline `Option` variable values (`OptionVar::Value`), `TableRelation`/`WHERE` value expression contexts, record/table members, `controladdin` procedures through `CurrPage.<Control>.`, built-in method/property documentation, and chained built-in return-type inference (including no-`()` calls for zero-parameter methods). Built-in datatype methods are aligned with the Microsoft Learn AL method library tables. |
| **Signature Help** | Triggered by `(` and `,`. Shows procedure signatures with active parameter tracking for both user-defined and built-in methods (including runtime data types from the AL method library). |
| **Document Symbols** | Nested hierarchical view (objects > procedures/triggers > parameters/variables, plus page `usercontrol` members and `controladdin` procedures/events). |
| **Workspace Symbol** | Search across all open documents. Case-insensitive substring matching. |
| **Rename** | Renames variables, parameters, procedures, and fields. Auto-quotes names with spaces. |
| **Document Highlight** | Highlights all references to the symbol under cursor within the same document. |
| **Document Formatting** | CST-based formatter with proper indentation, operator spacing, and blank line management. |
| **Folding Ranges** | Folds objects, procedures, triggers, var sections, control flow blocks, and block comments. |
| **Diagnostics** | Reports parser errors (`ERROR`/`MISSING`) plus semantic member diagnostics (unknown members, accessibility checks) with AL-aware fallbacks for common valid syntax patterns, including no-`()` zero-parameter calls, trigger return values, inline `Option` declarations, and advanced `TableRelation` expressions. |

## Project Structure

```
al-lsp/
├── crates/
│   ├── al-parser/       # Tree-sitter grammar for AL + Rust bindings
│   ├── al-syntax/       # Syntax analysis: AST, symbols, navigation, formatting, diagnostics
│   └── al-lsp/          # LSP server binary (tower-lsp)
├── editors/
│   └── vscode/          # VS Code extension client
└── test-fixtures/       # Sample .al files
```

### Crates

**al-parser** — Custom tree-sitter grammar supporting all 13 AL object types, procedures, triggers (including Boolean/named return values), control add-ins, page `usercontrol` sections, control flow statements (if/case/for/while/repeat/with), compound assignment operators (`+=`, `-=`, `*=`, `/=`), full AL expression grammar, `in` ranges (`A .. B`), `TableRelation` (including `IF/ELSE` and `WHERE(...)`), `CalcFormula`, `DecimalPlaces`, inline `Option` declarations, and no-`()` procedure invocations for zero-parameter calls. All keywords are case-insensitive.

**al-syntax** — Higher-level analysis layer built on the parser:
- `ast` — Symbol tree extraction from parse trees (objects, procedures, variables, fields, etc.)
- `symbols` — Case-insensitive symbol index with scoped lookup
- `navigation` — Go-to-definition resolution, reference finding, signature help context
- `formatting` — CST-based document formatter (indentation + spacing)
- `diagnostics` — Syntax error detection
- `document` — Per-file state management (rope text buffer, tree, symbol table)

**al-lsp** — The LSP server. Communicates over stdin/stdout. Holds a `WorldState` with concurrent indexes for open documents and workspace objects. Startup indexing is phased so open-file features are available immediately while cross-file workspace indexing warms in the background.

## Building

```bash
cargo build --release
```

The binary is `target/release/al-lsp`.

### Prerequisites

- Rust toolchain (edition 2021)
- Node.js (for the VS Code extension and tree-sitter grammar generation)

### Regenerating the Parser

If you modify `crates/al-parser/grammar.js`:

```bash
cd crates/al-parser
npx tree-sitter generate
cargo clean -p al-parser
cd ../..
cargo build
```

The `cargo clean -p al-parser` step is required to avoid stale build artifacts.

## Testing

```bash
cargo test
```

This runs tests across all three crates:
- **al-parser** — Grammar correctness tests (object types, procedures, triggers, statements, expressions, property grammars)
- **al-syntax** — Symbol extraction, navigation, reference finding, formatting, syntax diagnostics
- **al-lsp** — LSP handler integration tests (go-to-definition, go-to-implementation, references, hover, completion, signature help, diagnostics)

## VS Code Extension

### Development

```bash
cargo install --path crates/al-lsp
cd editors/vscode
npm install
npm run compile
```

Then open VS Code with the extension loaded:

```bash
code --extensionDevelopmentPath=$PWD/editors/vscode test-fixtures/
```

### Configuration

| Setting | Default | Description |
|---|---|---|
| `alLsp.serverPath` | `al-lsp` | Path to the `al-lsp` binary. Defaults to looking it up on `$PATH`. |

## Formatting Rules

The formatter enforces AL canonical style with 4-space indentation:

- `{`/`}` on their own lines at object level
- `begin`/`end` at procedure level, aligned with `procedure`/`trigger`
- `var` at the same level as `procedure`, variable declarations indented one level deeper
- No blank lines between `procedure` signature, `var`, and `begin`
- For/while/with loops: `begin` joins the `do` line, `end` aligns with the loop keyword, body indented two levels from the loop
- `repeat`/`until` at the same level, body indented one level
- Spacing enforced around `:=`, `+=`, `-=`, `*=`, `/=`, comparison operators, after commas, and before semicolons
- String literals and comments are preserved as-is

```al
codeunit 50100 "My Codeunit"
{
    procedure DoWork()
    var
        Counter: Integer;
    begin
        Counter := 0;
        for Counter := 1 to 5 do begin
                Counter += 1;
        end;
        repeat
            Counter -= 1;
        until Counter = 0;
    end;
}
```

## Supported AL Constructs

### Object Types

`codeunit`, `table`, `tableextension`, `page`, `pageextension`, `report`, `enum`, `enumextension`, `xmlport`, `query`, `interface`, `permissionset`, `controladdin`

### Statements

`if`/`then`/`else`, `case`/`of`/`end`, `for`/`to`/`downto`/`do`, `while`/`do`, `repeat`/`until`, `with`/`do`, `exit`, assignment (`:=`), compound assignment (`+=`, `-=`, `*=`, `/=`)

### Expressions

Logical (`or`, `xor`, `and`, `not`), comparison (`=`, `<>`, `<`, `>`, `<=`, `>=`, `in`), intervals (`..`), arithmetic (`+`, `-`, `*`, `/`, `mod`, `div`), method calls, member access, array access, qualified enum values (`Enum::Value` and `Rec."Enum Field"::Value`), qualified inline `Option` values (`OptionVar::Value`), string/integer/decimal/boolean/temporal literals

### Property Expressions

`TableRelation` (simple table target, table field target, `WHERE(...)`, nested `IF/ELSE`, `CONST`, `FILTER`, `FIELD`), `CalcFormula`, `DataItemLink`, `DataItemTableView` (`SORTING`, `ORDER`, `WHERE`), `DecimalPlaces`

### Type References

`simple_type`, `sized_type` (e.g. `Code[20]`), `Record` (with optional `temporary`), `Enum`, `Interface`, `List of [Type]`, `Dictionary of [K, V]`, `array [N] of Type`, `Option`, `Label`, `DotNet`, and runtime method-heavy types such as `Text`/`SecretText`/`TextBuilder`/`TextConst`, `Any`/`Boolean`/`Byte`/`Integer`/`Decimal`/`BigInteger`, `Guid`, `Date`/`DateFormula`/`Time`/`DateTime`/`Duration`, `InStream`/`OutStream`, `JsonObject`/`JsonArray`/`JsonToken`/`JsonValue`, `HttpClient` family, `XmlDocument` family (`XmlAttributeCollection`, `XmlCData`, `XmlComment`, `XmlDeclaration`, `XmlDocumentType`, `XmlNamespaceManager`, `XmlNameTable`, `XmlProcessingInstruction`, `XmlReadOptions`, `XmlText`, `XmlWriteOptions`), `RecordId`, `Variant`, `ErrorInfo`, `SessionSettings`, `Notification`, `Dialog`, `Session`/`SessionInformation`, `Database`, `System`, `TaskScheduler`, `FilterPageBuilder`, `Blob`, `File`/`FileUpload`, `Version`, `NavApp`, `ModuleInfo`/`ModuleDependencyInfo`, `Media`/`MediaSet`, `NumberSequence`, `CompanyProperty`/`ProductName`/`IsolatedStorage`/`Cookie`/`DataTransfer`/`Debugger`/`RequestPage`/`WebServiceActionContext`, `TestPage`/`TestField`/`TestAction`/`TestRequestPage`/`TestPart`/`TestFilter`/`TestFilterField`/`TestHttpRequestMessage`/`TestHttpResponseMessage`, and `RecordRef`/`FieldRef`/`KeyRef`

### Declaration Details

- Procedures and triggers support both unnamed and named return values.
- Zero-parameter procedure invocations without `()` are accepted where AL allows them.
- Inline `Option` declarations are supported for globals, locals, and parameters, including empty first members.
- Record variables resolve to record/table built-ins, fields, and accessible table procedures.

## License

MIT
