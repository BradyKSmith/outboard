#!/usr/bin/env bash
# Build Outboard from source and wire it into the shell.
# Idempotent: re-run any time to install the latest build.
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="${OUTBOARD_BIN_DIR:-$HOME/.local/bin}"
ZSHRC="${ZDOTDIR:-$HOME}/.zshrc"

say() { printf '\033[1m==>\033[0m %s\n' "$*"; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }

command -v git >/dev/null 2>&1 || die "git is required"
command -v cargo >/dev/null 2>&1 || die "cargo is required — install Rust via https://rustup.rs"

say "Building release binary"
cargo build --release --manifest-path "$REPO_DIR/Cargo.toml"

say "Installing to $BIN_DIR/outboard"
mkdir -p "$BIN_DIR"
install -m 755 "$REPO_DIR/target/release/outboard" "$BIN_DIR/outboard"

# PATH check (append only if the bin dir is genuinely absent).
case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *)
    if ! grep -qs "\.local/bin" "$ZSHRC"; then
      say "Adding $BIN_DIR to PATH in $ZSHRC"
      printf '\nexport PATH="%s:$PATH"\n' "$BIN_DIR" >> "$ZSHRC"
    fi
    ;;
esac

# Shell integration: the eval wrapper that makes create/cd change directory.
EVAL_LINE='eval "$(outboard init zsh)"'
if grep -qsF "$EVAL_LINE" "$ZSHRC"; then
  say "Shell integration already present in $ZSHRC"
else
  say "Adding shell integration to $ZSHRC"
  printf '\n# Outboard: cd into workspaces from create/cd\n%s\n' "$EVAL_LINE" >> "$ZSHRC"
fi

say "Installed $("$BIN_DIR/outboard" --version)"
say "Reload your terminal (or run: exec zsh) and you're good to go"
