use std::collections::HashSet;

use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::{kind::SyntaxKind, SyntaxNode};

pub fn extract_target_from_node(
    target: SyntaxKind,
    node: SyntaxNode,
    db: &SimpleParserDatabase,
) -> Vec<SyntaxNode> {
    let mut target_set = HashSet::new();
    target_set.insert(target);

    walk_node_for_targets(&target_set, node, db)
}

pub fn walk_node_for_targets(
    targets: &HashSet<SyntaxKind>,
    node: SyntaxNode,
    db: &SimpleParserDatabase,
) -> Vec<SyntaxNode> {
    let mut matches = vec![];

    if targets.contains(&node.kind(db)) {
        println!("Found a match: {:?}", node.kind(db));
        matches.push(node.clone());
    }

    match node.kind(db) {
    SyntaxKind::SyntaxFile => {
        println!("entered syntax file arm");
        node.descendants(db)
            .skip(1)
            .for_each(|item| {
                matches.append(&mut walk_node_for_targets(targets, item, db))
            });
    }
    SyntaxKind::ItemList => {
        println!("entered item list arm");
        node.descendants(db).skip(1)
            .for_each(|item| {
                matches.append(&mut walk_node_for_targets(targets, item, db))
            });
    }
        SyntaxKind::FunctionWithBody => {
            println!("entered function with body arm");
            for item in node.descendants(db).skip(1) {
                matches.append(&mut walk_node_for_targets(targets, item, db))
            }
        }
        _ => {}
    }
    matches
}
