use crate::{config::Config, node::Node};
use anyhow::Result;
use ignore::{WalkBuilder, overrides::OverrideBuilder};

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
        builder.git_ignore(false).git_global(false).git_exclude(false);
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

        // skip the root itself
        if path == config.root {
            continue;
        }

        let is_dir = entry.file_type().map(|f| f.is_dir()).unwrap_or(false);

        nodes.push(Node {
            path: path.to_path_buf(),
            is_dir,
        });
    }

    Ok(nodes)
}
