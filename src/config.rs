use crate::cli::Cli;
use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
    pub include_hidden: bool,
    pub use_gitignore: bool,
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        Self {
            root: cli.root(),
            include_hidden: cli.include_hidden,
            use_gitignore: !cli.no_git,
        }
    }
}
