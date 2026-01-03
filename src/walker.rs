use crate::config::Config;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn walk(config: &Config) -> Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    visit_dir(&config.root, &mut entries)?;
    Ok(entries)
}

fn visit_dir(path: &PathBuf, entries: &mut Vec<PathBuf>) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            entries.push(entry_path.clone());
            visit_dir(&entry_path, entries)?;
        }
    }
    Ok(())
}
