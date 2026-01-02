use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "project-structure")]
struct Cli {
    /// Root path of the project
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.path.unwrap_or_else(|| ".".into());

    println!("Project root: {}", root.display());
    Ok(())
}
