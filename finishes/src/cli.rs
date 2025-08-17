use clap::{Args, Parser, Subcommand};
use inquire::{MultiSelect, Text};
use std::{fs, path::Path, process::Command};

use crate::{config::Config, copy, ignore, manifest, scan};

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
    /// Sync files based on saved configuration
    Sync(SyncArgs),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init()?,
        Commands::Sync(args) => sync(args)?,
    }

    Ok(())
}

#[derive(Debug, Args)]
struct SyncArgs {
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    clean: bool,
    #[arg(long)]
    force: bool,
}

fn sync(args: SyncArgs) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("unable to determine config directory")?
        .join("finishes");
    let config_path = config_dir.join("config.json");
    let config: Config = serde_json::from_str(&fs::read_to_string(&config_path)?)?;

    if args.clean && config.destination.exists() {
        if args.dry_run {
            println!("[dry-run] remove {}", config.destination.display());
        } else {
            fs::remove_dir_all(&config.destination)?;
        }
    }

    let gitignore = ignore::build(&config.source_repo)?;
    let files = scan::scan(&config.source_repo, &gitignore)?;
    let manifest_files = copy::copy_and_hash(
        &files,
        &config.source_repo,
        &config.destination,
        args.dry_run,
        args.force,
    )?;
    let commit_sha = String::from_utf8(
        Command::new("git")
            .arg("-C")
            .arg(&config.source_repo)
            .arg("rev-parse")
            .arg("HEAD")
            .output()?
            .stdout,
    )?
    .trim()
    .to_string();
    manifest::write_manifest(
        &config.destination,
        commit_sha,
        manifest_files,
        args.dry_run,
    )?;
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
