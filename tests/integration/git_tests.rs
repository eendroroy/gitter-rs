#[test]
fn git_head_output() {
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

#[test]
fn git_bare_output() {
    gitter_test!(
        args:  {"git", "-i", "never", "-f", "! name:gitter-rs", "rev-parse", "--is-inside-work-tree"}
        stdout: {
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "true",
            r"\$ git rev-parse --is-inside-work-tree", "false",
            r"\$ git rev-parse --is-inside-work-tree", "false",
        }
        stderr: { }
    );
}

#[test]
fn git_bare_quiet_output() {
    gitter_test!(
        args:  {"git", "-i", "never", "-q", "-f", "! name:gitter-rs", "rev-parse", "--is-inside-work-tree"}
        stdout: {
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
            r"\$ git rev-parse --is-inside-work-tree",
        }
        stderr: { }
    );
}

#[test]
fn silent_git_bare_quiet_output() {
    gitter_test!(
        args:  {"rev-parse", "--is-inside-work-tree"}
        stdout: {
            r"^\s*gitter-rs.*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^true\s*$",

            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^false\s*$",

            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit\s+\d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ git rev-parse --is-inside-work-tree\s*$",
            r"^false\s*$",
        }
        stderr: { }
    );
}
