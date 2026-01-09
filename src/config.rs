use crate::{cli::{Cli, SortBy}, node::Node};
use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
    pub include_hidden: bool,
    pub use_gitignore: bool,
    pub exclude_patterns: Vec<String>,
    pub depth: Option<usize>,
    pub sort: SortBy,
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        let patterns: Vec<String> = cli
            .exclude
            .clone()
            .unwrap_or_default()
            .split_whitespace()
            .map(String::from)
            .collect();

        Self {
            root: cli.root(),
            include_hidden: cli.include_hidden,
            use_gitignore: !cli.no_git,
            exclude_patterns: patterns,
            depth: cli.depth,
            sort: cli.sort,
        }
    }
}


impl Config {
    pub fn sort_nodes(&self, nodes: &mut Vec<Node>) {
        match self.sort {
            SortBy::Name => nodes.sort_by_key(|n| n.path.clone()),
            SortBy::Type => nodes.sort_by_key(|n| !n.is_dir),
        }
    }
}