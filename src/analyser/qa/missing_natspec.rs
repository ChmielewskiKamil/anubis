use std::collections::HashSet;
use std::path::PathBuf;

use cairo_lang_filesystem::ids::FileId;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind::{
    FunctionDeclaration, FunctionWithBody, TokenIdentifier,
};
use cairo_lang_syntax::node::SyntaxNode;

pub fn missing_natspec_qa(source_code: SyntaxNode, db: SimpleParserDatabase, file_id: FileId) {
    let ast = source_code.descendants(&db);

    for node in ast {
        if node.kind(&db) == FunctionWithBody {
            // Traverse children of `FunctionWithBody` to find `FunctionDeclaration`
            for child in node.descendants(&db) {
                if child.kind(&db) == FunctionDeclaration {
                    // Now, find the `name` node within `FunctionDeclaration`
                    for name_child in child.descendants(&db) {
                        if name_child.kind(&db) == TokenIdentifier {
                            let position =
                                name_child.offset().position_in_file(&db, file_id).unwrap();
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn it_should_find_missing_returns_natspec() {
    let db = SimpleParserDatabase::default();
    let mut file_path = PathBuf::new();
    file_path.push("test_files/qa_001_missing_natspec.cairo");
    let file_id = FileId::new(&db, file_path);
    let ast = db.file_syntax(file_id).unwrap().descendants(&db);
}
