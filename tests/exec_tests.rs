use assert_cmd::Command;
use regex::Regex;

#[test]
fn test_repo_exec_echo_output() {
    let output = Command::cargo_bin("gitter")
        .unwrap()
        .args(&["exec", "-i", "never", "-c", "never", "echo", "\"{_name_} => {_commit:c_}\""])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|line| !line.contains("gitter-rs")).collect();

    let expected_patterns = vec![
        "repo_00 => 1",
        "repo_01 => 1",
        "repo_02 => 1",
        "repo_03 => 1",
        "repo_04 => 1",
        "repo_05 => 1",
        "repo_06 => 1",
        "repo_07 => 0",
        "repo_bare_00 => 1",
        "repo_bare_06 => 1",
    ];

    assert_eq!(lines.len(), expected_patterns.len(), "Unexpected number of lines");

    for (line, pattern) in lines.iter().zip(expected_patterns.iter()) {
        let re = Regex::new(pattern).unwrap();
        assert!(re.is_match(line), "Line did not match:\n{}\nExpected pattern:\n{}", line, pattern);
    }
}
#[test]
fn test_repo_exec_basename_output() {
    let output = Command::cargo_bin("gitter")
        .unwrap()
        .args(&["exec", "-i", "never", "-c", "never", "basename", "\"{_path:a_}{_name_}\""])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|line| !line.contains("gitter-rs")).collect();

    let expected_patterns = vec![
        "repo_00",
        "repo_01",
        "repo_02",
        "repo_03",
        "repo_04",
        "repo_05",
        "repo_06",
        "repo_07",
        "repo_bare_00",
        "repo_bare_06",
    ];

    assert_eq!(lines.len(), expected_patterns.len(), "Unexpected number of lines");

    for (line, pattern) in lines.iter().zip(expected_patterns.iter()) {
        let re = Regex::new(pattern).unwrap();
        assert!(re.is_match(line), "Line did not match:\n{}\nExpected pattern:\n{}", line, pattern);
    }
}
