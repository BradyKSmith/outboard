use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::CreateArgs;
use crate::config::Config;
use crate::{git, names, paths, prompt, registry, shell};

pub fn run(args: CreateArgs) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let Some(repo) = git::repo_root(&cwd)? else {
        anyhow::bail!("outboard create must be run inside a git repository");
    };
    let config = Config::load()?;
    let prefix = config.branch_prefix.clone().unwrap_or_default();

    let generated = args.name.is_none();
    let name = match args.name {
        Some(name) => {
            validate_name(&name)?;
            if registry::load(&name)?.is_some() {
                anyhow::bail!(
                    "workspace '{name}' already exists — `outboard cd {name}` to enter it"
                );
            }
            if git::branch_exists(&repo, &format!("{prefix}{name}")) {
                anyhow::bail!(
                    "branch '{prefix}{name}' already exists in this repository — pick another name"
                );
            }
            name
        }
        None => names::generate(|candidate| {
            registry::load(candidate).map(|r| r.is_some()).unwrap_or(true)
                || git::branch_exists(&repo, &format!("{prefix}{candidate}"))
        }),
    };
    let branch = format!("{prefix}{name}");

    let base = match &args.base {
        Some(base) => base.clone(),
        None => git::default_branch(&repo)?,
    };

    let worktree = config
        .worktree_root()?
        .join(paths::repo_slug(&repo))
        .join(&name);
    if worktree.exists() {
        anyhow::bail!("worktree path {} already exists", worktree.display());
    }
    if let Some(parent) = worktree.parent() {
        std::fs::create_dir_all(parent)?;
    }

    git::run(
        &repo,
        &[
            "worktree",
            "add",
            "-b",
            &branch,
            &worktree.to_string_lossy(),
            &base,
        ],
    )?;

    let rename_prompt = if generated && config.rename_prompt.enabled {
        Some(prompt::inject(
            &repo,
            &worktree,
            &name,
            &config.rename_prompt.file,
        )?)
    } else {
        None
    };

    let record = registry::WorkspaceRecord {
        name: name.clone(),
        repo,
        worktree: worktree.clone(),
        branch: branch.clone(),
        generated_name: generated,
        created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        rename_prompt,
    };
    registry::save(&record)?;

    if shell::integrated() {
        eprintln!("Created workspace '{name}' (branch '{branch}' from '{base}')");
        println!("{}", worktree.display());
    } else {
        println!("Created workspace '{name}' (branch '{branch}' from '{base}')");
        println!("  {}", worktree.display());
        println!("hint: add `eval \"$(outboard init zsh)\"` to .zshrc to land in new workspaces automatically");
    }
    Ok(())
}

/// Workspace names become branch names, directory names, and registry file
/// names, so keep them to a safe kebab charset.
fn validate_name(name: &str) -> anyhow::Result<()> {
    let valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && !name.starts_with('-');
    if !valid {
        anyhow::bail!(
            "invalid workspace name '{name}': use lowercase letters, digits, '-' and '_'"
        );
    }
    Ok(())
}
