#[test]
fn gitter_should_fail_on_invalid_directory() {
    define_gitter_command_test!(
        args: ["list", "-d", "3", "-C", "/non/existent/directory"],
        expected: [],
        expect_empty_stdout: true,
        stderr_contains: "ERR:  (/non/existent/directory) No such file or directory (os error 2)"
    );
}
