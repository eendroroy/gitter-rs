#[test]
fn gitter_completion() {
    gitter_test_present!(
        args: { "completion" }
        stdout: true
        stderr: false
    );
}

#[test]
fn gitter_completion_help() {
    gitter_test_partial!(
        args: { "completion", "--help" }
        stdout: {
            "(experimental, may not work)",
            "Usage: gitter completion [OPTIONS]",
            "Options:",
            "--bash",
            "Generate completion for bash",
            "--elvish",
            "Generate completion for elvish",
            "--fish",
            "Generate completion for fish",
            "--power-shell",
            "Generate completion for PowerShell",
            "--zsh",
            "Generate completion for zsh",
            "-h, --help",
            "Print help"
        }
        stderr: { }
    );
}

#[test]
fn gitter_completion_bash() {
    gitter_test_partial!(
        args: {"completion", "--bash"}
        stdout: { "_gitter() {" }
        stderr: { }
    );
}

#[test]
fn gitter_completion_zsh() {
    gitter_test_partial!(
        args: { "completion", "--zsh" }
        stdout: { "#compdef gitter" }
        stderr: { }
    );
}

#[test]
fn gitter_completion_fish() {
    gitter_test_partial!(
        args: { "completion", "--fish" }
        stdout: { "__fish_gitter_global_optspecs" }
        stderr: { }
    );
}

#[test]
fn gitter_completion_elvish() {
    gitter_test_partial!(
        args: { "completion", "--elvish" }
        stdout: { "edit:completion:arg-completer[gitter]" }
        stderr: { }
    );
}

#[test]
fn gitter_completion_powershell() {
    gitter_test_partial!(
        args: {"completion", "--power-shell" }
        stdout: { "Register-ArgumentCompleter -Native -CommandName 'gitter'" }
        stderr: { }
    );
}
