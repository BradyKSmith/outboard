use crate::cli::Shell;
use crate::shell;

pub fn run(sh: Shell) -> anyhow::Result<()> {
    match sh {
        Shell::Zsh => print!("{}", shell::ZSH_INIT),
    }
    Ok(())
}
