#[test]
fn list_output() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_03 on feature/feature-3 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_04 on feature/feature-4 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_05 on feature/feature-5 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_06 on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_07 on detached \[.*\] by.*\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { }
    );
}
#[test]
fn list_output_aligned() {
    gitter_test!(
        args: { "list", "-d", "3", "-f", "! name:gitter-rs" }
        stdout: {
            r"^\.local/       repo_00           on master            \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_02           on master            \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_03           on feature/feature-3 \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_04           on feature/feature-4 \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_05           on feature/feature-5 \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_06           on detached          \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_07           on detached          \[.*\] by.*\s*$",
            r"^\.local/       repo_bare_00 bare on master            \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/       repo_bare_06 bare on detached          \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11           on master            \[[0-9a-f]*\] by indrajit \s*\d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { }
    );
}

#[test]
fn list_filtered_output() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-f", "branch:master && ! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { }
    );
}

#[test]
fn list_sorted_output() {
    gitter_test!(
        args: { "list", "-d", "3", "-a", "never", "-s", "{_branch:n_}{_name_}", "-f", "! name:gitter-rs" }
        stdout: {
            r"^\.local/repo_06 on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_07 on detached \[.*\] by.*\s*$",
            r"^\.local/repo_bare_06 bare on detached \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_03 on feature/feature-3 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_04 on feature/feature-4 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_05 on feature/feature-5 \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        }
        stderr: { }
    );
}
