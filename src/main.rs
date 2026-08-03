use clap::Parser;

mod cli;
mod commands;
mod config;
mod git;
mod names;
mod paths;
mod prompt;
mod registry;
mod shell;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Create(args) => commands::create::run(args),
        cli::Command::Init { shell } => commands::init::run(shell),
        cli::Command::Cd { name } => commands::cd::run(name),
        cli::Command::Path { name } => commands::path::run(&name),
        cli::Command::Ls { all } => commands::ls::run(all),
        cli::Command::Rename { workspace, branch } => commands::rename::run(&workspace, &branch),
        cli::Command::Destroy(args) => commands::destroy::run(args),
    }
}
