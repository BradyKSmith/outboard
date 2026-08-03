use std::path::PathBuf;

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
