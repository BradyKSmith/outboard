# Outboard

## Mission Statement

Outboard is a CLI-first workspace orchestrator for long-running, agent-driven software development.

Its purpose is to give developers a fast, repeatable, and isolated way to create disposable development environments without turning the underlying machine into a collection of fragile, manually maintained workspaces.

Outboard treats the host machine as persistent infrastructure, the Git worktree as the durable unit of work, and the execution environment as disposable.

It is designed for developers who prefer terminals, remote access, multiplexers, Git, and automation over heavyweight graphical workspace managers.

---

## The Problem Outboard Solves

Modern coding agents can work autonomously for hours, modify large portions of a codebase, install dependencies, run containers, launch browsers, and execute tests.

Running those agents directly on a developer’s primary operating system creates several problems:

- Agents can modify host configuration.
- Dependencies from different projects can conflict.
- Docker resources accumulate and become difficult to manage.
- Long-running sessions are tied to a laptop or temporary terminal.
- Multiple concurrent tasks become difficult to organize.
- Workspaces are often created manually and inconsistently.
- An agent may gain access to unrelated repositories, credentials, or files.
- Reproducing a workspace on another machine is difficult.

Traditional ephemeral virtual machines solve many of these problems, but they introduce provisioning time, infrastructure complexity, and ongoing cloud cost that are unnecessary for individual developers and small teams.

Outboard provides the useful parts of ephemeral development environments without requiring every task to receive an entire cloud VM.

---

## Core Concept

Outboard separates a development workspace into three distinct layers.

### Persistent Workspace

The persistent workspace contains the work that must survive runtime failure or replacement.

This includes:

- Git worktree
- Branch state
- Outboard workspace metadata
- Logs and artifacts
- Optional persistent browser or application state

The workspace lives on the host and remains available even when no agent or runtime is running.

### Disposable Runtime

The runtime contains everything required to execute the work.

This may include:

- Coding agent
- Shell and development tools
- Language runtimes
- Package managers
- Docker daemon
- Application services
- Browser
- Playwright
- Build and test tooling

The runtime is isolated from the host and may be destroyed and recreated without losing the worktree.

Docker Sandboxes is the initial preferred runtime because it provides each agent with an isolated microVM while still allowing a selected Git worktree to be mounted into the environment.

### Workspace Orchestration

Outboard coordinates the relationship between the persistent workspace and the disposable runtime.

It creates the worktree, launches the runtime, starts the agent, tracks workspace state, and provides a consistent way to reconnect, stop, resume, archive, or destroy the environment.

---

## Product Vision

Outboard should make starting an isolated coding task feel as lightweight as creating a Git branch.

A developer should be able to run a command such as:

```bash
outboard create fix-authentication
```

Outboard would then:

1. Create a Git worktree and branch.
2. Create the workspace metadata.
3. Launch an isolated runtime.
4. Mount or clone the worktree into that runtime.
5. Start the selected coding agent.
6. Prepare the project’s development and testing environment.
7. Register the workspace so it can be resumed later.

The developer should not need to manually configure containers, terminal sessions, browser profiles, ports, or working directories for every task.

---

## What Outboard Is

Outboard is:

- A workspace lifecycle manager
- A Git worktree orchestrator
- A runtime abstraction for coding agents
- A CLI-first alternative to graphical agent workspace applications
- A persistent control layer for disposable development environments
- A tool for managing multiple concurrent agent-driven tasks
- A bridge between local development and ephemeral cloud development

---

## What Outboard Is Not

Outboard is not:

- A coding agent
- An IDE
- A terminal multiplexer
- A replacement for Git
- A container runtime
- A virtual machine platform
- A CI/CD system
- A graphical desktop environment

Outboard should integrate with these tools rather than replacing them.

Docker provides isolation.

Git provides source control.

Claude Code, Codex, and other agents perform the development work.

tmux, Herdr, or another terminal interface provides session interaction.

Outboard owns the lifecycle that connects them.

---

## Primary Goals

### Fast Workspace Creation

Creating an isolated development workspace should take seconds rather than requiring manual setup or cloud provisioning.

### Durable Work, Disposable Execution

Code and workspace metadata must survive the destruction or failure of the runtime.

The runtime should be replaceable without losing meaningful work.

### Agent Isolation

Coding agents should operate inside an isolated environment rather than directly on the host operating system.

The agent may be granted access to a specific worktree without receiving unrestricted access to the rest of the host.

### Concurrent Development

A developer should be able to run multiple independent workspaces and agents against the same repository without branch conflicts or dependency collisions.

### Reproducibility

A workspace should be describable through repository configuration and recreated on another compatible machine.

### Remote-First Operation

Outboard should work naturally over SSH and inside terminal multiplexers.

It should not require a local graphical interface.

### Runtime Portability

Outboard should not be permanently coupled to one execution technology.

Docker Sandboxes may be the initial runtime, but the architecture should allow future runtimes such as:

- Standard Docker
- Podman
- Local virtual machines
- Azure virtual machines
- Remote Linux hosts
- Other sandbox or microVM platforms

---

## Isolation Modes

Outboard should recognize that different tasks require different levels of isolation.

### Mounted Worktree Mode

The host creates and owns the Git worktree.

The worktree is mounted read/write into the sandbox.

The agent runs inside the sandbox, while its file edits appear immediately in the host worktree.

This should be the default for trusted repositories and normal development.

### Cloned Workspace Mode

The sandbox receives a private clone of the repository rather than direct write access to the host worktree.

The agent’s changes remain inside the sandbox until they are explicitly exported, committed, or fetched.

This mode is intended for untrusted code, experimental agents, and higher-risk autonomous tasks.

---

## Intended User Experience

Outboard should feel predictable and boring in the best possible way.

A developer should be able to:

- Create a workspace.
- Attach to its agent session.
- Leave the session running for hours.
- Disconnect from the host.
- Reconnect from another machine.
- Inspect the Git diff.
- Replace a failed runtime.
- Resume the task.
- Archive or destroy the workspace when finished.

The developer should not need to remember where the worktree lives, which container belongs to it, which ports were assigned, or which terminal session contains the agent.

Outboard should maintain that relationship.

---

## Long-Term Direction

Outboard aims to become a portable control plane for agentic software development workspaces.

The same workspace definition should eventually be capable of running:

- Locally on a Mac or Linux workstation
- Inside Docker Sandboxes
- On a dedicated remote development machine
- On an ephemeral cloud VM
- On a shared development host

The runtime may change, but the developer workflow should remain consistent.

Outboard’s long-term value is not the isolation technology itself.

Its value is providing a stable, developer-friendly workflow around Git, agents, runtimes, and workspace lifecycle management.

---

## Product Principle

The central principle of Outboard is:

> The workspace is persistent. The runtime is replaceable.

Git holds the code.

The host holds the workspace identity.

The sandbox performs the work.

Outboard connects them.
