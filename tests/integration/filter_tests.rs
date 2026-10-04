#[test]
fn filter_rejects_invalid_active_durations() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<48 && ! name:gitter-rs" }
        stdout: { }
        stderr: { "ERR:  Error parsing filter expression: Invalid filter clause: active:<48" }
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<48x && ! name:gitter-rs" }
        stdout: { }
        stderr: { "ERR:  Error parsing filter expression: Invalid filter clause: active:<48x" }
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:-48h && ! name:gitter-rs" }
        stdout: { }
        stderr: { "ERR:  Error parsing filter expression: Invalid filter clause: active:-48h" }
    );
}

#[test]
fn filter_selects_repositories_by_branch() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "branch:master && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: {}
    );
}

#[test]
fn filter_boolean_precedence_grouping_and_negation() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "name:repo_00 || name:repo_02 && branch:feature/feature-3" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$"
        }
        stderr: {}
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "(name:repo_00 || name:repo_02) && ! branch:feature/feature-3" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$"
        }
        stderr: {}
    );
}

#[test]
fn filter_path_name_and_suffix_patterns() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "path:.local/ign_10/" }
        stdout: {
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$"
        }
        stderr: {}
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "name:+00" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$"
        }
        stderr: {}
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "name:+bare+" }
        stdout: {
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$"
        }
        stderr: {}
    );
}

#[test]
fn filter_bare_and_active_greater_than_conditions() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "bare:" }
        stdout: {
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$"
        }
        stderr: {}
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:>100y" }
        stdout: {}
        stderr: { "WARN: repo_07 =>  Failed to parse timestamp" }
    );
}

#[test]
fn filter_rejects_malformed_boolean_expressions() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "name:repo_00 &&" }
        stdout: {}
        stderr: { "ERR:  Error parsing filter expression:" }
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "(name:repo_00 || name:repo_02" }
        stdout: {}
        stderr: { "ERR:  Error parsing filter expression:" }
    );
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "unknown:repo_00" }
        stdout: {}
        stderr: { "ERR:  Error parsing filter expression: Invalid filter clause: unknown:repo_00" }
    );
}

#[test]
fn filter_selects_commits_younger_than_one_year() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<1y && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_03 on feature/feature-3 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_04 on feature/feature-4 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_05 on feature/feature-5 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_06 on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { "WARN: repo_07 =>  Failed to parse timestamp" }
    );
}

#[test]
fn filter_selects_commits_younger_than_one_month() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<1mo && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_03 on feature/feature-3 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_04 on feature/feature-4 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_05 on feature/feature-5 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_06 on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { "WARN: repo_07 =>  Failed to parse timestamp" }
    );
}

#[test]
fn filter_selects_commits_younger_than_two_days() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<2d && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { "WARN: repo_07 =>  Failed to parse timestamp" }
    );
}

#[test]
fn filter_selects_commits_younger_than_forty_eight_hours() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<48h && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { "WARN: repo_07 =>  Failed to parse timestamp" }
    );
}
