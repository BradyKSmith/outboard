use serde::Deserialize;
use std::path::PathBuf;

use crate::paths;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Overrides the default worktree root (`<home>/worktrees`).
    pub worktree_root: Option<PathBuf>,
    /// Prefix applied to branch names on the git side only (e.g. "brady/").
    pub branch_prefix: Option<String>,
    #[serde(default)]
    pub rename_prompt: RenamePrompt,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenamePrompt {
    pub enabled: bool,
    /// Agent instruction file the prompt is written to.
    pub file: String,
}

impl Default for RenamePrompt {
    fn default() -> Self {
        Self {
            enabled: true,
            file: "AGENTS.md".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = paths::config_file()?;
        match std::fs::read_to_string(&path) {
            Ok(text) => toml::from_str(&text)
                .map_err(|e| anyhow::anyhow!("invalid config at {}: {e}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(anyhow::anyhow!("cannot read {}: {e}", path.display())),
        }
    }

    pub fn worktree_root(&self) -> anyhow::Result<PathBuf> {
        match &self.worktree_root {
            Some(root) => Ok(root.clone()),
            None => paths::worktrees_dir(),
        }
    }
}
