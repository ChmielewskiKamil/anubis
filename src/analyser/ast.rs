use std::collections::HashSet;

use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::{db::SyntaxGroup, kind::SyntaxKind, SyntaxNode};

/// Will return a vector of all the nodes in the tree that match the target. For example, when
/// looking for a function declaration, it will return a vector of all the function declarations
/// starting the search at the `node` that was passed in.
///
/// # Arguments
/// * `target` - The `SyntaxKind` of the node you want to find for example
/// `SyntaxKind::FunctionWithBody`. You can find all the possible target `SyntaxKind`s in the
/// [`kind.rs#SyntaxKind`](https://github.com/starkware-libs/cairo/blob/95a4c5309461886abe375a81cecb93346ed91fa6/crates/cairo-lang-syntax/src/node/kind.rs) enum.
/// * `node` - The `SyntaxNode` you want to search through. This can be either the root node - the
/// `SyntaxFile` that is returned from the `SimpleParserDatabase::file_syntax()` method, or any other node that you got from previous extraction.
/// * `db` - The `SimpleParserDatabase` that you got from the `SimpleParserDatabase::default()` method. It is the Salsa database and is used in the official Cairo Language Parser.
pub fn extract_target_from_node(
    target: SyntaxKind,
    node: &SyntaxNode,
    db: &SimpleParserDatabase,
) -> Vec<SyntaxNode> {
    let mut target_set = HashSet::new();
    target_set.insert(target);

    walk_node_for_targets(&target_set, &node, db)
}

pub fn walk_node_for_targets(
    targets: &HashSet<SyntaxKind>,
    node: &SyntaxNode,
    db: &SimpleParserDatabase,
) -> Vec<SyntaxNode> {
    let mut matches = vec![];

    if targets.contains(&node.kind(db)) {
        println!("Found a match: {:?}", node.kind(db));
        matches.push(node.clone());
    }

    // The db.get_children() returns only the direct children of the node.
    // We don't want to traverse all the way down the tree with the node.descendants() method, 
    // as this results in duplicate matches.
    match node.kind(db) {
        SyntaxKind::SyntaxFile => {
            println!("entered syntax file arm");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }
        SyntaxKind::ItemList => {
            println!("entered item list arm");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        SyntaxKind::FunctionWithBody => {
            println!("entered function with body arm");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        SyntaxKind::FunctionDeclaration => {
            println!("entered function declaration");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        SyntaxKind::FunctionSignature => {
            println!("entered function signature");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        SyntaxKind::ParamList => {
            println!("entered param list");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        SyntaxKind::Param => {
            println!("entered param");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        // e.g. function parameter modifier like (ref b: bool) -> ref is the modifier
        SyntaxKind::ModifierList => {
            println!("entered modifier list");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        // e.g. function name
        SyntaxKind::TerminalIdentifier => {
            println!("entered terminal identifier");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        // e.g. function parameter type
        SyntaxKind::TypeClause => {
            println!("entered type clause");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        // e.g. function parameter name
        SyntaxKind::ExprPath => {
            println!("entered expr path");
            db.get_children(node.clone()).iter().for_each(|child| {
                println!("{:?}", child.kind(db));
                matches.append(&mut walk_node_for_targets(targets, child, db))
            });
        }

        _ => {}
    }
    matches
}
