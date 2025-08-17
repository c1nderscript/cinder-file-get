use finishes::{ignore, scan};
use std::fs;
use tempfile::tempdir;

#[test]
fn excludes_dependency_dirs() {
    let root = tempdir().unwrap();
    fs::create_dir(root.path().join("node_modules")).unwrap();
    fs::write(root.path().join("node_modules/file.rs"), "").unwrap();
    fs::create_dir(root.path().join("target")).unwrap();
    fs::write(root.path().join("target/file.rs"), "").unwrap();
    fs::create_dir(root.path().join("venv")).unwrap();
    fs::write(root.path().join("venv/file.rs"), "").unwrap();
    fs::write(
        root.path().join(".finishesignore"),
        "node_modules/\ntarget/\nvenv/\n",
    )
    .unwrap();
    let ig = ignore::build(root.path()).unwrap();
    let files = scan::scan(root.path(), &ig).unwrap();
    assert!(files.is_empty());
}
