#[test]
fn completion() {
    gitter_test_present!(
        args: { "completion" }
        stdout: true
        stderr: false
    );
}

#[test]
fn completion_help() {
    gitter_test_partial!(
        args: { "completion", "--help" }
        stdout: {
            "Generate shell completion",
            "Usage: gitter completion [OPTIONS] [SHELL]",
            "Arguments:",
            "[SHELL]",
            "Shell to generate completion for",
            "Options:",
            "-h, --help",
            "Print help"
        }
        stderr: { }
    );
}

#[test]
fn completion_bash() {
    gitter_test_partial!(
        args: {"completion", "bash"}
        stdout: { "_gitter() {" }
        stderr: { }
    );
}

#[test]
fn completion_zsh() {
    gitter_test_partial!(
        args: { "completion", "zsh" }
        stdout: { "#compdef gitter" }
        stderr: { }
    );
}

#[test]
fn completion_fish() {
    gitter_test_partial!(
        args: { "completion", "fish" }
        stdout: { "__fish_gitter_global_optspecs" }
        stderr: { }
    );
}

#[test]
fn completion_elvish() {
    gitter_test_partial!(
        args: { "completion", "elvish" }
        stdout: { "edit:completion:arg-completer[gitter]" }
        stderr: { }
    );
}

#[test]
fn completion_powershell() {
    gitter_test_partial!(
        args: {"completion", "powershell" }
        stdout: { "Register-ArgumentCompleter -Native -CommandName 'gitter'" }
        stderr: { }
    );
}
