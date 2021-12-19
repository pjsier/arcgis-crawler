use anyhow::{self, Result};

use crate::nodes::ServerNode;

#[derive(Debug, Default)]
struct NodeTree {
    tree: BTreeMap<String, NodeTree>,
}

impl NodeTree {
    pub fn insert(&mut self, path: Vec<String>) {
        let mut cur = self;
        for p in path.iter() {
            cur = cur.tree.entry(p).or_insert_with(NodeTree::default)
        }
    }

    pub fn _print(&self, top: bool, parent: Vec<String>) {
        let normal_prefix = format!("{}│   ", prefix);
        let last_prefix = format!("{}    ", prefix);

        for (idx, (path, it)) in self.tree.iter().enumerate() {
            let current_path = parent_path.join(path);
            let style = ansi_style_for_path(&lscolors, &current_path);

            let contains_singleton_dir = it.contains_singleton_dir();

            let painted = match full_path {
                false => style.paint(path.to_string_lossy()),
                true => match contains_singleton_dir && !join_with_parent {
                    false => style.paint(current_path.to_string_lossy()),
                    true => style.paint(""),
                },
            };

            // If this folder only contains a single dir, we skip printing it because it will be
            // picked up and printed on the next iteration. If this is a full path (even if it
            // contains more than one directory), we also want to skip printing, because the full
            // path will be printed all at once (see painted above), not part by part.
            // If this is a full path however the prefix must be printed at the very beginning.
            let should_print = (contains_singleton_dir && !join_with_parent)
                || !contains_singleton_dir
                || !full_path;

            let newline = if contains_singleton_dir { "" } else { "\n" };
            let is_last = idx == self.trie.len() - 1;

            if !is_last {
                if should_print {
                    print!("{}├── {}{}", prefix, painted, newline);
                }
                &normal_prefix
            } else {
                if should_print {
                    print!("{}└── {}{}", prefix, painted, newline);
                }
                &last_prefix
            };

            it._print(
                false,
                next_prefix,
                contains_singleton_dir,
                lscolors,
                current_path,
                full_path,
            )
        }
    }

    pub fn print(&self) {}
}

fn print(nodes: Vec<ServerNode>) -> Result<()> {
    let mut nodes = nodes;
    nodes.sort();

    let mut tree = NodeTree::default();
    for node in nodes {
        tree.insert(node.0);
    }

    // for (idx, node) in nodes.iter().enumerate() {

    // }
    Ok(())
}

// https://github.com/jez/as-tree/blob/0036c20f66795774eb9cda3ccbae6ca1e1c19444/src/main.rs#L42-L111
// fn _print(
//     &self,
//     top: bool,
//     prefix: &str,
//     join_with_parent: bool,
//     lscolors: &LsColors,
//     parent_path: PathBuf,
//     full_path: bool,
// ) {
//     let normal_prefix = format!("{}│   ", prefix);
//     let last_prefix = format!("{}    ", prefix);

//     for (idx, (path, it)) in self.trie.iter().enumerate() {
//         let current_path = parent_path.join(path);
//         let style = ansi_style_for_path(&lscolors, &current_path);

//         let contains_singleton_dir = it.contains_singleton_dir();

//         let painted = match full_path {
//             false => style.paint(path.to_string_lossy()),
//             true => match contains_singleton_dir && !join_with_parent {
//                 false => style.paint(current_path.to_string_lossy()),
//                 true => style.paint(""),
//             },
//         };

//         // If this folder only contains a single dir, we skip printing it because it will be
//         // picked up and printed on the next iteration. If this is a full path (even if it
//         // contains more than one directory), we also want to skip printing, because the full
//         // path will be printed all at once (see painted above), not part by part.
//         // If this is a full path however the prefix must be printed at the very beginning.
//         let should_print = (contains_singleton_dir && !join_with_parent)
//             || !contains_singleton_dir
//             || !full_path;

//         let newline = if contains_singleton_dir { "" } else { "\n" };
//         let is_last = idx == self.trie.len() - 1;

//         let next_prefix = if join_with_parent {
//             let joiner = if full_path || top || parent_path == PathBuf::from("/") {
//                 ""
//             } else {
//                 "/"
//             };
//             if should_print {
//                 print!("{}{}{}", style.paint(joiner), painted, newline);
//             }
//             prefix
//         } else if !is_last {
//             if should_print {
//                 print!("{}├── {}{}", prefix, painted, newline);
//             }
//             &normal_prefix
//         } else {
//             if should_print {
//                 print!("{}└── {}{}", prefix, painted, newline);
//             }
//             &last_prefix
//         };

//         it._print(
//             false,
//             next_prefix,
//             contains_singleton_dir,
//             lscolors,
//             current_path,
//             full_path,
//         )
//     }
// }
