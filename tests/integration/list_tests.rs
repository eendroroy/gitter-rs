#[test]
fn test_repo_listing_output() {
    define_gitter_command_test!(
        args: ["list", "-d", "3", "-a", "never"],
        expected: [
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
        ]
    );
}
#[test]
fn

    test_repo_listing_output_aligned() {
    define_gitter_command_test!(
        args: ["list", "-d", "3", "-f", "! name:gitter-rs"],
        expected: [
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
        ]
    );
}

#[test]
fn     test_repo_listing_filtered_output() {
    define_gitter_command_test!(
        args: ["list", "-d", "3", "-a", "never", "-f", "branch:master"],
        expected: [
            r"^\.local/repo_00 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_02 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/repo_bare_00 bare on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
            r"^\.local/ign_10/repo_11 on master \[[0-9a-f]*\] by indrajit \d+ (s |mi|h |d |mo|y )\s*$",
        ]
    );
}

#[test]
fn test_repo_listing_sorted_output() {
    define_gitter_command_test!(
        args: ["list", "-d", "3", "-a", "never", "-s", "{_branch:n_}{_name_}"],
        expected: [
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
        ]
    );
}
