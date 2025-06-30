use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use super::EntriesManager;

#[derive(Debug, Eq, PartialEq, Hash)]
enum EntryKind {
    Copy,
    Link,
    Skip,
    Error(String),
}

#[derive(Debug)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
    kinds: HashSet<EntryKind>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            children: BTreeMap::new(),
            kinds: HashSet::new(),
        }
    }
}

impl EntriesManager {
    pub fn show_entry_tree(&self) {
        let mut root = TreeNode::new();

        fn insert_path(root: &mut TreeNode, path: &Path, kind: EntryKind) {
            let mut node = root;
            for comp in path.components() {
                let comp_str = comp.as_os_str().to_string_lossy().to_string();
                node = node.children.entry(comp_str).or_insert_with(TreeNode::new);
            }
            node.kinds.insert(kind);
        }

        // Collect entries
        for entry in &self.copy_list.entries {
            insert_path(&mut root, &entry.target_path, EntryKind::Copy);
        }
        for entry in &self.link_list.entries {
            insert_path(&mut root, &entry.target_path, EntryKind::Link);
        }
        for entry in &self.skip_list {
            insert_path(&mut root, &entry.target_path, EntryKind::Skip);
        }
        for (entry, error) in &self.error_list {
            insert_path(
                &mut root,
                &entry.target_path,
                EntryKind::Error(error.clone()),
            );
        }

        fn print_tree(node: &TreeNode, prefix: &str, last: bool, name: &str) {
            if !name.is_empty() {
                let branch = if last { "└── " } else { "├── " };
                let label = label_list(&node.kinds);
                if !label.is_empty() {
                    println!("{}{}{} {}", prefix, branch, label, name);
                } else {
                    println!("{}{}{}", prefix, branch, name);
                }
            }
            let len = node.children.len();
            for (i, (child_name, child_node)) in node.children.iter().enumerate() {
                let is_last = i == len - 1;
                let mut path = child_name.clone();
                let mut curr_node = child_node;
                while curr_node.kinds.is_empty()
                    && curr_node.children.len() == 1
                    && curr_node.children.values().next().unwrap().kinds.is_empty()
                    && curr_node.children.values().next().unwrap().children.len() == 1
                {
                    let (next_name, next_node) = curr_node.children.iter().next().unwrap();
                    if path == "/" {
                        path = String::new();
                    }
                    path = format!("{}/{}", path, next_name);
                    curr_node = next_node;
                }
                if !curr_node.kinds.is_empty() && curr_node.children.is_empty() {
                    let sub_label = label_list(&curr_node.kinds);
                    if !sub_label.is_empty() {
                        let new_prefix = if name.is_empty() {
                            "".to_string()
                        } else {
                            format!("{}{}", prefix, if last { "    " } else { "│   " })
                        };
                        println!(
                            "{}{}{} {}",
                            new_prefix,
                            if is_last { "└── " } else { "├── " },
                            sub_label,
                            path
                        );
                    }
                } else {
                    let new_prefix = if name.is_empty() {
                        "".to_string()
                    } else {
                        format!("{}{}", prefix, if last { "    " } else { "│   " })
                    };
                    print_tree(curr_node, &new_prefix, is_last, &path);
                }
            }
        }

        // 标签辅助函数
        fn label_list(kinds: &HashSet<EntryKind>) -> String {
            let mut label = String::new();
            for kind in kinds {
                match kind {
                    EntryKind::Copy => label.push_str("[C]"),
                    EntryKind::Link => label.push_str("[L]"),
                    EntryKind::Skip => label.push_str("[S]"),
                    EntryKind::Error(_) => label.push_str("[E]"),
                }
            }
            label
        }
        // print!("{:#?}", &root);
        print_tree(&root, "", true, "");
    }
}
