use std::path::{Path, PathBuf};
use std::process::Command;

/// Run `git <args>` in `dir`, returning trimmed stdout on success and a
/// message containing git's stderr on failure. All git interaction goes
/// through the system binary (ADR 0002).
pub fn run(dir: &Path, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| anyhow::anyhow!("failed to invoke git (is it installed?): {e}"))?;
    if !output.status.success() {
        anyhow::bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Like `run`, but a non-zero exit is an expected answer, not an error.
pub fn try_run(dir: &Path, args: &[&str]) -> anyhow::Result<Option<String>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| anyhow::anyhow!("failed to invoke git (is it installed?): {e}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&output.stdout).trim().to_string()))
}

pub fn succeeds(dir: &Path, args: &[&str]) -> bool {
    matches!(try_run(dir, args), Ok(Some(_)))
}

/// Root of the repository containing `dir`, if any.
pub fn repo_root(dir: &Path) -> anyhow::Result<Option<PathBuf>> {
    Ok(try_run(dir, &["rev-parse", "--show-toplevel"])?.map(PathBuf::from))
}

/// The repository's shared .git directory (common across worktrees).
pub fn common_dir(repo: &Path) -> anyhow::Result<PathBuf> {
    run(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
    .map(PathBuf::from)
}

pub fn branch_exists(repo: &Path, branch: &str) -> bool {
    succeeds(
        repo,
        &["rev-parse", "--verify", "--quiet", &format!("refs/heads/{branch}")],
    )
}

/// The repository's default branch: what origin/HEAD points at when it
/// resolves to a local branch, else main/master if present, else the
/// currently checked-out branch.
pub fn default_branch(repo: &Path) -> anyhow::Result<String> {
    if let Some(remote_head) =
        try_run(repo, &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])?
        && let Some(name) = remote_head.strip_prefix("origin/")
            && branch_exists(repo, name) {
                return Ok(name.to_string());
            }
    for candidate in ["main", "master"] {
        if branch_exists(repo, candidate) {
            return Ok(candidate.to_string());
        }
    }
    run(repo, &["symbolic-ref", "--short", "HEAD"])
        .map_err(|_| anyhow::anyhow!("cannot determine the repository's default branch"))
}

/// True when the worktree has uncommitted changes, including untracked files.
pub fn is_dirty(worktree: &Path) -> anyhow::Result<bool> {
    Ok(!run(worktree, &["status", "--porcelain"])?.is_empty())
}

/// True when every commit on `branch` is reachable from `other`.
pub fn is_merged_into(repo: &Path, branch: &str, other: &str) -> bool {
    Command::new("git")
        .args(["merge-base", "--is-ancestor", branch, other])
        .current_dir(repo)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// True when the branch has an upstream and no commits the upstream lacks.
pub fn is_pushed(repo: &Path, branch: &str) -> anyhow::Result<bool> {
    let upstream = format!("{branch}@{{upstream}}");
    if try_run(repo, &["rev-parse", "--verify", "--quiet", &upstream])?.is_none() {
        return Ok(false);
    }
    let unpushed = run(repo, &["rev-list", "--count", &format!("{upstream}..{branch}")])?;
    Ok(unpushed == "0")
}

/// `(behind, ahead)` of `branch` relative to `base`.
pub fn behind_ahead(repo: &Path, base: &str, branch: &str) -> anyhow::Result<(u32, u32)> {
    let counts = run(
        repo,
        &["rev-list", "--left-right", "--count", &format!("{base}...{branch}")],
    )?;
    let mut parts = counts.split_whitespace();
    let behind = parts.next().unwrap_or("0").parse().unwrap_or(0);
    let ahead = parts.next().unwrap_or("0").parse().unwrap_or(0);
    Ok((behind, ahead))
}
