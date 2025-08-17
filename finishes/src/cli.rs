use clap::{Parser, Subcommand};
use inquire::{MultiSelect, Text};
use std::{fs, path::Path};

use crate::config::Config;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize finishes configuration
    Init,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init()?,
    }

    Ok(())
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Text::new("Source repository path:").prompt()?;
    let dest = Text::new("Destination directory:").prompt()?;
    let options = vec!["rs", "md", "toml"];
    let file_types = MultiSelect::new("Select file types", options).prompt()?;

    let config = Config {
        source_repo: repo.into(),
        destination: dest.into(),
        file_types: file_types.into_iter().map(|s| s.to_string()).collect(),
    };

    let config_dir = dirs::config_dir()
        .ok_or("unable to determine config directory")?
        .join("finishes");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.json");
    fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;

    let ignore_path = Path::new(&config.source_repo).join(".finishesignore");
    if !ignore_path.exists() {
        fs::write(&ignore_path, "# Patterns to ignore\n")?;
    }

    Ok(())
}
