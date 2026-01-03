use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "project-structure")]
pub struct Cli {
    /// Root path of the project
    path: Option<PathBuf>,

    /// Include hidden files and directories
    #[arg(long)]
    pub include_hidden: bool,

    /// Do not use .gitignore rules
    #[arg(long)]
    pub no_git: bool,
}

impl Cli {
    pub fn root(&self) -> PathBuf {
        self.path.clone().unwrap_or_else(|| ".".into())
    }
}
