use cairo_lang_parser::utils::SimpleParserDatabase;

use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use crate::analyser::ast::extract_target_from_node;

pub fn unused_parameter_qa(source_code: SyntaxNode, db: &SimpleParserDatabase) {
    // It will return a vector of SyntaxNodes of the ParamList kind
    // from every single function in a file.
    let params:Vec<SyntaxNode> = extract_target_from_node(SyntaxKind::ParamList, source_code, db);

    params.iter().for_each(|param_group| {
        println!("{:?}", param_group.clone().get_text_without_trivia(db));
    })
}
