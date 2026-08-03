//! End-to-end tests: each test gets its own OUTBOARD_HOME and fixture repo,
//! and drives the real binary the way a shell would.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

struct Fixture {
    root: PathBuf,
    home: PathBuf,
    repo: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "outboard-e2e-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let home = root.join("home");
        let repo = root.join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        let fixture = Self { root, home, repo };
        fixture.git(&["init", "-q", "-b", "main"]);
        fixture.git(&["commit", "-q", "--allow-empty", "-m", "init"]);
        fixture
    }

    fn git(&self, args: &[&str]) {
        self.git_in(&self.repo, args);
    }

    fn git_in(&self, dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(["-c", "user.email=t@t", "-c", "user.name=t"])
            .args(args)
            .current_dir(dir)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?} failed");
    }

    fn outboard(&self, args: &[&str]) -> Output {
        self.outboard_in(&self.repo, args, false)
    }

    fn outboard_in(&self, dir: &Path, args: &[&str], integrated: bool) -> Output {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_outboard"));
        cmd.args(args)
            .current_dir(dir)
            .env("OUTBOARD_HOME", &self.home);
        if integrated {
            cmd.env("OUTBOARD_SHELL_INTEGRATION", "1");
        }
        cmd.output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let out = self.outboard(args);
        assert!(
            out.status.success(),
            "outboard {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).to_string()
    }

    fn fails(&self, args: &[&str]) -> String {
        let out = self.outboard(args);
        assert!(!out.status.success(), "outboard {args:?} unexpectedly succeeded");
        String::from_utf8_lossy(&out.stderr).to_string()
    }

    fn worktree(&self, name: &str) -> PathBuf {
        let out = self.ok(&["path", name]);
        PathBuf::from(out.trim())
    }

    fn git_stdout(&self, dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git").args(args).current_dir(dir).output().unwrap();
        assert!(out.status.success(), "git {args:?} failed");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn create_explicit_name() {
    let f = Fixture::new();
    f.ok(&["create", "fix-auth"]);
    let wt = f.worktree("fix-auth");
    assert!(wt.exists());
    assert_eq!(f.git_stdout(&wt, &["branch", "--show-current"]), "fix-auth");
    // Explicit names get no rename prompt.
    assert!(!wt.join("AGENTS.md").exists());
    assert!(f.home.join("registry/fix-auth.json").exists());
}

#[test]
fn create_rejects_duplicates() {
    let f = Fixture::new();
    f.ok(&["create", "fix-auth"]);
    let err = f.fails(&["create", "fix-auth"]);
    assert!(err.contains("already exists"), "unexpected error: {err}");
    f.git(&["branch", "stray"]);
    let err = f.fails(&["create", "stray"]);
    assert!(err.contains("branch 'stray' already exists"), "unexpected error: {err}");
}

#[test]
fn create_rejects_bad_names_and_non_repos() {
    let f = Fixture::new();
    let err = f.fails(&["create", "Bad Name"]);
    assert!(err.contains("invalid workspace name"), "unexpected error: {err}");
    let outside = f.root.join("not-a-repo");
    std::fs::create_dir_all(&outside).unwrap();
    let out = f.outboard_in(&outside, &["create", "x"], false);
    assert!(!out.status.success());
}

#[test]
fn generated_name_injects_excluded_prompt() {
    let f = Fixture::new();
    let out = f.outboard_in(&f.repo, &["create"], true);
    assert!(out.status.success());
    let wt = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
    let name = wt.file_name().unwrap().to_string_lossy().to_string();

    let agents = std::fs::read_to_string(wt.join("AGENTS.md")).unwrap();
    assert!(agents.contains("<!-- outboard:begin -->"));
    assert!(agents.contains(&format!("outboard rename {name}")));
    // The prompt must be invisible to git.
    assert_eq!(f.git_stdout(&wt, &["status", "--porcelain"]), "");

    f.ok(&["rename", &name, "fix-timeout"]);
    assert!(!wt.join("AGENTS.md").exists());
    let exclude =
        std::fs::read_to_string(f.repo.join(".git/info/exclude")).unwrap_or_default();
    assert!(!exclude.contains("outboard"), "exclude not cleaned: {exclude}");
    assert_eq!(f.git_stdout(&wt, &["branch", "--show-current"]), "fix-timeout");
    // Resolvable by both handles after rename.
    assert_eq!(f.worktree(&name), wt);
    assert_eq!(f.worktree("fix-timeout"), wt);
}

#[test]
fn tracked_instruction_file_uses_skip_worktree() {
    let f = Fixture::new();
    std::fs::write(f.repo.join("AGENTS.md"), "# Project guide\n").unwrap();
    f.git(&["add", "AGENTS.md"]);
    f.git(&["commit", "-q", "-m", "agents"]);

    let out = f.outboard_in(&f.repo, &["create"], true);
    assert!(out.status.success());
    let wt = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
    let name = wt.file_name().unwrap().to_string_lossy().to_string();

    let agents = std::fs::read_to_string(wt.join("AGENTS.md")).unwrap();
    assert!(agents.starts_with("# Project guide"));
    assert!(agents.contains("<!-- outboard:begin -->"));
    assert_eq!(f.git_stdout(&wt, &["status", "--porcelain"]), "");

    f.ok(&["rename", &name, "add-payments"]);
    let restored = std::fs::read_to_string(wt.join("AGENTS.md")).unwrap();
    assert_eq!(restored, "# Project guide\n");
    assert_eq!(f.git_stdout(&wt, &["status", "--porcelain"]), "");
    assert_eq!(f.git_stdout(&wt, &["ls-files", "-v", "AGENTS.md"]), "H AGENTS.md");
}

#[test]
fn destroy_guards_and_teardown() {
    let f = Fixture::new();
    f.ok(&["create", "task"]);
    let wt = f.worktree("task");

    // Dirty worktree: refused.
    std::fs::write(wt.join("junk.txt"), "x").unwrap();
    let err = f.fails(&["destroy", "task"]);
    assert!(err.contains("uncommitted changes"), "unexpected error: {err}");

    // Unmerged, unpushed commits: refused.
    f.git_in(&wt, &["add", "-A"]);
    f.git_in(&wt, &["commit", "-q", "-m", "work"]);
    let err = f.fails(&["destroy", "task"]);
    assert!(err.contains("exist nowhere else"), "unexpected error: {err}");

    // keep-branch: allowed, branch survives, worktree and record gone.
    f.ok(&["destroy", "task", "--keep-branch"]);
    assert!(!wt.exists());
    assert!(!f.home.join("registry/task.json").exists());
    assert_eq!(f.git_stdout(&f.repo, &["branch", "--list", "task"]), "task");
}

#[test]
fn destroy_merged_workspace_without_ceremony() {
    let f = Fixture::new();
    f.ok(&["create", "done"]);
    let wt = f.worktree("done");
    f.ok(&["destroy", "done"]);
    assert!(!wt.exists());
    assert_eq!(f.git_stdout(&f.repo, &["branch", "--list", "done"]), "");
}

#[test]
fn destroy_force_discards_everything() {
    let f = Fixture::new();
    f.ok(&["create", "risky"]);
    let wt = f.worktree("risky");
    std::fs::write(wt.join("junk.txt"), "x").unwrap();
    f.ok(&["destroy", "risky", "--force"]);
    assert!(!wt.exists());
    assert_eq!(f.git_stdout(&f.repo, &["branch", "--list", "risky"]), "");
}

#[test]
fn ls_scopes_to_repo() {
    let f = Fixture::new();
    f.ok(&["create", "here"]);

    let other = f.root.join("other");
    std::fs::create_dir_all(&other).unwrap();
    f.git_in(&other, &["init", "-q", "-b", "main"]);
    f.git_in(&other, &["commit", "-q", "--allow-empty", "-m", "init"]);
    let out = f.outboard_in(&other, &["create", "there"], false);
    assert!(out.status.success());

    let scoped = f.ok(&["ls"]);
    assert!(scoped.contains("here") && !scoped.contains("there"), "{scoped}");
    let all = f.ok(&["ls", "--all"]);
    assert!(all.contains("here") && all.contains("there"), "{all}");
}

#[test]
fn integrated_create_prints_only_the_path() {
    let f = Fixture::new();
    let out = f.outboard_in(&f.repo, &["create", "quiet"], true);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let path = PathBuf::from(stdout.trim());
    assert!(path.is_dir(), "stdout was not a bare path: {stdout}");
    assert_eq!(stdout.trim().lines().count(), 1);
}
