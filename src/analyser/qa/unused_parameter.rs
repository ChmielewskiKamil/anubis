use std::collections::HashSet;
use std::path::PathBuf;

use cairo_lang_parser::db::ParserGroup;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind::{
    FunctionDeclaration, FunctionWithBody, ItemList, SyntaxFile, TokenIdentifier,
};
use cairo_lang_syntax::node::SyntaxNode;

use crate::analyser::ast::extract_target_from_node;

pub fn unused_parameter_qa(source_code: SyntaxNode, db: &SimpleParserDatabase) {
    let nodes = extract_target_from_node(FunctionWithBody, source_code, db);
    nodes.iter().for_each(|node| {
        println!("Found a function with body: {:?}", node.kind(db));
    });
}
