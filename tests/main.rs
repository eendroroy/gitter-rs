use chrono::{Duration, Utc};
use ctor::ctor;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

/// Make git behave the same regardless of the developer's global config
/// (e.g. `safe.bareRepository=explicit` breaks commands inside bare repos).
pub const GIT_TEST_ENV: [(&str, &str); 3] = [
    ("GIT_CONFIG_COUNT", "1"),
    ("GIT_CONFIG_KEY_0", "safe.bareRepository"),
    ("GIT_CONFIG_VALUE_0", "all"),
];

#[macro_export]
macro_rules! gitter_test {
    (
        args: {$($arg:expr),* $(,)?}
        stdout: {$($out_pat:expr),* $(,)?}
        stderr: {$($err_pat:expr),* $(,)?}
    ) => {
        {
            use std::process::Command;
            use assert_cmd::prelude::*;
            use regex::Regex;

            let mut command = Command::cargo_bin("gitter").unwrap();
            command.envs($crate::GIT_TEST_ENV);
            command.args(&[$($arg),*]);


            let output = command.output().unwrap();

            let (stdout, stderr) =
                (String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

            let (out_patterns, err_patterns): (Vec<&str>, Vec<&str>) =
                (vec![$($out_pat),*], vec![$($err_pat),*]);

            let (out_lines, err_lines): (Vec<&str>, Vec<&str>) =
                (stdout.lines().collect(), stderr.lines().collect());

            assert_eq!(
                out_lines.len(),
                out_patterns.len(),
                "STDOUT: Unexpected number of lines. Got {}, expected {}.\nActual output:\n{}",
                out_lines.len(),
                out_patterns.len(),
                stdout
            );

            assert_eq!(
                err_lines.len(),
                err_patterns.len(),
                "STDERR: Unexpected number of lines. Got {}, expected {}.\nActual output:\n{}",
                err_lines.len(),
                err_patterns.len(),
                stdout
            );

            for (line, pattern) in out_lines.iter().zip(out_patterns.iter()) {
                let re = Regex::new(pattern).unwrap();
                assert!(
                    re.is_match(line),
                    "STDOUT: Line did not match:\n--{}--\nExpected pattern:\n--{}--",
                    line,
                    pattern
                );
            }

            for (line, pattern) in err_lines.iter().zip(err_patterns.iter()) {
                let re = Regex::new(pattern).unwrap();
                assert!(
                    re.is_match(line),
                    "STDERR: Line did not match:\n--{}--\nExpected pattern:\n--{}--",
                    line,
                    pattern
                );
            }
        }
    };
}

#[macro_export]
macro_rules! gitter_test_present {
    (
        args: {$($arg:expr),* $(,)?}
        stdout: true
        stderr: $err_sel:tt
    ) => {
        $crate::gitter_test_present!(@impl args: {$($arg),*} stdout_check: true, stderr_check: $err_sel);
    };
    (
        args: {$($arg:expr),* $(,)?}
        stdout: false
        stderr: $err_sel:tt
    ) => {
        $crate::gitter_test_present!(@impl args: {$($arg),*} stdout_check: false, stderr_check: $err_sel);
    };

    (@impl args: {$($arg:expr),*} stdout_check: $out_bool:expr, stderr_check: true) => {
        $crate::gitter_test_present!(@final args: {$($arg),*} out: $out_bool, err: true);
    };
    (@impl args: {$($arg:expr),*} stdout_check: $out_bool:expr, stderr_check: false) => {
        $crate::gitter_test_present!(@final args: {$($arg),*} out: $out_bool, err: false);
    };

    (@final args: {$($arg:expr),*} out: $out_val:expr, err: $err_val:expr) => {
        use std::process::Command;
        use assert_cmd::prelude::*;
        use predicates::prelude::*;

        let mut cmd = Command::cargo_bin("gitter").unwrap();
        cmd.envs($crate::GIT_TEST_ENV);
        let mut assert = cmd.args(&[$($arg),*]).assert();

        if $out_val {
            assert = assert.stdout(predicate::str::is_empty().not());
        } else {
            assert = assert.stdout(predicate::str::is_empty());
        }

        if $err_val {
            assert = assert.stderr(predicate::str::is_empty().not());
        } else {
            assert = assert.stderr(predicate::str::is_empty());
        }

        let _ = assert;
    };
}

#[macro_export]
macro_rules! gitter_test_partial {
    (
        args: {$($arg:expr),* $(,)?}
        stdout: {$($out_sub:expr),* $(,)?} $(,)?
        stderr: {$($err_sub:expr),* $(,)?} $(,)?
    ) => {
        use std::process::Command;
        use assert_cmd::prelude::*;
        use predicates::prelude::predicate::str::contains;

        let mut cmd = Command::cargo_bin("gitter").unwrap();
        cmd.envs($crate::GIT_TEST_ENV);
        let assert = cmd.args(&[$($arg),*]).assert();

        $( let assert = assert.stdout(contains($out_sub)); )*

        $( let assert = assert.stderr(contains($err_sub)); )*

        let _ = assert;
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
    pub mod bash_tests;
    pub mod completion_tests;
    pub mod exec_tests;
    pub mod filter_tests;
    pub mod git_tests;
    pub mod gitter_tests;
    pub mod help_tests;
    pub mod ignore_tests;
    pub mod list_tests;
    pub mod meta_tests;
    pub mod placeholders_tests;
    pub mod script_tests;
}
