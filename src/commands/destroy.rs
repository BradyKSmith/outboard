use crate::cli::DestroyArgs;
use crate::{git, prompt, registry};

pub fn run(args: DestroyArgs) -> anyhow::Result<()> {
    let mut record = registry::resolve(&args.name)?
        .ok_or_else(|| anyhow::anyhow!("no workspace or branch named '{}'", args.name))?;
    let name = record.name.clone();
    let worktree_exists = record.worktree.exists();

    if !args.force {
        if worktree_exists && git::is_dirty(&record.worktree)? {
            anyhow::bail!(
                "workspace '{name}' has uncommitted changes — commit or stash them, \
                 or `outboard destroy {name} --force` to discard"
            );
        }
        if !args.keep_branch && git::branch_exists(&record.repo, &record.branch) {
            let base = git::default_branch(&record.repo)?;
            let merged = git::is_merged_into(&record.repo, &record.branch, &base);
            let pushed = git::is_pushed(&record.repo, &record.branch)?;
            if !merged && !pushed {
                anyhow::bail!(
                    "branch '{}' has commits that exist nowhere else (not merged into \
                     '{base}', not pushed) — push or merge first, use --keep-branch to \
                     preserve the branch, or --force to discard",
                    record.branch
                );
            }
        }
    }

    if worktree_exists {
        let path = record.worktree.to_string_lossy().to_string();
        let mut cmd = vec!["worktree", "remove"];
        if args.force {
            cmd.push("--force");
        }
        cmd.push(&path);
        git::run(&record.repo, &cmd)?;
    } else {
        git::run(&record.repo, &["worktree", "prune"])?;
    }

    if let Some(state) = record.rename_prompt.take() {
        prompt::remove(&record.repo, &record.worktree, &name, &state)?;
    }

    let mut kept_branch = false;
    if git::branch_exists(&record.repo, &record.branch) {
        if args.keep_branch {
            kept_branch = true;
        } else {
            git::run(&record.repo, &["branch", "-D", &record.branch])?;
        }
    }

    registry::delete(&name)?;

    if kept_branch {
        println!("destroyed workspace '{name}' (kept branch '{}')", record.branch);
    } else {
        println!("destroyed workspace '{name}'");
    }
    Ok(())
}
