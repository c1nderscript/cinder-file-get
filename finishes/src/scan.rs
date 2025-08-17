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
