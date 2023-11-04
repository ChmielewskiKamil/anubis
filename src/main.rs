mod analyser;
use analyser::*;
use std::env::args;

use cairo_lang_filesystem::ids::FileId;
use cairo_lang_parser::db::ParserGroup;

use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind::{
    FunctionDeclaration, FunctionWithBody, TokenIdentifier,
};
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};

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
    let ast = db.file_syntax(file_id).unwrap().descendants(&db);
    
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
                            println!("Location: {:?}", name_child.offset().position_in_file(&db, file_id));
                            break; // Break after finding the name
                        }
                    }
                    break; // Break after processing `FunctionDeclaration`
                }
            }
        }
    }
}
