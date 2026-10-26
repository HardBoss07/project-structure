use crate::{config::Config, node::Node};
use anyhow::Result;
use ignore::{WalkBuilder, overrides::OverrideBuilder};
use std::fs::File;
use std::io::Read;

pub fn walk(config: &Config) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();

    let mut builder = WalkBuilder::new(&config.root);
    builder.hidden(!config.include_hidden);

    if let Some(depth) = config.depth {
        builder.max_depth(Some(depth));
    }

    if config.use_gitignore {
        builder.git_ignore(true).git_global(true).git_exclude(true);
    } else {
        builder
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false);
    }

    if !config.exclude_patterns.is_empty() {
        let mut overrides = OverrideBuilder::new(&config.root);
        for pattern in &config.exclude_patterns {
            overrides.add(&format!("!{}", pattern))?;
        }
        builder.overrides(overrides.build()?);
    }

    let walker = builder.build();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if path == config.root {
            continue;
        }

        let is_dir = entry.file_type().map(|f| f.is_dir()).unwrap_or(false);

        let mut line_count = 0;
        let mut byte_count = 0;
        let mut is_binary_or_image = false;

        if !is_dir {
            if let Ok(metadata) = std::fs::metadata(path) {
                byte_count = metadata.len();
            }

            if is_image_extension(path) {
                is_binary_or_image = true;
            } else if let Ok(mut file) = File::open(path) {
                let mut buffer = [0u8; 1024];
                let n = file.read(&mut buffer).unwrap_or(0);
                if buffer[..n].contains(&0) {
                    is_binary_or_image = true;
                } else if config.count.is_some() {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        line_count = count_lines(&content);
                    } else {
                        is_binary_or_image = true;
                    }
                }
            }
        }

        nodes.push(Node {
            path: path.to_path_buf(),
            is_dir,
            is_binary_or_image,
            line_count,
            byte_count,
        });
    }

    Ok(nodes)
}

fn count_lines(content: &str) -> usize {
    if content.is_empty() {
        return 0;
    }
    let mut lines = 0;
    for byte in content.bytes() {
        if byte == b'\n' {
            lines += 1;
        }
    }
    if !content.ends_with('\n') {
        lines += 1;
    }
    lines
}

fn is_image_extension(path: &std::path::Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "ico" | "svg" | "tiff" | "avif"
    )
}
