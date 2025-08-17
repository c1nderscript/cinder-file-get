use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Cli::parse();
    Ok(())
}
