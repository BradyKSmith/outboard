# Shell out to the system git binary; no embedded git library

Outboard invokes the system `git` executable for every Git operation instead
of embedding a library (libgit2 bindings, gitoxide). Worktrees are a newer,
quirkier corner of Git where library support lags and diverges; the system
binary is the only implementation guaranteed to match what the user gets by
hand, which also makes every Outboard failure reproducible and debuggable by
copy-pasting the command. The cost — parsing process output and requiring
`git` on PATH — is acceptable because `git` is already Outboard's stated
single runtime dependency.
