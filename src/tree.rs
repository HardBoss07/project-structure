use crate::{config::Config, node::Node};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TreeNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
}

pub fn build(nodes: &[Node], config: &Config) -> TreeNode {
    let mut map: HashMap<PathBuf, TreeNode> = HashMap::new();

    for node in nodes {
        map.insert(
            node.path.clone(),
            TreeNode {
                path: node.path.clone(),
                is_dir: node.is_dir,
                children: Vec::new(),
            },
        );
    }

    let mut root = TreeNode {
        path: config.root.clone(),
        is_dir: true,
        children: Vec::new(),
    };

    for node in nodes {
        if let Some(parent) = node.path.parent() {
            if parent == config.root {
                if let Some(child) = map.remove(&node.path) {
                    root.children.push(child);
                }
            }
        }
    }

    root
}