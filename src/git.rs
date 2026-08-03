use std::path::Path;
use std::process::Command;

/// Run `git <args>` in `dir`, returning trimmed stdout on success and a
/// message containing git's stderr on failure. All git interaction goes
/// through the system binary (ADR 0002).
pub fn run(dir: &Path, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| anyhow::anyhow!("failed to invoke git (is it installed?): {e}"))?;
    if !output.status.success() {
        anyhow::bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
