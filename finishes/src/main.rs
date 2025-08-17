use finishes::{cli, logging};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();
    cli::run()
}
