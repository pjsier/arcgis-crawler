use anyhow::{self, Result};
use itertools::EitherOrBoth::{Both, Right};
use itertools::Itertools;
use ptree::{print_tree, TreeBuilder};

use crate::nodes::ServerNode;

pub fn print_node_tree(url: String, nodes: Vec<ServerNode>) -> Result<()> {
    let mut nodes = nodes;
    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut tree_builder = TreeBuilder::new(url);
    let mut nodes_iter = nodes.clone().into_iter().peekable();

    // Set up first node without a pair
    if let Some(node) = nodes.clone().first() {
        let mut node_iter = node.clone().0.into_iter().peekable();
        while let Some(node_value) = node_iter.next() {
            let is_last = node_iter.peek().is_none();
            if is_last {
                tree_builder.add_empty_child(node_value);
            } else {
                tree_builder.begin_child(node_value);
            }
        }
    }

    while let Some(node) = nodes_iter.next() {
        let default_node = ServerNode(vec![]);
        let peek_node = &*nodes_iter.peek().unwrap_or(&default_node);
        let node_len = node.0.len();
        let mut pair_iter = node
            .0
            .into_iter()
            .zip_longest(peek_node.clone().0.into_iter())
            .enumerate()
            .peekable();
        let mut is_same = true;
        while let Some(pair) = pair_iter.next() {
            let is_last = pair_iter.peek().is_none();
            match pair {
                (idx, Both(left, right)) => {
                    if is_same && !is_last && left != right {
                        for _ in idx..(node_len - 1) {
                            tree_builder.end_child();
                        }
                        is_same = false;
                    }
                    if is_last {
                        tree_builder.add_empty_child(right);
                    } else if !is_same || left != right {
                        tree_builder.begin_child(right);
                    }
                }
                (_, Right(right)) => {
                    if is_last {
                        tree_builder.add_empty_child(right);
                    } else {
                        tree_builder.begin_child(right);
                    }
                }
                _ => {}
            }
        }
    }

    let tree = tree_builder.build();
    print_tree(&tree)?;

    Ok(())
}
