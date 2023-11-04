
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_parser::db::ParserGroup;

use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase};
use cairo_lang_syntax::node::kind::SyntaxKind::{
    FunctionDeclaration, FunctionWithBody, TokenIdentifier,
};
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};

fn main() {
    let db = SimpleParserDatabase::default();
    let mut file_path = PathBuf::new();
    file_path.push("test_files/example1.cairo");
    let file_id = FileId::new(&db, file_path.clone());
    let ast = db.file_syntax(file_id).unwrap().descendants(&db);

    let (syntax_root, _) = get_syntax_root_and_diagnostics_from_file(&db, file_path);
    // False and False because we don't want colored output and trivia
    let ast_visual = print_tree(&db, &syntax_root, false, false);
    // Write the content to a markdown file
    let mut file = File::create("test_files/example_ast.md").unwrap();
    file.write_all(ast_visual.as_bytes()).unwrap();

    for node in ast {
        if node.kind(&db) == FunctionWithBody {
            // Traverse children of `FunctionWithBody` to find `FunctionDeclaration`
            for child in node.descendants(&db) {
                if child.kind(&db) == FunctionDeclaration {
                    // Now, find the `name` node within `FunctionDeclaration`
                    for name_child in child.descendants(&db) {
                        if name_child.kind(&db) == TokenIdentifier {
                            // This should be the function name
                            println!("Function name: {:?}", name_child.text(&db).unwrap());
                            break; // Break after finding the name
                        }
                    }
                    break; // Break after processing `FunctionDeclaration`
                }
            }
        }
    }
}
