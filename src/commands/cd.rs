use std::io::{BufRead, Write};

use crate::{registry, shell};

pub fn run(name: Option<String>) -> anyhow::Result<()> {
    let record = match name {
        Some(name) => registry::resolve(&name)?
            .ok_or_else(|| anyhow::anyhow!("no workspace or branch named '{name}'"))?,
        None => pick()?,
    };
    if !record.worktree.exists() {
        anyhow::bail!(
            "worktree for '{}' is missing from disk ({})",
            record.name,
            record.worktree.display()
        );
    }
    if shell::integrated() {
        println!("{}", record.worktree.display());
    } else {
        println!("{}", record.worktree.display());
        eprintln!("hint: add `eval \"$(outboard init zsh)\"` to .zshrc so this actually cds");
    }
    Ok(())
}

/// Bare `outboard cd`: show the table and let the user pick by number.
fn pick() -> anyhow::Result<registry::WorkspaceRecord> {
    let records = registry::list()?;
    if records.is_empty() {
        anyhow::bail!("no workspaces exist — `outboard create` makes one");
    }
    let mut err = std::io::stderr().lock();
    for (i, record) in records.iter().enumerate() {
        writeln!(
            err,
            "{:>3}) {:<12} → {}",
            i + 1,
            record.name,
            record.branch
        )?;
    }
    write!(err, "workspace: ")?;
    err.flush()?;
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    let line = line.trim();
    if let Ok(index) = line.parse::<usize>()
        && index >= 1 && index <= records.len() {
            return Ok(records[index - 1].clone());
        }
    records
        .iter()
        .find(|r| r.name == line || r.branch == line)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no such workspace: '{line}'"))
}
