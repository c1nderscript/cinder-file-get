use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::manifest::ManifestFile;

pub fn copy_and_hash(
    files: &[PathBuf],
    root: &Path,
    dest: &Path,
    dry_run: bool,
    force: bool,
) -> Result<Vec<ManifestFile>, Box<dyn std::error::Error>> {
    let mut out = Vec::new();
    for src in files {
        let rel = src.strip_prefix(root)?;
        let target = dest.join(rel);
        if dry_run {
            println!("[dry-run] copy {} -> {}", src.display(), target.display());
        } else {
            if target.exists() && !force {
                continue;
            }
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(src, &target)?;
        }
        let mut file = fs::File::open(src)?;
        let mut hasher = Sha256::new();
        let bytes = io::copy(&mut file, &mut hasher)?;
        let sha = hex::encode(hasher.finalize());
        out.push(ManifestFile {
            path: rel.to_string_lossy().into_owned(),
            bytes,
            sha256: sha,
        });
    }
    Ok(out)
}
