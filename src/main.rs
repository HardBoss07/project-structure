use anyhow::Result;
use clap::Parser;

mod cli;
mod config;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from(cli);

    println!("Project root: {}", config.root.display());
    Ok(())
}
