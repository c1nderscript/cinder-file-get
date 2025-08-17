use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub fn build(root: &Path) -> Result<Gitignore, Box<dyn std::error::Error>> {
    let mut builder = GitignoreBuilder::new(root);
    builder.add(root.join(".gitignore"));
    builder.add(root.join(".finishesignore"));
    Ok(builder.build()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn unions_git_and_finishes_patterns() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join(".gitignore"), "foo.rs\n").unwrap();
        std::fs::write(root.path().join(".finishesignore"), "bar.rs\n").unwrap();
        let ig = build(root.path()).unwrap();
        assert!(ig
            .matched_path_or_any_parents(root.path().join("foo.rs"), false)
            .is_ignore());
        assert!(ig
            .matched_path_or_any_parents(root.path().join("bar.rs"), false)
            .is_ignore());
    }
}
