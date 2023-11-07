use std::collections::HashSet;

use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::{db::SyntaxGroup, kind::SyntaxKind, SyntaxNode};

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
    println!("No match: {:?}", node.kind(db));
    match node.kind(db) {
        SyntaxKind::SyntaxFile => {
            println!("entered syntax file arm");
            db.get_children(node).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child.clone(), db))
            });
        }
        SyntaxKind::ItemList => {
            println!("entered item list arm");
            db.get_children(node).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child.clone(), db))
            });
        }
        SyntaxKind::FunctionWithBody => {
            println!("entered function with body arm");
            db.get_children(node).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child.clone(), db))
            });
        }

        SyntaxKind::FunctionDeclaration => {
            println!("entered function declaration");
            db.get_children(node).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child.clone(), db))
            });
        }

        SyntaxKind::FunctionSignature => {
            println!("entered function signature");
            db.get_children(node).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child.clone(), db))
            });
        }

        SyntaxKind::ParamList => {
            println!("entered param list");
            db.get_children(node).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child.clone(), db))
            });
        }

        _ => {}
    }
    matches
}
