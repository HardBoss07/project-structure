use clap::{Parser, ValueEnum};
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

    /// Space-seperated exclude patterns (gitignore-style)
    #[arg(long)]
    pub exclude: Option<String>,

    /// Filter Depth
    #[arg(long)]
    pub depth: Option<usize>,

    /// Sort Results flag
    #[arg(long, value_enum, default_value = "name")]
    pub sort: SortBy,
}

#[derive(Clone, ValueEnum)]
pub enum SortBy {
    Name,
    Type,
}

impl Cli {
    pub fn root(&self) -> PathBuf {
        self.path.clone().unwrap_or_else(|| ".".into())
    }
}
