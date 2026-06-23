use assert_cmd::Command;

#[test]
fn gitter_should_fail_on_invalid_directory() {
    let output = Command::cargo_bin("gitter")
        .unwrap()
        .args(&["list", "-d", "3", "-C", "/non/existent/directory"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stdout.is_empty(), "{}", stdout);
    assert!(
        stderr.contains("ERR:  (/non/existent/directory) No such file or directory (os error 2)")
    );
}
