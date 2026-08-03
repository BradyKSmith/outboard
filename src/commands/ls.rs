use crate::{git, registry};

pub fn run(all: bool) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let current_repo = git::repo_root(&cwd)?;

    let mut records = registry::list()?;
    let scoped = !all && current_repo.is_some();
    if scoped {
        let repo = current_repo.as_ref().unwrap();
        records.retain(|r| &r.repo == repo);
    }
    if records.is_empty() {
        if scoped {
            println!("no workspaces for this repository (`outboard ls --all` for every repo)");
        } else {
            println!("no workspaces exist — `outboard create` makes one");
        }
        return Ok(());
    }

    let name_width = records.iter().map(|r| r.name.len()).max().unwrap_or(0);
    let branch_width = records.iter().map(|r| r.branch.len()).max().unwrap_or(0);
    let mut last_repo = None;
    for record in &records {
        if !scoped && last_repo != Some(&record.repo) {
            println!("{}", record.repo.display());
            last_repo = Some(&record.repo);
        }
        let indent = if scoped { "" } else { "  " };
        println!(
            "{indent}{:<name_width$}  → {:<branch_width$}  {}",
            record.name,
            record.branch,
            status(record)
        );
    }
    Ok(())
}

fn status(record: &registry::WorkspaceRecord) -> String {
    if !record.worktree.exists() {
        return "worktree missing".to_string();
    }
    let mut parts = Vec::new();
    match git::is_dirty(&record.worktree) {
        Ok(true) => parts.push("dirty".to_string()),
        Ok(false) => parts.push("clean".to_string()),
        Err(_) => parts.push("status unavailable".to_string()),
    }
    if let Ok(base) = git::default_branch(&record.repo)
        && base != record.branch {
            match git::behind_ahead(&record.repo, &base, &record.branch) {
                Ok((0, 0)) => parts.push(format!("even with {base}")),
                Ok((behind, 0)) => parts.push(format!("merged, {behind} behind {base}")),
                Ok((0, ahead)) => parts.push(format!("{ahead} ahead of {base}")),
                Ok((behind, ahead)) => {
                    parts.push(format!("{ahead} ahead, {behind} behind {base}"))
                }
                Err(_) => {}
            }
        }
    parts.join(", ")
}
