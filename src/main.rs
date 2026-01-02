use anyhow::Result;
use clap::Parser;

mod cli;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.root();

    println!("Project root: {}", root.display());
    Ok(())
}
