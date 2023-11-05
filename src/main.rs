mod analyser;
use analyser::qa::unused_parameter::unused_parameter_qa;

use std::env::args;

use cairo_lang_filesystem::ids::FileId;
use cairo_lang_parser::db::ParserGroup;

use cairo_lang_parser::utils::SimpleParserDatabase;


use std::{path::PathBuf};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        println!("TODO")
    } else {
        eprintln!("Please provide a single argument: a path to a directory with .cairo files")
    }

    let db = SimpleParserDatabase::default();
    let mut file_path = PathBuf::new();
    // 1st argument is occupied by the name of the executable
    file_path.push(args[2].clone());
    let file_id = FileId::new(&db, file_path);
    let ast = db.file_syntax(file_id).unwrap();

    unused_parameter_qa(ast, &db);
}
