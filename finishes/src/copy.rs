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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn hashes_manifest_entries() {
        let root = tempdir().unwrap();
        let src = root.path().join("file.md");
        std::fs::write(&src, b"hello").unwrap();
        let dest = tempdir().unwrap();
        let files = copy_and_hash(&[src.clone()], root.path(), dest.path(), true, true).unwrap();
        assert_eq!(files.len(), 1);
        let mut hasher = Sha256::new();
        hasher.update(b"hello");
        let expected = hex::encode(hasher.finalize());
        assert_eq!(files[0].sha256, expected);
    }
}
