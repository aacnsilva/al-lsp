fn main() {
    let src_dir = std::path::Path::new("src");

    let parser_c = src_dir.join("parser.c");
    if !parser_c.exists() {
        panic!(
            "parser.c not found at {:?}. Run `tree-sitter generate` in crates/al-parser/ first.",
            parser_c
        );
    }

    // Ensure cargo rebuilds when the generated parser changes
    println!("cargo:rerun-if-changed=src/parser.c");
    println!("cargo:rerun-if-changed=grammar.js");

    let mut build = cc::Build::new();
    build
        .include(src_dir)
        .file(&parser_c)
        .warnings(false);

    // tree-sitter 0.24 ABI
    build.define("TREE_SITTER_HIDE_SYMBOLS", None);
    build.compile("tree_sitter_al");
}
