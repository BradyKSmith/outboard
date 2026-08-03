use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::paths;
use crate::prompt::RenamePromptState;

/// One workspace record, stored as `registry/<name>.json`. Holds only what
/// git cannot know; git remains the source of truth for code state.
#[derive(Serialize, Deserialize, Clone)]
pub struct WorkspaceRecord {
    /// Permanent workspace name (see ADR 0001).
    pub name: String,
    /// Path to the main repository this workspace belongs to.
    pub repo: PathBuf,
    /// Path to the workspace's worktree.
    pub worktree: PathBuf,
    /// Current branch name (updated by `outboard rename`).
    pub branch: String,
    /// Whether the name was generated (drives the rename prompt).
    pub generated_name: bool,
    /// Unix epoch seconds at creation.
    pub created_at: u64,
    /// Present while a rename prompt is active in the worktree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_prompt: Option<RenamePromptState>,
}

pub fn load(name: &str) -> anyhow::Result<Option<WorkspaceRecord>> {
    let path = paths::registry_dir()?.join(format!("{name}.json"));
    match std::fs::read_to_string(&path) {
        Ok(text) => Ok(Some(serde_json::from_str(&text).map_err(|e| {
            anyhow::anyhow!("corrupt registry entry {}: {e}", path.display())
        })?)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(anyhow::anyhow!("cannot read {}: {e}", path.display())),
    }
}

pub fn save(record: &WorkspaceRecord) -> anyhow::Result<()> {
    let dir = paths::registry_dir()?;
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", record.name));
    let json = serde_json::to_string_pretty(record)?;
    std::fs::write(&path, json)?;
    Ok(())
}

pub fn delete(name: &str) -> anyhow::Result<()> {
    let path = paths::registry_dir()?.join(format!("{name}.json"));
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(anyhow::anyhow!("cannot remove {}: {e}", path.display())),
    }
}

pub fn list() -> anyhow::Result<Vec<WorkspaceRecord>> {
    let dir = paths::registry_dir()?;
    let mut records = Vec::new();
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(records),
        Err(e) => return Err(anyhow::anyhow!("cannot read {}: {e}", dir.display())),
    };
    for entry in entries {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            let text = std::fs::read_to_string(&path)?;
            records.push(serde_json::from_str(&text).map_err(|e| {
                anyhow::anyhow!("corrupt registry entry {}: {e}", path.display())
            })?);
        }
    }
    records.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(records)
}

/// Resolve a workspace by name or by current branch name (commands accept
/// either handle).
pub fn resolve(handle: &str) -> anyhow::Result<Option<WorkspaceRecord>> {
    if let Some(record) = load(handle)? {
        return Ok(Some(record));
    }
    Ok(list()?.into_iter().find(|r| r.branch == handle))
}
