use ignore::gitignore::Gitignore;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MAX_BYTES: u64 = 25 * 1024 * 1024;

pub fn scan(
    root: &Path,
    gitignore: &Gitignore,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let allowed = ["md", "mdx", "markdown", "go", "rs", "py"];
    let mut files = Vec::new();
    for entry in WalkDir::new(root) {
        let entry = entry?;
        let path = entry.path();
        let is_dir = entry.file_type().is_dir();
        if gitignore
            .matched_path_or_any_parents(path, is_dir)
            .is_ignore()
        {
            if is_dir {
                continue;
            }
            continue;
        }
        if is_dir {
            continue;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some(e) if allowed.contains(&e) => {}
            _ => continue,
        }
        let meta = entry.metadata()?;
        if meta.len() > MAX_BYTES {
            continue;
        }
        if meta.file_type().is_symlink() {
            let target = std::fs::canonicalize(path)?;
            if !target.starts_with(root) {
                continue;
            }
        }
        if is_binary(path)? {
            continue;
        }
        files.push(path.to_path_buf());
    }
    Ok(files)
}

fn is_binary(path: &Path) -> Result<bool, std::io::Error> {
    use std::io::Read;
    let mut f = std::fs::File::open(path)?;
    let mut buf = [0u8; 8000];
    let n = f.read(&mut buf)?;
    Ok(buf[..n].contains(&0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ignore::gitignore::GitignoreBuilder;
    use tempfile::tempdir;

    #[test]
    fn symlink_outside_root_is_ignored() {
        let root = tempdir().unwrap();
        let external = tempdir().unwrap();
        let external_file = external.path().join("ext.md");
        std::fs::write(&external_file, "hi").unwrap();
        std::os::unix::fs::symlink(&external_file, root.path().join("link.md")).unwrap();
        let ignore = GitignoreBuilder::new(root.path()).build().unwrap();
        let files = scan(root.path(), &ignore).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn filters_by_extension() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("keep.rs"), "").unwrap();
        std::fs::write(root.path().join("skip.txt"), "").unwrap();
        let ignore = GitignoreBuilder::new(root.path()).build().unwrap();
        let files = scan(root.path(), &ignore).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("keep.rs"));
    }
}
