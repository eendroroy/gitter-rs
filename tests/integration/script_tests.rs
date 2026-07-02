#[test]
fn test_repo_script_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
        }
        stdout: {
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
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_execution_quiet() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--quiet",
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .* # Modified In-Memory$",
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_zsh_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--zsh",
        }
        stdout: {
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
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_bash_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--bash",
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_07\s+on detached\s+\[\] by\s*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ bash .* # Modified In-Memory$",
            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$"
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_fish_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--fish"
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_07\s+on detached\s+\[\] by\s*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ fish .* # Modified In-Memory$",
            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$"
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_elvish_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--elvish",
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_07\s+on detached\s+\[\] by\s*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ elvish .* # Modified In-Memory$",
            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$"
        }
        stderr: { }
    );
}

#[cfg(not(windows))]
#[test]
fn test_repo_script_pwsh_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--power-shell"
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_07\s+on detached\s+\[\] by\s*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ pwsh .* # Modified In-Memory$",
            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$"
        }
        stderr: { }
    );
}

#[cfg(windows)]
#[test]
fn test_repo_script_powershell_execution_output() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--placeholder",
            "--path", "./scripts/script_test.sh",
            "--power-shell"
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_07\s+on detached\s+\[\] by\s*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_bare_00\s+bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ powershell .* # Modified In-Memory$",
            r"^\.local/repo_bare_06\s+bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$"
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_execution_output_without_placeholders() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--path", "./scripts/script_test.sh",
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
            r"^\{_path:r_\}\{_name_\} \{_language_\} \{_bare_\} on \{_branch:n_\} \[\{_hash:8_\}\] by \{_author:n_\} \{_time:r_\}",
        }
        stderr: { }
    );
}

#[test]
fn test_repo_script_execution_without_placeholders_quiet() {
    gitter_test!(
        args: {
            "script",
            "--filter", "! name:gitter-rs",
            "--path", "./scripts/script_test.sh",
            "--quiet",
        }
        stdout: {
            r"^\.local/repo_00\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_02\s+on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_03\s+on feature/feature-3\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_04\s+on feature/feature-4\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_05\s+on feature/feature-5\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_06\s+on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_07\s+on detached\s+\[\s*\] by\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_bare_00 bare on master\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",

            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*$",
        }
        stderr: { }
    );
}
