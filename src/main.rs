use cairo_lang_parser::parser::Parser;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_parser::printer::print_tree;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_diagnostics::DiagnosticsBuilder;
use cairo_lang_parser::utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase};
use std::path::PathBuf;

fn main() {
    let db = SimpleParserDatabase::default();
    let mut file_path = PathBuf::new();
    file_path.push("test_files/example0.cairo");
    let file_id = FileId::new(&db, file_path.clone());
    let ast = db.file_syntax(file_id).unwrap().descendants(&db);

    ast.for_each(|node| {
        println!("{:?}", node.text(&db));
        println!("{:?}", node.kind(&db));
    });
}
