use finishes::scan::scan;
use ignore::gitignore::GitignoreBuilder;
use proptest::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

const MAX_BYTES: u64 = 25 * 1024 * 1024;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(16))]
    #[test]
    fn no_escape_and_respects_size(size in 0u64..(2*MAX_BYTES), use_symlink in proptest::bool::ANY) {
        let root = tempdir().unwrap();
        let file_path = root.path().join("keep.md");
        let mut f = File::create(&file_path).unwrap();
        let chunk = vec![b'a'; 8192];
        let mut written = 0u64;
        while written < size {
            let to_write = std::cmp::min(chunk.len() as u64, size - written) as usize;
            f.write_all(&chunk[..to_write]).unwrap();
            written += to_write as u64;
        }

        let _external = if use_symlink {
            let external = tempdir().unwrap();
            let external_file = external.path().join("ext.md");
            fs::write(&external_file, "hi").unwrap();
            std::os::unix::fs::symlink(&external_file, root.path().join("link.md")).unwrap();
            Some(external)
        } else {
            None
        };

        let ig = GitignoreBuilder::new(root.path()).build().unwrap();
        let files = scan(root.path(), &ig).unwrap();
        if size <= MAX_BYTES {
            assert!(files.iter().any(|p| p.ends_with("keep.md")));
        } else {
            assert!(!files.iter().any(|p| p.ends_with("keep.md")));
        }
        assert!(files.iter().all(|p| p.starts_with(root.path())));
    }
}
