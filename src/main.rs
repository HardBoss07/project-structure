use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod cli;
mod config;
mod node;
mod render;
mod tree;
mod walker;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from(cli.clone());

    let mut nodes = walker::walk(&config)?;
    config.sort_nodes(&mut nodes);

    let tree = tree::build(&nodes, &config);
    let output = render::ascii(&tree, &config.filter);

    let file_path: Option<PathBuf> = match &cli.output {
        Some(Some(path)) => Some(path.clone()),
        Some(None) => Some("Project Structure.md".into()),
        None => None,
    };

    if let Some(path) = file_path {
        let markdown = format!("# Project Structure\n\n```\n{}\n```", output);
        fs::write(&path, markdown)?;
        println!("Project structure written to {}", path.display());
    } else {
        println!("{}", output);
    }

    Ok(())
}
