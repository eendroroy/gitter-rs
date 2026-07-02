#[test]
fn exec_echo_output() {
    gitter_test!(
        args:  {"exec", "-d", "3", "-i", "never", "-c", "never", "-f", "! name:gitter-rs", "echo", "{_name_} {_commit:c_}"}
        stdout: {
            "repo_00 1",
            "repo_02 1",
            "repo_03 1",
            "repo_04 1",
            "repo_05 1",
            "repo_06 1",
            "repo_07 0",
            "repo_bare_00 1",
            "repo_bare_06 1",
            "repo_11 1",
        }
        stderr: {}
    );
}

#[test]
fn exec_basename_output() {
    gitter_test!(
        args: { "exec", "-d", "3", "-i", "never", "-c", "never", "-f", "! name:gitter-rs", "basename", "{_path:a_}{_name_}" }
        stdout: {
            "repo_00",
            "repo_02",
            "repo_03",
            "repo_04",
            "repo_05",
            "repo_06",
            "repo_07",
            "repo_bare_00",
            "repo_bare_06",
            "repo_11",
        }
        stderr: {}
    );
}
