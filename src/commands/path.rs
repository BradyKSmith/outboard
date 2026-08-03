use crate::registry;

pub fn run(name: &str) -> anyhow::Result<()> {
    match registry::resolve(name)? {
        Some(record) => {
            println!("{}", record.worktree.display());
            Ok(())
        }
        None => anyhow::bail!("no workspace or branch named '{name}'"),
    }
}
