use assert_cmd::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Workspace(PathBuf);

impl Workspace {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("gitter-report-{name}-{}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn add_repo(&self, name: &str, branch: &str, source: &str, dirty: bool) {
        self.add_repo_with_date(name, branch, source, dirty, None);
    }

    fn add_repo_with_date(
        &self,
        name: &str,
        branch: &str,
        source: &str,
        dirty: bool,
        date: Option<&str>,
    ) {
        let repo = self.0.join(name);
        fs::create_dir(&repo).unwrap();
        run_git(&repo, &["init", "-q", "-b", branch]);
        let contents = if source.ends_with(".py") {
            "print('report test')\n"
        } else {
            "fn main() { println!(\"report test\"); }\n"
        };
        fs::write(repo.join(source), contents).unwrap();
        run_git(&repo, &["add", "."]);
        let date_env: Vec<(&str, &str)> = date
            .map(|date| vec![("GIT_AUTHOR_DATE", date), ("GIT_COMMITTER_DATE", date)])
            .unwrap_or_default();
        run_git_with_env(
            &repo,
            &[
                "-c",
                "user.name=Report Test",
                "-c",
                "user.email=report@test.invalid",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-qm",
                "initial",
            ],
            &date_env,
        );
        if dirty {
            fs::write(repo.join(".untracked"), "dirty\n").unwrap();
        }
    }

    fn report(&self, command: &str, args: &[&str]) -> Output {
        Command::cargo_bin("gitter")
            .unwrap()
            .envs(crate::GIT_TEST_ENV)
            .args(["report", command, "-C"])
            .arg(&self.0)
            .args(args)
            .output()
            .unwrap()
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn run_git(directory: &Path, args: &[&str]) {
    run_git_with_env(directory, args, &[]);
}

fn run_git_with_env(directory: &Path, args: &[&str], env: &[(&str, &str)]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(directory)
        .envs(crate::GIT_TEST_ENV)
        .envs(env.iter().copied())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn metadata_report_lists_details_for_each_repository() {
    let workspace = Workspace::new("metadata");
    workspace.add_repo("repo-a", "main", "main.rs", true);
    workspace.add_repo("repo-b", "feature", "app.py", false);
    run_git(&workspace.0.join("repo-a"), &["remote", "add", "origin", "https://example.invalid/a"]);
    run_git(&workspace.0.join("repo-a"), &["tag", "v1"]);

    let output = workspace.report("metadata", &[]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let blocks: Vec<&str> = stdout.split("\n\n").collect();
    assert_eq!(blocks.len(), 2, "{stdout}");

    let squash = |block: &str| {
        block
            .lines()
            .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let (a, b) = (squash(blocks[0]), squash(blocks[1]));
    assert!(a.starts_with("repo-a"), "{a}");
    for expected in [
        "Remote: https://example.invalid/a",
        "Branch: main",
        "State: dirty",
        "Last author: Report Test report@test.invalid",
        "Commits: 1",
        "Branches: 1",
        "Tags: 1",
        "Contributors: 1",
        "Lines of code: 1",
        "Languages: Rust (1)",
    ] {
        assert!(a.contains(expected), "missing {expected:?} in:\n{a}");
    }
    assert!(a.contains("Last commit:"));

    assert!(b.starts_with("repo-b"), "{b}");
    for expected in
        ["Remote: unknown", "Branch: feature", "State: clean", "Tags: 0", "Languages: Python (1)"]
    {
        assert!(b.contains(expected), "missing {expected:?} in:\n{b}");
    }
}

#[test]
fn metadata_report_handles_an_empty_workspace() {
    let workspace = Workspace::new("metadata-empty");

    let output = workspace.report("metadata", &[]);

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn report_help_lists_metadata() {
    gitter_test_partial!(
        args: { "report", "--help" }
        stdout: {
            "Usage: gitter report [OPTIONS] <COMMAND>",
            "Commands:",
            "metadata", "Show detailed metadata for each repository",
        }
        stderr: { }
    );
}
