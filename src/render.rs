use crate::tree::TreeNode;

pub fn ascii(root: &TreeNode) -> String {
    let mut lines = Vec::new();
    // We start with an empty prefix for the root
    render_node(root, "", true, &mut lines, true);
    lines.join("\n")
}

fn render_node(node: &TreeNode, prefix: &str, is_last: bool, lines: &mut Vec<String>, is_root: bool) {
    let name = node.path.file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| node.path.to_string_lossy());

    if is_root {
        // Just print the root name (e.g., ".")
        lines.push(name.to_string());
    } else {
        let connector = if is_last { "└── " } else { "├── " };
        lines.push(format!("{}{}{}", prefix, connector, name));
    }

    // CRITICAL FIX: Determine the prefix for the NEXT level
    let next_prefix = if is_root {
        "".to_string() 
    } else {
        format!("{}{}", prefix, if is_last { "    " } else { "│   " })
    };

    let count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        render_node(child, &next_prefix, i + 1 == count, lines, false);
    }
}