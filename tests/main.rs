use chrono::{Duration, Utc};
use ctor::ctor;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

#[macro_export]
macro_rules! define_gitter_test {
    // Arm 1: Standard tests with only stdout regex checks
    (
        $name:ident,
        args: [$($arg:expr),* $(,)?],
        expected: [$($pattern:expr),* $(,)?] $(,)?
    ) => {
        define_gitter_test!(
            @impl $name,
            args: [$($arg),*],
            expected: [$($pattern),*],
            expect_empty_stdout: false,
            stderr_contains: None::<&str> // <-- Added explicit type annotation here
        );
    };

    // Arm 2: Tests with a stderr check that still look for stdout patterns
    (
        $name:ident,
        args: [$($arg:expr),* $(,)?],
        expected: [$($pattern:expr),* $(,)?],
        stderr_contains: $stderr_pat:expr $(,)?
    ) => {
        define_gitter_test!(
            @impl $name,
            args: [$($arg),*],
            expected: [$($pattern),*],
            expect_empty_stdout: false,
            stderr_contains: Some($stderr_pat)
        );
    };

    // Arm 3: Failure path tests where stdout must be completely blank
    (
        $name:ident,
        args: [$($arg:expr),* $(,)?],
        expected: [$($pattern:expr),* $(,)?],
        expect_empty_stdout: $empty_stdout:expr,
        stderr_contains: $stderr_pat:expr $(,)?
    ) => {
        define_gitter_test!(
            @impl $name,
            args: [$($arg),*],
            expected: [$($pattern),*],
            expect_empty_stdout: $empty_stdout,
            stderr_contains: Some($stderr_pat)
        );
    };

    // Main Internal Implementation Engine
    (
        @impl $name:ident,
        args: [$($arg:expr),* $(,)?],
        expected: [$($pattern:expr),* $(,)?],
        expect_empty_stdout: $empty_stdout:expr,
        stderr_contains: $stderr_pat:expr
    ) => {
        #[test]
        fn $name() {
            use std::process::Command;
            use assert_cmd::prelude::*;
            use regex::Regex;

            let output = Command::cargo_bin("gitter")
                .unwrap()
                .args(&[$($arg),*])
                .output()
                .unwrap();

            let stdout = String::from_utf8_lossy(&output.stdout);

            if $empty_stdout {
                assert!(stdout.is_empty(), "Expected empty stdout, but got:\n{}", stdout);
            } else {
                let lines: Vec<&str> = stdout
                    .lines()
                    .filter(|line| !line.contains("gitter-rs"))
                    .collect();

                let expected_patterns: Vec<&str> = vec![$($pattern),*];

                assert_eq!(
                    lines.len(),
                    expected_patterns.len(),
                    "Unexpected number of lines. Got {}, expected {}.\nActual output:\n{}",
                    lines.len(),
                    expected_patterns.len(),
                    stdout
                );

                for (line, pattern) in lines.iter().zip(expected_patterns.iter()) {
                    let re = Regex::new(pattern).unwrap();
                    assert!(
                        re.is_match(line),
                        "Line did not match:\n{}\nExpected pattern:\n{}",
                        line,
                        pattern
                    );
                }
            }

            if let Some(stderr_pat) = $stderr_pat {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(
                    stderr.contains(stderr_pat),
                    "Expected substring matching pattern in stderr, got:\n{}",
                    stderr
                );
            }
        }
    };
}

#[macro_export]
macro_rules! define_gitter_help_test {
    (
        $name:ident,
        args: [$($arg:expr),* $(,)?],
        contains: [$($substring:expr),* $(,)?] $(,)?
    ) => {
        #[test]
        fn $name() {
            use std::process::Command;
            use assert_cmd::prelude::*;
            use predicates::prelude::predicate::str::contains;

            let mut cmd = Command::cargo_bin("gitter").unwrap();
            let assert = cmd.args(&[$($arg),*]).assert();

            // Chain every string assertion dynamically on the asset reference
            $(
                let assert = assert.stdout(contains($substring));
            )*

            // Drop the reference to finalize evaluation
            let _ = assert;
        }
    };
}

static GLOBAL_INIT: Once = Once::new();

#[ctor(unsafe)]
fn global_test_setup() {
    GLOBAL_INIT.call_once(|| {
        println!("----- Creating git repositories for tests -----");
        run_provision();
    });
}

fn run_provision() {
    let local_dir = PathBuf::from(".local");
    fs::create_dir_all(&local_dir).expect("Failed to create .local directory");
    let base_path = fs::canonicalize(&local_dir).expect("Failed to get absolute path for .local");

    for i in 0..=11 {
        let days_ago = (i + 1) as i64;
        let time_stamp =
            (Utc::now() - Duration::days(days_ago)).format("%Y-%m-%dT%H:%M:%S").to_string();

        let repo_dir = base_path.join(format!("repo_{:02}", i));
        let bare_dir = base_path.join(format!("repo_bare_{:02}", i));

        let create_dir = |repo_dir: &Path| {
            if repo_dir.exists() {
                println!("{} already exists. Deleting....", repo_dir.display());
                fs::remove_dir_all(repo_dir).expect("Failed to delete existing repo_dir");
            }
            fs::create_dir_all(repo_dir).expect("Failed to create repo directory");
        };

        let git_init = |repo_dir: &Path| {
            Command::new("git")
                .args(["-C", repo_dir.to_str().unwrap(), "init", "-b", "master"])
                .output()
                .expect("Failed to run git init");
        };

        let make_first_commit = |repo_dir: &Path| {
            let file_path = repo_dir.join("file");
            let file_contents = format!("{}/file-1\n", repo_dir.display());
            fs::write(&file_path, file_contents)
                .expect(&format!("Failed to write file {}", &file_path.display()));

            Command::new("git")
                .args(["-C", repo_dir.to_str().unwrap(), "add", "."])
                .output()
                .expect("Failed git add");

            Command::new("git")
                .args(["-C", repo_dir.to_str().unwrap(), "commit", "-m", "first commit"])
                .env("GIT_AUTHOR_DATE", &time_stamp)
                .env("GIT_COMMITTER_DATE", &time_stamp)
                .output()
                .expect("Failed git commit");
        };

        let clone_bare_repo = |source: &Path, target: &Path| {
            if target.exists() {
                println!("{} already exists. Deleting....", target.display());
                fs::remove_dir_all(target).expect("Failed to delete existing bare_dir");
            }
            Command::new("git")
                .args(["clone", "--bare", source.to_str().unwrap(), target.to_str().unwrap()])
                .output()
                .expect("Failed to clone bare repo");
        };

        match i {
            0 => {
                create_dir(&repo_dir);
                git_init(&repo_dir);
                make_first_commit(&repo_dir);
                clone_bare_repo(&repo_dir, &bare_dir);
            }
            1 | 2 => {
                create_dir(&repo_dir);
                git_init(&repo_dir);
                make_first_commit(&repo_dir);
            }
            3 | 4 | 5 => {
                create_dir(&repo_dir);
                git_init(&repo_dir);
                make_first_commit(&repo_dir);
                Command::new("git")
                    .args([
                        "-C",
                        repo_dir.to_str().unwrap(),
                        "checkout",
                        "-b",
                        &format!("feature/feature-{}", i),
                    ])
                    .output()
                    .expect("Failed to checkout feature branch");
            }
            6 => {
                create_dir(&repo_dir);
                git_init(&repo_dir);
                make_first_commit(&repo_dir);

                let file2_path = repo_dir.join("file-2");
                let file2_contents = format!("{}/file-2\n", repo_dir.display());
                fs::write(&file2_path, file2_contents).expect("Failed to write file-2");

                Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "add", "."])
                    .output()
                    .expect("Failed git add for file-2");

                Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "commit", "-m", "second commit"])
                    .env("GIT_AUTHOR_DATE", &time_stamp)
                    .env("GIT_COMMITTER_DATE", &time_stamp)
                    .output()
                    .expect("Failed git commit for second commit");

                let rev_output = Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "rev-list", "--max-parents=0", "HEAD"])
                    .output()
                    .expect("Failed to run git rev-list");

                let root_commit = String::from_utf8_lossy(&rev_output.stdout).trim().to_string();

                Command::new("git")
                    .args(["-C", repo_dir.to_str().unwrap(), "checkout", &root_commit])
                    .output()
                    .expect("Failed to checkout root commit");

                clone_bare_repo(&repo_dir, &bare_dir); // Cleaned up the duplicate clone execution
            }
            7 => {
                create_dir(&repo_dir);
                git_init(&repo_dir);
            }
            8 | 9 => {
                let repo_dir = base_path.join(format!("ign_8_9/repo_{:02}", i));
                create_dir(&repo_dir);
                git_init(&repo_dir);
                make_first_commit(&repo_dir);
            }
            10 | 11 => {
                let repo_dir = base_path.join(format!("ign_10/repo_{:02}", i));
                create_dir(&repo_dir);
                git_init(&repo_dir);
                make_first_commit(&repo_dir);
            }
            _ => {}
        }
    }

    let ignore_path = base_path.join(".gitterignore");

    if ignore_path.exists() {
        println!("{} already exists. Deleting....", ignore_path.display());
        fs::remove_file(&ignore_path).expect("Failed to delete existing .gitterignore file");
    }

    let ignore_contents = "repo_01\nign_8_9/*\nign_10/repo_10\n";
    fs::write(&ignore_path, ignore_contents).expect("Failed to write '.gitterignore'");
}

mod integration {
    pub mod exec_tests;
    pub mod filter_tests;
    pub mod gitter_tests;
    pub mod help_tests;
    pub mod list_tests;
}
