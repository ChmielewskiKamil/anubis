use cairo_lang_parser::utils::SimpleParserDatabase;

use cairo_lang_syntax::node::kind::SyntaxKind::FunctionWithBody;
use cairo_lang_syntax::node::SyntaxNode;

use crate::analyser::ast::extract_target_from_node;

pub fn unused_parameter_qa(source_code: SyntaxNode, db: &SimpleParserDatabase) {
    let nodes = extract_target_from_node(FunctionWithBody, source_code, db);
    for node in &nodes {
        println!("Found a function with body: {:?}", node.kind(db));
    }
}
