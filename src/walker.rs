use crate::{config::Config, node::Node};
use anyhow::Result;
use std::path::PathBuf;
use ignore::{WalkBuilder, overrides::{OverrideBuilder}};

pub fn walk(config: &Config) -> Result<Vec<PathBuf>> {
    let mut nodes = Vec::new();

    let mut builder = WalkBuilder::new(&config.root);
    builder.hidden(!config.include_hidden);

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
        nodes.push(entry.path().to_path_buf());
    }

    Ok(nodes)
}
