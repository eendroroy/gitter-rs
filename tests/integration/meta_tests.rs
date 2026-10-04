use assert_cmd::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Isolated workspace with one local "remote" repository to clone from.
struct Sandbox {
    root: PathBuf,
    ws: PathBuf,
    remote: String,
}

impl Sandbox {
    fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("gitter-meta-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let ws = root.join("ws");
        let remote = root.join("remote");
        fs::create_dir_all(&ws).unwrap();
        fs::create_dir_all(&remote).unwrap();
        let git = |dir: &Path, args: &[&str]| {
            let status = Command::new("git")
                .args(["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false"])
                .args(args)
                .current_dir(dir)
                .envs(crate::GIT_TEST_ENV)
                .output()
                .unwrap();
            assert!(status.status.success(), "{:?}", status);
        };
        git(&remote, &["init", "-q", "-b", "main"]);
        git(&remote, &["commit", "-q", "--allow-empty", "-m", "init"]);
        git(&remote, &["branch", "dev"]);
        Sandbox {
            root,
            ws,
            remote: remote.to_string_lossy().to_string(),
        }
    }

    fn gitter(&self, args: &[&str]) -> Output {
        Command::cargo_bin("gitter")
            .unwrap()
            .envs(crate::GIT_TEST_ENV)
            .args(["meta"])
            .args(args)
            .current_dir(&self.ws)
            .output()
            .unwrap()
    }

    fn git(&self, args: &[&str]) -> Output {
        Command::new("git")
            .args(args)
            .current_dir(&self.ws)
            .envs(crate::GIT_TEST_ENV)
            .output()
            .unwrap()
    }

    fn metafile(&self) -> String {
        fs::read_to_string(self.ws.join(".gitter.meta.toml")).unwrap()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn out(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).to_string()
}

fn err(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).to_string()
}

#[test]
fn meta_requires_a_subcommand() {
    let sb = Sandbox::new("nosub");
    let o = sb.gitter(&[]);
    assert!(!o.status.success());
    assert!(err(&o).contains("Usage: gitter meta [OPTIONS] <COMMAND>"));
}

#[test]
fn meta_init_creates_and_refuses_overwrite() {
    let sb = Sandbox::new("init");
    assert!(sb.gitter(&["init"]).status.success());
    assert!(sb.metafile().contains("version = 1"));

    let o = sb.gitter(&["init"]);
    assert!(!o.status.success());
    assert!(err(&o).contains("already exists"));
    assert!(sb.gitter(&["init", "--force"]).status.success());
}

#[test]
fn meta_add_creates_file_and_detects_duplicates() {
    let sb = Sandbox::new("add");
    let o = sb.gitter(&["add", &sb.remote, "-p", "libs", "-N", "lib", "-b", "dev"]);
    assert!(o.status.success(), "{}", err(&o));
    let file = sb.metafile();
    assert!(file.contains("path = \"libs/lib\""));
    assert!(file.contains("branch = \"dev\""));

    let o = sb.gitter(&["add", &sb.remote, "-p", "libs", "-N", "lib"]);
    assert!(!o.status.success());
    assert!(err(&o).contains("already exists in the metafile"));
}

#[test]
fn meta_add_rejects_paths_outside_the_workspace() {
    let sb = Sandbox::new("escape");
    let o = sb.gitter(&["add", &sb.remote, "-p", "../outside", "-N", "repo"]);

    assert!(!o.status.success());
    assert!(err(&o).contains("must stay inside the workspace"));
    assert!(!sb.ws.join(".gitter.meta.toml").exists());
}

#[test]
fn meta_add_dry_run_writes_nothing() {
    let sb = Sandbox::new("adddry");
    let o = sb.gitter(&["add", &sb.remote, "--dry-run", "--clone"]);
    assert!(o.status.success());
    assert!(out(&o).contains("git"));
    assert!(!sb.ws.join(".gitter.meta.toml").exists());
    assert!(!sb.ws.join("remote").exists());
}

#[test]
fn meta_add_reports_custom_file_write_failures() {
    let sb = Sandbox::new("write-fail");
    let file = "missing/blocked.toml";

    let o = sb.gitter(&["add", &sb.remote, "-N", "lib", "--file", file]);

    assert!(!o.status.success());
    assert!(err(&o).contains("Unable to write metafile"));
    assert!(err(&o).contains(file));
}

#[test]
fn meta_add_clone_checks_out_branch() {
    let sb = Sandbox::new("addclone");
    let o = sb.gitter(&["add", &sb.remote, "-N", "r", "-b", "dev", "--clone"]);
    assert!(o.status.success(), "{}", err(&o));
    let head = fs::read_to_string(sb.ws.join("r/.git/HEAD")).unwrap();
    assert!(head.contains("refs/heads/dev"));
}

#[test]
fn meta_add_reports_git_checkout_failures() {
    let sb = Sandbox::new("add-fail");
    let o = sb.gitter(&["add", &sb.remote, "-N", "lib", "-b", "missing", "--clone"]);

    assert!(!o.status.success());
    assert!(err(&o).contains("pathspec"));
    assert!(sb.ws.join("lib/.git").exists());
    assert!(sb.metafile().contains("path = \"lib\""));
}

#[test]
fn meta_remove_by_name_and_path() {
    let sb = Sandbox::new("remove");
    sb.gitter(&["add", &sb.remote, "-N", "one"]);
    sb.gitter(&["add", &sb.remote, "-p", "sub", "-N", "two"]);

    let o = sb.gitter(&["remove", "one", "--dry-run"]);
    assert!(o.status.success());
    assert!(sb.metafile().contains("path = \"one\""));

    assert!(sb.gitter(&["rm", "one", "sub/two"]).status.success());
    assert!(!sb.metafile().contains("path ="));

    let o = sb.gitter(&["rm", "ghost"]);
    assert!(!o.status.success());
    assert!(err(&o).contains("not in the metafile"));
}

#[test]
fn meta_remove_rejects_ambiguous_names() {
    let sb = Sandbox::new("ambiguous");
    sb.gitter(&["add", &sb.remote, "-p", "one", "-N", "same"]);
    sb.gitter(&["add", &sb.remote, "-p", "two", "-N", "same"]);

    let o = sb.gitter(&["remove", "same"]);
    assert!(!o.status.success());
    assert!(err(&o).contains("ambiguous, use the full path"));
}

#[test]
fn meta_list_reports_missing_file_and_lists_recorded_repositories() {
    let sb = Sandbox::new("list");
    let o = sb.gitter(&["list"]);
    assert!(!o.status.success());
    assert!(err(&o).contains("Unable to read metafile"));

    sb.gitter(&["add", &sb.remote, "-N", "one"]);
    let o = sb.gitter(&["ls"]);
    assert!(o.status.success());
    assert!(out(&o).contains("one"));
}

#[test]
fn meta_list_and_restore_report_empty_metafiles() {
    let sb = Sandbox::new("empty");
    assert!(sb.gitter(&["init"]).status.success());

    let listed = sb.gitter(&["list"]);
    assert!(listed.status.success());
    assert!(err(&listed).contains("No repositories recorded"));

    let restored = sb.gitter(&["restore"]);
    assert!(restored.status.success());
    assert!(out(&restored).contains("No repositories recorded"));
}

#[test]
fn meta_rejects_a_metafile_from_a_newer_version() {
    let sb = Sandbox::new("future-version");
    fs::write(sb.ws.join(".gitter.meta.toml"), "version = 99\nrepos = []\n").unwrap();

    let o = sb.gitter(&["list"]);
    assert!(!o.status.success());
    assert!(err(&o).contains("supports up to 1"));
}

#[test]
fn meta_corrupt_file_is_an_error_not_empty() {
    let sb = Sandbox::new("corrupt");
    fs::write(sb.ws.join(".gitter.meta.toml"), "garbage").unwrap();
    for args in [&["save", "--merge"][..], &["add", "u", "-N", "x"][..], &["restore"][..]] {
        let o = sb.gitter(args);
        assert!(!o.status.success());
        assert!(err(&o).contains("Invalid metafile"));
    }
    assert_eq!(sb.metafile(), "garbage");
}

#[test]
fn meta_reads_legacy_layout() {
    let sb = Sandbox::new("legacy");
    fs::write(
        sb.ws.join(".gitter.meta.toml"),
        "[[repos]]\npath = \"./libs/\"\nname = \"old\"\nurl = \"u\"\n",
    )
    .unwrap();
    let o = sb.gitter(&["list"]);
    assert!(o.status.success());
    assert!(out(&o).contains("libs/") && out(&o).contains("old"));
}

#[test]
fn meta_restore_changes_repository_status_from_missing_to_ok() {
    let sb = Sandbox::new("restore");
    sb.gitter(&["add", &sb.remote, "-p", "libs", "-N", "lib", "-b", "dev"]);

    let o = sb.gitter(&["status"]);
    assert!(!o.status.success());
    assert!(out(&o).contains("missing"));

    let o = sb.gitter(&["restore", "--dry-run"]);
    assert!(o.status.success());
    assert!(!sb.ws.join("libs/lib").exists());

    let o = sb.gitter(&["restore", "-j", "2"]);
    assert!(o.status.success(), "{}", err(&o));
    assert!(sb.ws.join("libs/lib/.git").exists());

    let o = sb.gitter(&["status"]);
    assert!(o.status.success(), "{}", out(&o));
    assert!(out(&o).contains("ok"));

    let o = sb.gitter(&["restore"]);
    assert!(o.status.success());
    assert!(out(&o).contains("already cloned"));
}

#[test]
fn meta_restore_reports_conflicting_directory() {
    let sb = Sandbox::new("conflict");
    sb.gitter(&["add", &sb.remote, "-N", "lib"]);
    fs::create_dir_all(sb.ws.join("lib")).unwrap();
    fs::write(sb.ws.join("lib/file"), "x").unwrap();

    let o = sb.gitter(&["restore"]);
    assert!(!o.status.success());
    assert!(out(&o).contains("not empty"));
    assert!(err(&o).contains("1 of 1 repositories failed"));
}

#[test]
fn meta_restore_reports_git_failures() {
    let sb = Sandbox::new("restore-fail");
    let missing_remote = sb.root.join("missing-remote").to_string_lossy().to_string();
    sb.gitter(&["add", &missing_remote, "-N", "lib"]);

    let o = sb.gitter(&["restore"]);
    assert!(!o.status.success());
    assert!(out(&o).contains("failed:"));
    assert!(err(&o).contains("1 of 1 repositories failed"));
}

#[test]
fn meta_restore_skips_a_repository_with_a_different_remote() {
    let sb = Sandbox::new("restore-remote");
    sb.gitter(&["add", &sb.remote, "-N", "lib"]);
    let clone = sb.git(&["clone", "-q", &sb.remote, "lib"]);
    assert!(clone.status.success(), "{}", String::from_utf8_lossy(&clone.stderr));
    let update_remote =
        sb.git(&["-C", "lib", "remote", "set-url", "origin", "https://other.invalid/repo.git"]);
    assert!(update_remote.status.success());

    let o = sb.gitter(&["restore"]);
    assert!(!o.status.success());
    assert!(out(&o).contains("different remote"));
    assert!(err(&o).contains("1 of 1 repositories failed"));
}

#[test]
fn meta_status_reports_wrong_branch_dirty_and_wrong_remote() {
    let sb = Sandbox::new("status-states");
    sb.gitter(&["add", &sb.remote, "-N", "lib", "-b", "dev"]);
    let clone = sb.git(&["clone", "-q", &sb.remote, "lib"]);
    assert!(clone.status.success(), "{}", String::from_utf8_lossy(&clone.stderr));

    let wrong_branch = sb.gitter(&["status"]);
    assert!(!wrong_branch.status.success());
    assert!(out(&wrong_branch).contains("wrong-branch"));

    let checkout = sb.git(&["-C", "lib", "checkout", "-q", "-b", "dev", "origin/dev"]);
    assert!(checkout.status.success(), "{}", String::from_utf8_lossy(&checkout.stderr));
    let clean = sb.gitter(&["status"]);
    assert!(clean.status.success(), "{}", out(&clean));
    assert!(out(&clean).contains("ok"));

    fs::write(sb.ws.join("lib/untracked"), "untracked").unwrap();
    let dirty = sb.gitter(&["status"]);
    assert!(dirty.status.success(), "{}", out(&dirty));
    assert!(out(&dirty).contains("dirty"));

    let update_remote =
        sb.git(&["-C", "lib", "remote", "set-url", "origin", "https://other.invalid/repo.git"]);
    assert!(update_remote.status.success());
    let wrong_remote = sb.gitter(&["status"]);
    assert!(!wrong_remote.status.success());
    assert!(out(&wrong_remote).contains("wrong-remote"));
}

#[test]
fn meta_save_merges_repositories_prunes_missing_entries_and_reports_untracked() {
    let sb = Sandbox::new("save");
    sb.gitter(&["add", &sb.remote, "-N", "lib", "--clone"]);
    sb.gitter(&["add", &sb.remote, "-N", "gone"]);
    // an on-disk repo that is not recorded yet
    Command::new("git")
        .args(["clone", "-q", &sb.remote, "extra"])
        .current_dir(&sb.ws)
        .output()
        .unwrap();

    let o = sb.gitter(&["status"]);
    assert!(out(&o).contains("untracked"));

    let o = sb.gitter(&["save", "--merge", "--prune", "--dry-run"]);
    assert!(o.status.success(), "{}", err(&o));
    assert!(sb.metafile().contains("gone"));

    assert!(sb.gitter(&["save", "--merge", "--prune"]).status.success());
    let file = sb.metafile();
    assert!(file.contains("path = \"extra\""));
    assert!(file.contains("path = \"lib\""));
    assert!(!file.contains("gone"));

    assert!(sb.gitter(&["save"]).status.success());
    assert!(sb.metafile().contains("path = \"extra\""));
}

#[test]
fn meta_commands_write_to_a_custom_metafile_path() {
    let sb = Sandbox::new("file");
    let o = sb.gitter(&["add", &sb.remote, "-N", "lib", "--file", "other.toml"]);
    assert!(o.status.success());
    assert!(sb.ws.join("other.toml").exists());
    assert!(!sb.ws.join(".gitter.meta.toml").exists());
}
