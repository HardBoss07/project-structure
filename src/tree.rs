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
    let mut node_map: HashMap<PathBuf, &Node> = nodes.iter().map(|n| (n.path.clone(), n)).collect();
    let mut children_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

    // 1. Map parent -> child paths
    for node in nodes {
        if let Some(parent) = node.path.parent() {
            children_map
                .entry(parent.to_path_buf())
                .or_default()
                .push(node.path.clone());
        }
    }

    // 2. Recursive builder
    fn build_node(
        path: PathBuf,
        node_map: &HashMap<PathBuf, &Node>,
        children_map: &HashMap<PathBuf, Vec<PathBuf>>,
    ) -> TreeNode {
        let node_data = node_map.get(&path);

        let is_dir = node_data.map(|n| n.is_dir).unwrap_or(true);
        let mut children = Vec::new();

        if is_dir {
            if let Some(child_paths) = children_map.get(&path) {
                for cp in child_paths {
                    children.push(build_node(cp.clone(), node_map, children_map));
                }
            }
        }

        // Sort for consistent output: Dirs first, then name
        children.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.path.cmp(&b.path)));

        TreeNode {
            path,
            is_dir,
            children,
        }
    }

    build_node(config.root.clone(), &node_map, &children_map)
}
