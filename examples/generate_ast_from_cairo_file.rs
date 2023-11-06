use cairo_lang_parser::{
    printer::print_tree,
    utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase},
};
use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    let db = SimpleParserDatabase::default();
    let mut file_path = PathBuf::new();
    file_path.push("src/analyser/analyser_test_data/qa_002_unused_parameter.cairo");
    let (syntax_root, _) = get_syntax_root_and_diagnostics_from_file(&db, file_path);
    // False and True because we don't want colored output but want the trivia (e.g. comments)
    let ast_visual = print_tree(&db, &syntax_root, false, true);
    let mut file = File::create("examples/example_ast.md").unwrap();
    file.write_all(ast_visual.as_bytes()).unwrap();
}
