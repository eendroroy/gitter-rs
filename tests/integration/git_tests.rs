#[test]
fn git_output() {
    gitter_test!(
        args:  {"git", "-i", "never", "-c", "never", "-f", "! name:gitter-rs", "rev-parse", "--abbrev-ref", "HEAD"}
        stdout: {
            "master",
            "master",
            "feature/feature-3",
            "feature/feature-4",
            "feature/feature-5",
            "HEAD",
            "HEAD",
            "master",
            "HEAD",
        }
        stderr: {
            "fatal: ambiguous argument 'HEAD': unknown revision or path not in the working tree.",
            "Use '--' to separate paths from revisions, like this:",
            r"'git <command> \[<revision>...\] -- \[<file>...\]'",
        }
    );
}
