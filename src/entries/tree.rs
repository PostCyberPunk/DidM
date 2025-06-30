use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use super::EntriesManager;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EntryKind {
    Copy,
    Link,
    Skip,
    // Error(String),
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
pub struct TreeManager {
    root: TreeNode,
}
impl TreeManager {
    pub fn new() -> Self {
        Self {
            root: TreeNode::new(),
        }
    }
    pub fn print(self) {
        Self::print_tree(&self.root, "", true, "");
    }
    pub fn insert_path(&mut self, path: &Path, kind: EntryKind) {
        let mut node = &mut self.root;
        for comp in path.iter() {
            let comp_str = comp.to_string_lossy().to_string();
            node = node.children.entry(comp_str).or_insert_with(TreeNode::new);
        }
        node.kinds.insert(kind);
    }
    fn print_tree(node: &TreeNode, prefix: &str, last: bool, name: &str) {
        //fold path
        let mut curr_name = name.to_string();
        let mut curr_node = node;
        while curr_name.starts_with("/")
            && curr_node.children.len() == 1
            && curr_node.kinds.is_empty()
        {
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
                    // EntryKind::Error(e) => label.push_str("[E]"),
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
            Self::print_tree(child, &new_prefix, is_last, child_name);
        }
    }
}
