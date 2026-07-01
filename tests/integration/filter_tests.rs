use assert_cmd::Command;
use regex::Regex;

#[test]
fn test_filter_branch() {
    let output = Command::cargo_bin("gitter")
        .unwrap()
        .args(&["list", "-d", "3", "-a", "never", "-f", "branch:master"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|line| !line.contains("gitter-rs")).collect();

    let expected_patterns = vec![
        r"^\./\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
    ];

    assert_eq!(lines.len(), expected_patterns.len(), "Unexpected number of lines");

    for (line, pattern) in lines.iter().zip(expected_patterns.iter()) {
        let re = Regex::new(pattern).unwrap();
        assert!(re.is_match(line), "Line did not match:\n{}\nExpected pattern:\n{}", line, pattern);
    }
}

#[test]
fn test_filter_active() {
    let output = Command::cargo_bin("gitter")
        .unwrap()
        .args(&["list", "-d", "3", "-a", "never", "-f", "active:<1M"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let lines: Vec<&str> = stdout.lines().filter(|line| !line.contains("gitter-rs")).collect();

    let expected_patterns = vec![
        r"^\./\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_03 on feature/feature-3 \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_04 on feature/feature-4 \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_05 on feature/feature-5 \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_06 on detached \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
        r"^\./\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (minute|minutes|hours|days|months|years) ago\s*$",
    ];

    assert_eq!(lines.len(), expected_patterns.len(), "Unexpected number of lines");

    for (line, pattern) in lines.iter().zip(expected_patterns.iter()) {
        let re = Regex::new(pattern).unwrap();
        assert!(re.is_match(line), "Line did not match:\n{}\nExpected pattern:\n{}", line, pattern);
    }

    assert!(
        stderr.contains("WARN: repo_07 =>  Failed to parse timestamp"),
        "Expected WARN message in stderr, got:\n{}",
        stderr
    );
}
