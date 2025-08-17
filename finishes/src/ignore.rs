use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub fn build(root: &Path) -> Result<Gitignore, Box<dyn std::error::Error>> {
    let mut builder = GitignoreBuilder::new(root);
    builder.add(root.join(".gitignore"));
    builder.add(root.join(".finishesignore"));
    Ok(builder.build()?)
}
