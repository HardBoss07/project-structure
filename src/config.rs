use crate::cli::Cli;
use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
    pub include_hidden: bool,
    pub use_gitignore: bool,
    pub exclude_patterns: Vec<String>,
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
        }
    }
}
