#[test]
fn gitter_should_fail_on_invalid_directory() {
    gitter_test!(
        args: { "list", "-d", "3", "-C", "/non/existent/directory" }
        stdout: { }
        stderr: { r"ERR:  \(/non/existent/directory\) No such file or directory \(os error 2\)" }
    );
}
