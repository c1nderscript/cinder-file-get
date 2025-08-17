use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub source_repo: PathBuf,
    pub destination: PathBuf,
    pub file_types: Vec<String>,
}
