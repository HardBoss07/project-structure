use crate::{cli::CountMode, config::Config, node::Node};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TreeNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_binary_or_image: bool,
    pub line_count: usize,
    pub byte_count: u64,
    pub children: Vec<TreeNode>,
}

pub fn build(nodes: &[Node], config: &Config) -> TreeNode {
    let node_map: HashMap<PathBuf, &Node> = nodes.iter().map(|n| (n.path.clone(), n)).collect();
    let mut children_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

    for node in nodes {
        if let Some(parent) = node.path.parent() {
            children_map
                .entry(parent.to_path_buf())
                .or_default()
                .push(node.path.clone());
        }
    }

    fn build_node(
        path: PathBuf,
        node_map: &HashMap<PathBuf, &Node>,
        children_map: &HashMap<PathBuf, Vec<PathBuf>>,
        config: &Config,
    ) -> TreeNode {
        let node_data = node_map.get(&path);

        let is_dir = node_data.map(|n| n.is_dir).unwrap_or(true);
        let mut children = Vec::new();

        if is_dir {
            if let Some(child_paths) = children_map.get(&path) {
                for cp in child_paths {
                    children.push(build_node(cp.clone(), node_map, children_map, config));
                }
            }
        }

        children.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.path.cmp(&b.path)));

        let mut line_count = node_data.map(|n| n.line_count).unwrap_or(0);
        let mut byte_count = node_data.map(|n| n.byte_count).unwrap_or(0);
        let is_binary_or_image = node_data.map(|n| n.is_binary_or_image).unwrap_or(false);

        if is_dir {
            line_count = 0;
            byte_count = 0;
            for child in &children {
                if config.count == Some(CountMode::Lines) {
                    if !child.is_binary_or_image {
                        line_count += child.line_count;
                    }
                } else {
                    byte_count += child.byte_count;
                }
            }
        }

        TreeNode {
            path,
            is_dir,
            is_binary_or_image,
            line_count,
            byte_count,
            children,
        }
    }

    build_node(config.root.clone(), &node_map, &children_map, config)
}
