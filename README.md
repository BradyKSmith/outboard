# Outboard

Git worktree workspace manager: durable named workspaces, zero path-thinking.

`outboard create` gives you a branch + worktree under one permanent name (a
generated city name if you don't provide one) and drops you into it.
`outboard cd`, `ls`, `rename`, and `destroy` handle the rest of the
lifecycle, including safety-checked teardown. Outboard depends on `git` and
nothing else.

```sh
eval "$(outboard init zsh)"   # once, in .zshrc — enables the cd behavior

outboard create               # new workspace "oslo", shell now inside it
outboard rename oslo fix-auth # branch renamed; workspace name is permanent
outboard ls                   # oslo → fix-auth  dirty, 2 ahead
outboard destroy oslo         # worktree + branch + record, safety-checked
```

- Vision and long-term direction: [outboard-product-spec.md](./outboard-product-spec.md)
- Domain vocabulary: [CONTEXT.md](./CONTEXT.md)
- Decisions: [docs/adr/](./docs/adr/)

Status: v0 in progress — CLI surface and design settled; `create`, `cd`,
`ls`, `rename`, and `destroy` are stubs pending implementation.
