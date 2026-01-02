use crate::cli::Cli;
use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        Self {
            root: cli.root(),
        }
    }
}
