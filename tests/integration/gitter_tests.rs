define_gitter_test!(
    gitter_should_fail_on_invalid_directory,
    args: ["list", "-d", "3", "-C", "/non/existent/directory"],
    expected: [],
    expect_empty_stdout: true,
    stderr_contains: "ERR:  (/non/existent/directory) No such file or directory (os error 2)"
);
