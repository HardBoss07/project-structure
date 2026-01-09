use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod walker;
mod node;
mod tree;
mod render;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from(cli);

    let mut nodes = walker::walk(&config)?;
    config.sort_nodes(&mut nodes);

    let tree = tree::build(&nodes, &config);
    let output = render::ascii(&tree);

    println!("{}", output);

    Ok(())
}
