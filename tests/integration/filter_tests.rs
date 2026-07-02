#[test]
fn test_filter_active_errors() {
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
fn test_filter_branch() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "branch:master" }
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
fn test_filter_active_1y() {
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
fn test_filter_active_1mo() {
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
fn test_filter_active_2d() {
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
fn test_filter_active_48h() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "active:<48h && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { "WARN: repo_07 =>  Failed to parse timestamp" }
    );
}
