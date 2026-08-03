use crate::config::Config;
use crate::{git, prompt, registry};

pub fn run(workspace: &str, new_branch: &str) -> anyhow::Result<()> {
    let mut record = registry::resolve(workspace)?
        .ok_or_else(|| anyhow::anyhow!("no workspace or branch named '{workspace}'"))?;
    if !record.worktree.exists() {
        anyhow::bail!(
            "worktree for '{}' is missing from disk ({})",
            record.name,
            record.worktree.display()
        );
    }

    let config = Config::load()?;
    let prefix = config.branch_prefix.unwrap_or_default();
    let new_branch = if !prefix.is_empty() && !new_branch.starts_with(&prefix) {
        format!("{prefix}{new_branch}")
    } else {
        new_branch.to_string()
    };
    if new_branch == record.branch {
        println!("branch is already named '{new_branch}'");
        return Ok(());
    }
    if git::branch_exists(&record.repo, &new_branch) {
        anyhow::bail!("branch '{new_branch}' already exists in this repository");
    }

    let had_upstream = git::succeeds(
        &record.worktree,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{}@{{upstream}}", record.branch),
        ],
    );

    git::run(
        &record.worktree,
        &["branch", "-m", &record.branch, &new_branch],
    )?;

    if let Some(state) = record.rename_prompt.take() {
        prompt::remove(&record.repo, &record.worktree, &record.name, &state)?;
    }

    let old_branch = std::mem::replace(&mut record.branch, new_branch.clone());
    registry::save(&record)?;

    println!("workspace '{}': branch '{old_branch}' → '{new_branch}'", record.name);
    if had_upstream {
        println!("note: the remote still has '{old_branch}'; the upstream was carried over");
    }
    Ok(())
}
