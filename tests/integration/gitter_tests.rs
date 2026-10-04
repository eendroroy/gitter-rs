#[test]
fn list_rejects_a_nonexistent_working_directory() {
    gitter_test!(
        args: { "list", "-d", "3", "-C", "/non/existent/directory" }
        stdout: { }
        stderr: { r"ERR:  \(/non/existent/directory\) No such file or directory \(os error 2\)" }
    );
}

#[test]
fn common_options_before_or_after_command() {
    use assert_cmd::prelude::*;
    use std::process::Command;

    let run = |args: &[&str]| {
        let out = Command::cargo_bin("gitter")
            .unwrap()
            .envs(crate::GIT_TEST_ENV)
            .args(args)
            .output()
            .unwrap();
        assert!(out.status.success(), "{:?}: {}", args, String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).to_string()
    };

    let before = run(&["-C", ".local", "-d", "3", "-a", "never", "-f", "name:repo_00", "list"]);
    let after = run(&["list", "-C", ".local", "-d", "3", "-a", "never", "-f", "name:repo_00"]);
    let mixed = run(&["-d", "3", "ls", "-C", ".local", "-a", "never", "-f", "name:repo_00"]);
    assert!(before.contains("repo_00"));
    assert_eq!(before, after);
    assert_eq!(before, mixed);

    let git_before =
        run(&["-C", ".local", "-f", "name:repo_00", "-i", "never", "git", "log", "-1"]);
    let git_after = run(&["git", "-C", ".local", "-f", "name:repo_00", "-i", "never", "log", "-1"]);
    let implicit = run(&["-C", ".local", "-f", "name:repo_00", "-i", "never", "log", "-1"]);
    assert_eq!(git_before, git_after);
    assert_eq!(git_before, implicit);
}

#[test]
fn bare_common_options_without_command_is_an_error() {
    gitter_test_partial!(
        args: { "-C", "." }
        stdout: { }
        stderr: { "a command or git arguments are required" }
    );
}
