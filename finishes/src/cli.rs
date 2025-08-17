use clap::{Args, Parser, Subcommand};
use inquire::{MultiSelect, Text};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

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
    /// Display or update saved configuration
    Config(ConfigArgs),
    /// Diagnose configuration and export state
    Doctor,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init()?,
        Commands::Sync(args) => sync(args)?,
        Commands::Config(args) => config_cmd(args)?,
        Commands::Doctor => doctor()?,
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

#[derive(Debug, Args)]
struct ConfigArgs {
    #[arg(long)]
    source: Option<PathBuf>,
    #[arg(long)]
    dest: Option<PathBuf>,
    #[arg(long, action = clap::ArgAction::Append)]
    include: Vec<String>,
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

fn config_cmd(args: ConfigArgs) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("unable to determine config directory")?
        .join("finishes");
    let config_path = config_dir.join("config.json");
    let mut config: Config = if config_path.exists() {
        serde_json::from_str(&fs::read_to_string(&config_path)?)?
    } else {
        Config {
            source_repo: PathBuf::new(),
            destination: PathBuf::new(),
            file_types: Vec::new(),
        }
    };

    let mut changed = false;
    if let Some(src) = args.source {
        config.source_repo = src;
        changed = true;
    }
    if let Some(dest) = args.dest {
        config.destination = dest;
        changed = true;
    }
    if !args.include.is_empty() {
        config.file_types = args.include;
        changed = true;
    }
    if changed {
        fs::create_dir_all(&config_dir)?;
        fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
    }
    println!("source: {}", config.source_repo.display());
    println!("dest: {}", config.destination.display());
    println!("includes: {}", config.file_types.join(", "));
    Ok(())
}

fn doctor() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("unable to determine config directory")?
        .join("finishes");
    let config_path = config_dir.join("config.json");
    let config: Config = serde_json::from_str(&fs::read_to_string(&config_path)?)?;

    println!("source: {}", config.source_repo.display());
    if !config.source_repo.exists() {
        println!("  [!] missing source path");
    }
    println!("dest: {}", config.destination.display());
    if !config.destination.exists() {
        println!("  [!] destination does not exist");
    }

    println!("ignore rules:");
    for name in [".gitignore", ".finishesignore"] {
        let path = config.source_repo.join(name);
        if path.exists() {
            for line in fs::read_to_string(&path)?.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                println!("  {name}: {line}");
            }
        }
    }

    let gitignore = ignore::build(&config.source_repo)?;
    let files = scan::scan(&config.source_repo, &gitignore)?;
    println!("candidate files: {}", files.len());
    let mut total_bytes = 0u64;
    for f in &files {
        total_bytes += fs::metadata(f)?.len();
    }
    println!("total bytes: {total_bytes}");

    let existing = manifest::read_manifest(&config.destination)?;
    let mut manifest_map: HashMap<String, String> = HashMap::new();
    if let Some(m) = existing {
        for file in m.files {
            manifest_map.insert(file.path, file.sha256);
        }
    }
    let mut new_files = 0;
    let mut changed_files = 0;
    for path in &files {
        let rel = path.strip_prefix(&config.source_repo)?;
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher)?;
        let sha = hex::encode(hasher.finalize());
        let key = rel.to_string_lossy().into_owned();
        match manifest_map.get(&key) {
            Some(existing_sha) if existing_sha == &sha => {}
            Some(_) => changed_files += 1,
            None => new_files += 1,
        }
    }
    if new_files > 0 || changed_files > 0 {
        println!("would copy {new_files} new and {changed_files} updated files");
    } else {
        println!("no changes detected");
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
