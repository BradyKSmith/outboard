//! The rename prompt: an instruction block left for the coding agent in a
//! workspace with a generated name, asking it to rename the branch once the
//! task is understood. The block must never be committable.
//!
//! Two mechanisms, both verified against git's actual behavior:
//! - File absent in the repo: Outboard creates it and lists it in the shared
//!   `.git/info/exclude` with a workspace-tagged marker line (git has no
//!   per-worktree exclude). The entry is removed on rename/destroy.
//! - File tracked by the repo: Outboard appends a marked block and sets the
//!   `skip-worktree` bit, which hides the change from status/diff/add. The
//!   block is stripped and the bit cleared on rename.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::git;

pub const BEGIN: &str = "<!-- outboard:begin -->";
pub const END: &str = "<!-- outboard:end -->";

#[derive(Serialize, Deserialize, Clone)]
pub struct RenamePromptState {
    /// Instruction file name, relative to the worktree root.
    pub file: String,
    pub mode: Mode,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// Outboard created the file and added a shared-exclude entry.
    Created,
    /// Outboard appended to a tracked file under skip-worktree.
    Appended,
}

fn block(workspace: &str) -> String {
    format!(
        "{BEGIN}\nThis workspace's branch has the placeholder name \"{workspace}\". Once you\n\
         understand the task, choose a short kebab-case branch name and run:\n\n    \
         outboard rename {workspace} <descriptive-branch-name>\n\n\
         This block is managed by Outboard and disappears after the rename.\n{END}\n"
    )
}

fn exclude_marker(workspace: &str) -> String {
    format!("# outboard:{workspace}")
}

/// Inject the rename prompt into a freshly created worktree.
pub fn inject(
    repo: &Path,
    worktree: &Path,
    workspace: &str,
    file: &str,
) -> anyhow::Result<RenamePromptState> {
    let target = worktree.join(file);
    if target.exists() {
        let mut text = std::fs::read_to_string(&target)?;
        if !text.ends_with('\n') {
            text.push('\n');
        }
        text.push('\n');
        text.push_str(&block(workspace));
        std::fs::write(&target, text)?;
        git::run(worktree, &["update-index", "--skip-worktree", file])?;
        Ok(RenamePromptState {
            file: file.to_string(),
            mode: Mode::Appended,
        })
    } else {
        std::fs::write(&target, block(workspace))?;
        let exclude = git::common_dir(repo)?.join("info").join("exclude");
        if let Some(dir) = exclude.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut text = std::fs::read_to_string(&exclude).unwrap_or_default();
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(&format!("{}\n/{}\n", exclude_marker(workspace), file));
        std::fs::write(&exclude, text)?;
        Ok(RenamePromptState {
            file: file.to_string(),
            mode: Mode::Created,
        })
    }
}

/// Remove every trace of the prompt. Safe to call when the worktree is
/// already gone (destroy); missing pieces are skipped.
pub fn remove(
    repo: &Path,
    worktree: &Path,
    workspace: &str,
    state: &RenamePromptState,
) -> anyhow::Result<()> {
    match state.mode {
        Mode::Created => {
            let target = worktree.join(&state.file);
            if target.exists() {
                std::fs::remove_file(&target)?;
            }
            let exclude = git::common_dir(repo)?.join("info").join("exclude");
            if let Ok(text) = std::fs::read_to_string(&exclude) {
                let marker = exclude_marker(workspace);
                let mut lines: Vec<&str> = text.lines().collect();
                if let Some(pos) = lines.iter().position(|l| *l == marker) {
                    lines.remove(pos);
                    if pos < lines.len() && lines[pos] == format!("/{}", state.file) {
                        lines.remove(pos);
                    }
                    let mut out = lines.join("\n");
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    std::fs::write(&exclude, out)?;
                }
            }
        }
        Mode::Appended => {
            let target = worktree.join(&state.file);
            if let Ok(text) = std::fs::read_to_string(&target) {
                std::fs::write(&target, strip_block(&text))?;
            }
            if worktree.exists() {
                // Best-effort: the bit vanishes with the worktree anyway.
                let _ = git::try_run(
                    worktree,
                    &["update-index", "--no-skip-worktree", &state.file],
                );
            }
        }
    }
    Ok(())
}

/// Remove the marked block (and the blank line Outboard added before it).
fn strip_block(text: &str) -> String {
    let Some(begin) = text.find(BEGIN) else {
        return text.to_string();
    };
    let Some(end_start) = text[begin..].find(END) else {
        return text.to_string();
    };
    let mut end = begin + end_start + END.len();
    if text[end..].starts_with('\n') {
        end += 1;
    }
    let mut head = &text[..begin];
    while head.ends_with("\n\n") {
        head = &head[..head.len() - 1];
    }
    format!("{head}{}", &text[end..])
}
