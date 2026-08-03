/// The zsh integration emitted by `outboard init zsh`.
///
/// A child process cannot change its parent shell's directory, so `create`
/// and `cd` are wrapped: the binary prints the destination path on stdout
/// (human output goes to stderr when OUTBOARD_SHELL_INTEGRATION=1) and the
/// function performs the actual `cd`.
pub const ZSH_INIT: &str = r#"outboard() {
  case "$1" in
    create|cd)
      local dest
      dest="$(OUTBOARD_SHELL_INTEGRATION=1 command outboard "$@")" || return $?
      if [ -n "$dest" ]; then
        cd "$dest" || return $?
      fi
      ;;
    *)
      command outboard "$@"
      ;;
  esac
}
"#;

/// True when running under the shell wrapper: destination path goes to
/// stdout, everything else to stderr.
pub fn integrated() -> bool {
    std::env::var_os("OUTBOARD_SHELL_INTEGRATION").is_some()
}
