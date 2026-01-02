use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "project-structure")]
pub struct Cli {
    /// Root path of the project
    path: Option<PathBuf>,
}

impl Cli {
    pub fn root(&self) -> PathBuf {
        self.path.clone().unwrap_or_else(|| ".".into())
    }
}
