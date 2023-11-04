use cairo_lang_parser::db::ParserGroup;
use cairo_lang_parser::parser::Parser;
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase};
use std::path::PathBuf;

fn main() {
    let db = SimpleParserDatabase::default();
    let mut file_path = PathBuf::new();
    file_path.push("test_files/hello_world.cairo");
    let (syntax_root, diagnostics) = get_syntax_root_and_diagnostics_from_file(&db, file_path);
    let result = print_tree(&db, &syntax_root, false, false);
    println!("{}", result);
}
