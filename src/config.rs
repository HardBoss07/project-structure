use crate::cli::Cli;
use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
    pub include_hidden: bool,
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        Self {
            root: cli.root(),
            include_hidden: false, // default behavior
        }
    }
}
