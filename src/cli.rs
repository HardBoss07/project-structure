use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Clone)]
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

    /// Filter for just files or directories
    #[arg(long, value_enum)]
    pub filter: Option<Filter>,

    /// Output written to file instead of terminal
    #[arg(short, long, num_args(0..=1), require_equals(true))]
    pub output: Option<Option<PathBuf>>,

    /// Count lines or bytes for files and folders
    #[arg(long, num_args(0..=1), default_missing_value = "lines", value_enum)]
    pub count: Option<CountMode>,

    /// Compact display mode with optional ascii connectors
    #[arg(short, long, num_args(0..=1), default_missing_value = "compact", value_enum)]
    pub minimize: Option<MinimizeMode>,
}

#[derive(Clone, ValueEnum, PartialEq, Eq)]
pub enum Filter {
    All,
    Files,
    Dirs,
}

#[derive(Clone, ValueEnum, PartialEq, Eq)]
pub enum SortBy {
    Name,
    Type,
}

#[derive(Clone, ValueEnum, PartialEq, Eq, Copy)]
pub enum CountMode {
    Lines,
    Bytes,
}

#[derive(Clone, ValueEnum, PartialEq, Eq, Copy)]
pub enum MinimizeMode {
    Compact,
    Ascii,
}

impl Cli {
    pub fn root(&self) -> PathBuf {
        self.path.clone().unwrap_or_else(|| ".".into())
    }
}
