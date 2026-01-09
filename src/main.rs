use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod walker;
mod node;
mod tree;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from(cli);

    let mut nodes = walker::walk(&config)?;
    config.sort_nodes(&mut nodes);

    let _tree = tree::build(&nodes, &config);

    println!(
        "Project root: {} ({} entries)",
        config.root.display(),
        nodes.len()
    );

    Ok(())
}
