# Workspace names are permanent; only branches are renamed

A workspace's name (and therefore its directory under the worktree root) is
fixed at creation — either user-chosen or a generated city name — and never
changes. `outboard rename` renames only the Git *branch*, which carries the
task's meaning once it is understood (often renamed by the coding agent
working in the workspace). We adopted this split, modeled on Conductor,
because renaming a workspace would require `git worktree move` on a directory
that a shell, agent, or dev server is typically sitting inside — a rug-pull —
and because stable paths keep terminal history, editor sessions, and agent
context valid for the workspace's whole life.

## Considered Options

- **Rename everything (workspace + directory + branch) together** — gives
  self-describing handles (`outboard cd fix-auth-timeout`), but moves the
  directory under running processes and invalidates every recorded path.
  Rejected; the meaningful name lives in `outboard ls` output instead, and
  `cd` accepts either the workspace name or the branch name to compensate.
