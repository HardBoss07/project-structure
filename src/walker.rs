use crate::config::Config;
use anyhow::Result;
use ignore::WalkBuilder;
use std::path::PathBuf;

pub fn walk(config: &Config) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    let walker = WalkBuilder::new(&config.root)
        .hidden(!config.include_hidden)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker {
        let entry = entry?;
        paths.push(entry.path().to_path_buf());
    }

    Ok(paths)
}
