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
            //fold path
            let mut curr_name = name.to_string();
            let mut curr_node = node;
            while curr_node.kinds.is_empty() && curr_node.children.len() == 1 {
                let (child_name, child_node) = curr_node.children.iter().next().unwrap();
                curr_name = if curr_name.is_empty() {
                    child_name.clone()
                } else {
                    if curr_name == "/" {
                        curr_name = String::new();
                    }
                    format!("{}/{}", curr_name, child_name)
                };
                curr_node = child_node;
            }

            //print child
            if !curr_name.is_empty() {
                let branch = if last { "└── " } else { "├── " };
                let mut label = String::new();
                for kind in &curr_node.kinds {
                    match kind {
                        EntryKind::Copy => label.push_str("[C]"),
                        EntryKind::Link => label.push_str("[L]"),
                        EntryKind::Skip => label.push_str("[S]"),
                        EntryKind::Error(e) => label.push_str("[E]"),
                    }
                }
                print!("{}{}{} {}", prefix, branch, label, curr_name);
                println!();
            }
            let len = curr_node.children.len();

            for (i, (child_name, child)) in curr_node.children.iter().enumerate() {
                let is_last = i == len - 1;
                let new_prefix = if curr_name.is_empty() {
                    "".to_string()
                } else {
                    format!("{}{}", prefix, if last { "    " } else { "│   " })
                };
                print_tree(child, &new_prefix, is_last, child_name);
            }
        }
        print_tree(&root, "", true, "");
    }
}
