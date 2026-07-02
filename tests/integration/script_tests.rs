#[test]
fn test_repo_script_execution_output() {
    define_gitter_command_test!(
        args: [
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
        ],
        expected: [
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_07\s+on detached\s+\[\] by\s*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$"
        ]
    );
}
