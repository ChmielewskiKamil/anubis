use std::{env::args, fs::File, io::Write, path::PathBuf};
use cairo_lang_parser::{
    printer::print_tree,
    utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase},
};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let db = SimpleParserDatabase::default();
        let mut file_path = PathBuf::new();
        // 1st argument is occupied by the name of the executable
        file_path.push(args[2].clone());
        let (syntax_root, _) = get_syntax_root_and_diagnostics_from_file(&db, file_path);
        // False and False because we don't want colored output and trivia
        let ast_visual = print_tree(&db, &syntax_root, false, false);
        let mut file = File::create("examples/example_ast.md").unwrap();
        file.write_all(ast_visual.as_bytes()).unwrap();
    } else {
        eprintln!("Please provide a single argument: a path to a .cairo file")
    }
}
