use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod walker;
mod node;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from(cli);

    let nodes = walker::walk(&config)?;
    println!(
        "Project root: {} ({} entries)",
        config.root.display(),
        nodes.len()
    );

    Ok(())
}
