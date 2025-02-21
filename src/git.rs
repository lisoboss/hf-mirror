use git2::{Repository, TreeWalkMode, TreeWalkResult};
use std::path::PathBuf;

pub(crate) fn commit_head_id(path: &PathBuf) -> anyhow::Result<String> {
    let repo = Repository::open(path)?;
    let head = repo.head()?.peel_to_commit()?;
    Ok(head.id().to_string())
}

pub(crate) fn file_tree(
    path: &PathBuf,
    mut callback: impl FnMut(String, String, usize) -> (),
) -> anyhow::Result<()> {
    let repo = Repository::open(path)?;
    let head = repo.head()?.peel_to_commit()?;
    let tree = head.tree()?;

    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        if let Some(obj) = entry.to_object(&repo).ok() {
            if let Some(blob) = obj.as_blob() {
                let path = format!("{}{}", root, entry.name().unwrap_or(""));
                let oid = entry.id().to_string();
                let size = blob.size();

                callback(path, oid, size);
            }
        }
        TreeWalkResult::Ok
    })?;

    Ok(())
}
