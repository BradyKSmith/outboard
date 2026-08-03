use clap::{Args, Parser, Subcommand, ValueEnum};

/// Git worktree workspace manager: durable workspaces, zero path-thinking.
#[derive(Parser)]
#[command(name = "outboard", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a workspace: branch + worktree, registered under one permanent name
    Create(CreateArgs),
    /// Print the shell integration function, for `eval "$(outboard init zsh)"`
    Init {
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Change directory into a workspace (needs shell integration; bare `cd` lists workspaces)
    Cd {
        /// Workspace name or branch name
        name: Option<String>,
    },
    /// Print a workspace's worktree path
    Path {
        /// Workspace name or branch name
        name: String,
    },
    /// List workspaces with branch and status
    Ls {
        /// List workspaces across all repositories
        #[arg(long, short)]
        all: bool,
    },
    /// Rename a workspace's branch (workspace names are permanent; see ADR 0001)
    Rename {
        workspace: String,
        branch: String,
    },
    /// Destroy a workspace: worktree, branch, and registry entry
    Destroy(DestroyArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    /// Workspace name; omit to get a generated city name
    pub name: Option<String>,
    /// Ref to branch from (default: the repository's default branch)
    #[arg(long)]
    pub base: Option<String>,
}

#[derive(Args)]
pub struct DestroyArgs {
    /// Workspace name or branch name
    pub name: String,
    /// Skip the safety checks for uncommitted or unpushed work
    #[arg(long)]
    pub force: bool,
    /// Remove the worktree and registry entry but keep the branch
    #[arg(long)]
    pub keep_branch: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Shell {
    Zsh,
}
