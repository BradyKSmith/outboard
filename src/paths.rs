use std::path::{Path, PathBuf};

/// Everything Outboard owns lives under one home directory (default
/// `~/.outboard`, overridable via `OUTBOARD_HOME`).
pub fn home() -> anyhow::Result<PathBuf> {
    if let Some(dir) = std::env::var_os("OUTBOARD_HOME") {
        return Ok(PathBuf::from(dir));
    }
    let home = std::env::var_os("HOME")
        .ok_or_else(|| anyhow::anyhow!("HOME is not set and OUTBOARD_HOME was not provided"))?;
    Ok(PathBuf::from(home).join(".outboard"))
}

pub fn config_file() -> anyhow::Result<PathBuf> {
    Ok(home()?.join("config.toml"))
}

pub fn registry_dir() -> anyhow::Result<PathBuf> {
    Ok(home()?.join("registry"))
}

pub fn worktrees_dir() -> anyhow::Result<PathBuf> {
    Ok(home()?.join("worktrees"))
}

/// Directory name for a repository under the worktree root: its basename
/// plus a short stable hash of its absolute path, so same-named repos never
/// collide. FNV-1a rather than the std hasher, whose output may change
/// across Rust releases while these paths must stay stable.
pub fn repo_slug(repo: &Path) -> String {
    let name = repo
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".to_string());
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in repo.as_os_str().as_encoded_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{name}-{:08x}", (hash >> 32) as u32 ^ hash as u32)
}
