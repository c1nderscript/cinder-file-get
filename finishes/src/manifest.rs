use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ManifestFile {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExportManifest {
    pub commit_sha: String,
    pub file_count: usize,
    pub total_bytes: u64,
    pub files: Vec<ManifestFile>,
}

pub fn write_manifest(
    dest: &Path,
    commit_sha: String,
    files: Vec<ManifestFile>,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_bytes: u64 = files.iter().map(|f| f.bytes).sum();
    let manifest = ExportManifest {
        commit_sha,
        file_count: files.len(),
        total_bytes,
        files,
    };
    let json = serde_json::to_string_pretty(&manifest)?;
    let path = dest.join("export.manifest.json");
    if dry_run {
        println!("[dry-run] write {}", path.display());
    } else {
        std::fs::write(path, json)?;
    }
    Ok(())
}

pub fn read_manifest(
    dest: &Path,
) -> Result<Option<ExportManifest>, Box<dyn std::error::Error>> {
    let path = dest.join("export.manifest.json");
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(path)?;
    let manifest = serde_json::from_str(&data)?;
    Ok(Some(manifest))
}
