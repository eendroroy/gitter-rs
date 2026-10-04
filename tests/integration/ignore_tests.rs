use assert_cmd::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

struct Workspace(PathBuf);

impl Workspace {
    fn new() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("gitter-ignore-{}-{id}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn add_repo(&self, relative: &str) {
        let path = self.0.join(relative);
        fs::create_dir_all(&path).unwrap();
        let output = Command::new("git")
            .args(["init", "-q", "-b", "main"])
            .current_dir(&path)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));

        let output = Command::new("git")
            .args([
                "-c",
                "user.name=test",
                "-c",
                "user.email=test@example.test",
                "commit",
                "-q",
                "--allow-empty",
                "-m",
                "init",
            ])
            .current_dir(&path)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    }

    fn add_old_large_repo(&self, relative: &str) {
        let path = self.0.join(relative);
        fs::create_dir_all(&path).unwrap();
        let init = Command::new("git")
            .args(["init", "-q", "-b", "main"])
            .current_dir(&path)
            .output()
            .unwrap();
        assert!(init.status.success(), "{}", String::from_utf8_lossy(&init.stderr));

        let mut seed = 0x1234_5678_u32;
        let contents: Vec<u8> = (0..2 * 1024 * 1024)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                seed as u8
            })
            .collect();
        fs::write(path.join("large.bin"), contents).unwrap();

        for args in [&["add", "large.bin"][..], &["commit", "-q", "-m", "old large fixture"][..]] {
            let mut command = Command::new("git");
            if args[0] == "commit" {
                command
                    .args(["-c", "user.name=test", "-c", "user.email=test@example.test"])
                    .env("GIT_AUTHOR_DATE", "2000-01-01T00:00:00Z")
                    .env("GIT_COMMITTER_DATE", "2000-01-01T00:00:00Z");
            }
            let output = command.args(args).current_dir(&path).output().unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    fn list(&self) -> String {
        let output = Command::cargo_bin("gitter")
            .unwrap()
            .envs(crate::GIT_TEST_ENV)
            .args(["list", "-d", "5", "-a", "never"])
            .current_dir(&self.0)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    fn list_template(&self, template: &str) -> String {
        let output = Command::cargo_bin("gitter")
            .unwrap()
            .envs(crate::GIT_TEST_ENV)
            .args(["list", "-d", "5", "-a", "never", "--info-template", template])
            .current_dir(&self.0)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn ignore_file_patterns_exclude_repositories_and_preserve_nonmatches() {
    let workspace = Workspace::new();
    for path in [
        "exact/repo",
        "prefix-one",
        "parent/repo",
        "parentish/repo",
        "parent-prefix-one/repo",
        "nested/skip-child/repo",
        "nested/keep-child/repo",
    ] {
        workspace.add_repo(path);
    }

    fs::write(
        workspace.0.join(".gitterignore"),
        "# exact path\n\nexact/repo\nprefix*\nparent/*\nparent-prefix*/*\n",
    )
    .unwrap();
    fs::write(workspace.0.join("nested/.gitterignore"), "skip*\n").unwrap();

    let output = workspace.list();
    for ignored in [
        "exact/repo",
        "prefix-one",
        "parent/repo",
        "parent-prefix-one/repo",
        "nested/skip-child/repo",
    ] {
        assert!(!output.contains(ignored), "unexpected ignored repository {ignored}:\n{output}");
    }
    for visible in ["parentish/repo", "nested/keep-child/repo"] {
        assert!(output.contains(visible), "expected repository {visible}:\n{output}");
    }
}

#[test]
fn list_placeholders_exercise_old_commit_time_and_large_repo_size() {
    let workspace = Workspace::new();
    workspace.add_old_large_repo("aged/repository");

    let output = workspace.list_template("{_time:r_}|{_time:rc_}|{_size_}");
    let line = output.trim();
    let fields: Vec<_> = line.split('|').collect();
    assert_eq!(fields.len(), 3, "{output}");
    assert!(fields[0].trim().ends_with('y'), "{output}");
    assert!(fields[1].contains('y') && fields[1].contains("mo"), "{output}");
    assert!(fields[2].trim().ends_with('M') || fields[2].trim().ends_with('G'), "{output}");
}
