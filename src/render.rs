use crate::{
    cli::{CountMode, Filter, MinimizeMode},
    config::Config,
    tree::TreeNode,
};

pub fn ascii(root: &TreeNode, config: &Config) -> String {
    let mut raw_lines = Vec::new();
    let filter = &config.filter;

    let max_width = if config.count.is_some() && config.minimize.is_none() {
        compute_max_width(root, "", true, filter, config.minimize, true)
    } else {
        0
    };

    render_node(root, "", true, &mut raw_lines, true, config, max_width);
    raw_lines.join("\n")
}

struct RenderConnectors {
    branch: &'static str,
    last_branch: &'static str,
    vertical: &'static str,
    indent_space: &'static str,
}

fn get_connectors(mode: Option<MinimizeMode>) -> RenderConnectors {
    match mode {
        Some(MinimizeMode::Ascii) => RenderConnectors {
            branch: "+-- ",
            last_branch: "\\-- ",
            vertical: "|   ",
            indent_space: "    ",
        },
        Some(MinimizeMode::Compact) => RenderConnectors {
            branch: "├ ",
            last_branch: "└ ",
            vertical: "│ ",
            indent_space: "  ",
        },
        None => RenderConnectors {
            branch: "├── ",
            last_branch: "└── ",
            vertical: "│   ",
            indent_space: "    ",
        },
    }
}

fn format_count(node: &TreeNode, mode: CountMode) -> String {
    match mode {
        CountMode::Lines => {
            if node.is_binary_or_image {
                format!("({})", format_bytes(node.byte_count))
            } else {
                let label = if node.line_count == 1 {
                    "line"
                } else {
                    "lines"
                };
                format!("({} {})", node.line_count, label)
            }
        }
        CountMode::Bytes => format!("({})", format_bytes(node.byte_count)),
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes == 1 {
        return "1 byte".to_string();
    }
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b < KB {
        format!("{} bytes", bytes)
    } else if b < MB {
        format!("{:.3} KB", b / KB)
    } else if b < GB {
        format!("{:.3} MB", b / MB)
    } else {
        format!("{:.3} GB", b / GB)
    }
}

fn compute_max_width(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    filter: &Filter,
    minimize: Option<MinimizeMode>,
    is_root: bool,
) -> usize {
    match filter {
        Filter::Files if node.is_dir => return 0,
        Filter::Dirs if !node.is_dir => return 0,
        _ => {}
    }

    let connectors = get_connectors(minimize);
    let mut name = node
        .path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| node.path.to_string_lossy().to_string());

    if node.is_dir {
        name.push('/');
    }

    let line_len = if is_root {
        name.chars().count()
    } else {
        let connector = if is_last {
            connectors.last_branch
        } else {
            connectors.branch
        };
        prefix.chars().count() + connector.chars().count() + name.chars().count()
    };

    let next_prefix = if is_root {
        "".to_string()
    } else {
        format!(
            "{}{}",
            prefix,
            if is_last {
                connectors.indent_space
            } else {
                connectors.vertical
            }
        )
    };

    let mut max = line_len;

    let count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let child_max =
            compute_max_width(child, &next_prefix, i + 1 == count, filter, minimize, false);
        if child_max > max {
            max = child_max;
        }
    }

    max
}

fn render_node(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    lines: &mut Vec<String>,
    is_root: bool,
    config: &Config,
    max_width: usize,
) {
    match config.filter {
        Filter::Files if node.is_dir => return,
        Filter::Dirs if !node.is_dir => return,
        _ => {}
    }

    let connectors = get_connectors(config.minimize);

    let mut name = node
        .path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| node.path.to_string_lossy().to_string());

    if node.is_dir {
        name.push('/');
    }

    let node_str = if is_root {
        name.to_string()
    } else {
        let connector = if is_last {
            connectors.last_branch
        } else {
            connectors.branch
        };
        format!("{}{}{}", prefix, connector, name)
    };

    if let Some(count_mode) = config.count {
        let count_str = format_count(node, count_mode);
        if config.minimize.is_some() {
            lines.push(format!("{} {}", node_str, count_str));
        } else {
            let node_len = node_str.chars().count();
            let padding = if max_width >= node_len {
                max_width - node_len + 2
            } else {
                2
            };
            lines.push(format!("{}{}{}", node_str, " ".repeat(padding), count_str));
        }
    } else {
        lines.push(node_str);
    }

    let next_prefix = if is_root {
        "".to_string()
    } else {
        format!(
            "{}{}",
            prefix,
            if is_last {
                connectors.indent_space
            } else {
                connectors.vertical
            }
        )
    };

    let count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        render_node(
            child,
            &next_prefix,
            i + 1 == count,
            lines,
            false,
            config,
            max_width,
        );
    }
}
