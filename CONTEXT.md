# Outboard — Ubiquitous Language

Glossary of domain terms. Keep this free of implementation details.

## Workspace

The durable, named unit of work Outboard manages: a Git worktree, its branch,
and Outboard's record of them. A workspace's **name is permanent** — assigned
at creation and never changed for the life of the workspace. Its directory
location is likewise stable.

Not to be confused with a *Herdr workspace* (a UI container of tabs/panes) or
a *branch* (see below).

## Workspace Name

The permanent handle for a workspace (`oslo`, `fix-auth`). Either user-chosen
at creation or a **Generated Name** picked from a built-in city wordlist when
the user provides none. Used in every Outboard command.

## Generated Name (Placeholder)

A city name auto-assigned to a workspace created without an explicit name. It
is a real, permanent workspace name — "placeholder" refers only to the
*branch* it initially names, which is expected to be renamed once the task is
understood.

## Branch

The Git branch carrying the workspace's work. Created with the workspace,
initially named after it. The branch is the **meaning carrier**: it may be
renamed (by the developer or an agent) to describe the task. Renaming the
branch never changes the workspace name or directory.

## Rename

Changing a workspace's *branch* name. Workspaces themselves are never renamed.

## Base

The ref a workspace's branch is created from. Defaults to the repository's
default branch, not the currently checked-out branch.

## Registry

Outboard's host-level record of all workspaces (name, repository, worktree
location, branch, creation metadata). Git remains the source of truth for
code state; the registry holds only what Git cannot know.

## Worktree Root

The single configurable directory under which all Outboard-managed worktrees
live, organized by repository. Placement is Outboard's concern, never the
user's.

## Rename Prompt

The instruction Outboard leaves for a coding agent in a workspace with a
generated name, asking the agent to rename the branch descriptively once the
task is understood. Never committed to the repository.

## Agent

Any coding agent the developer runs inside a workspace. Opaque to Outboard:
Outboard does not launch, manage, or enumerate agents (v0).

## Destroy

Full teardown of a workspace: worktree, branch, and registry entry, guarded
by safety checks for unsaved or unmerged work.
