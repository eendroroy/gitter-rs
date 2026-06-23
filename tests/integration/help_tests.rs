use assert_cmd::Command;
use predicates::str::contains;

#[rustfmt::skip]
#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("gitter").unwrap();
    cmd.args(&["help"])
        .assert()
        .stdout(contains("Usage: gitter [COMMAND] [OPTIONS] [-- <RAW_ARGS>...]"))
        .stdout(contains("Commands:"))
        .stdout(contains("list")).stdout(contains("[aliases: ls, l]"))
        .stdout(contains("git")).stdout(contains("[aliases: g]"))
        .stdout(contains("exec")).stdout(contains("[aliases: e]"))
        .stdout(contains("script")).stdout(contains("[aliases: s]"))
        .stdout(contains("bash")).stdout(contains("[aliases: b]"))
        .stdout(contains("completion")).stdout(contains("[aliases: c, comp]"))
        .stdout(contains("help")).stdout(contains("gitter help --help"))
        .stdout(contains("meta")).stdout(contains("[aliases: m]"))
        .stdout(contains("Arguments:"))
        .stdout(contains("[RAW_ARGS]..."))
        .stdout(contains("Options:"))
        .stdout(contains("-C, --pwd <DIRECTORY>"))
        .stdout(contains("-d, --max-depth <MAX_DEPTH>"))
        .stdout(contains("-t, --info-template <INFO_TEMPLATE>"))
        .stdout(contains("-f, --filter <FILTER>"))
        .stdout(contains("-a, --align"))
        .stdout(contains("-c, --show-command <SHOW_COMMAND>"))
        .stdout(contains("-i, --show-info <SHOW_INFO>"))
        .stdout(contains("-q, --quiet"))
        .stdout(contains("-s, --sort <SORT>"))
        .stdout(contains("-r, --reverse"))
        .stdout(contains("-h, --help"))
        .stdout(contains("-V, --version"))
        .stdout(contains(""));
}

#[test]
fn test_help_placeholder() {
    let mut cmd = Command::cargo_bin("gitter").unwrap();
    cmd.args(&["help", "placeholder"])
        .assert()
        .stdout(contains("{_remote:n_}"))
        .stdout(contains("{_remote:f_}"))
        .stdout(contains("{_remote:p_}"))
        .stdout(contains("{_name_}"))
        .stdout(contains("{_path:r_}"))
        .stdout(contains("{_path:a_}"))
        .stdout(contains("{_branch:n_}"))
        .stdout(contains("{_branch:c_}"))
        .stdout(contains("{_hash:f_}"))
        .stdout(contains("{_hash:<n>_}"))
        .stdout(contains("{_commit:c_}"))
        .stdout(contains("{_author:e_}"))
        .stdout(contains("{_author:n_}"))
        .stdout(contains("{_time:r_}"))
        .stdout(contains("{_time:a_}"))
        .stdout(contains("{_dirty_}"))
        .stdout(contains("{_bare_}"))
        .stdout(contains("{_size_}"))
        .stdout(contains("{_language_}"));
}

#[test]
fn test_help_gitterignore() {
    let mut cmd = Command::cargo_bin("gitter").unwrap();
    cmd.args(&["help", "gitterignore"])
        .assert()
        .stdout(contains("Gitterignore File Format"))
        .stdout(contains("path/to/repo"))
        .stdout(contains("prefix*"))
        .stdout(contains("dir_name/*"))
        .stdout(contains("dir_prefix*/*"));
}

#[test]
fn test_help_filter() {
    let mut cmd = Command::cargo_bin("gitter").unwrap();
    cmd.args(&["help", "filter"])
        .assert()
        .stdout(contains("Description:"))
        .stdout(contains("General Syntax:"))
        .stdout(contains("<filter_clause> && <filter_clause>"))
        .stdout(contains("<filter_clause> || <filter_clause>"))
        .stdout(contains("! <filter_clause>"))
        .stdout(contains("(<expression>)"))
        .stdout(contains("Filter Clause Format:"))
        .stdout(contains("[!] <prefix>:<value_pattern>"))
        .stdout(contains("Prefixes:"))
        .stdout(contains("path"))
        .stdout(contains("name"))
        .stdout(contains("branch"))
        .stdout(contains("dirty"))
        .stdout(contains("bare"))
        .stdout(contains("language"))
        .stdout(contains("active"))
        .stdout(contains("Value Patterns:"))
        .stdout(contains("value"))
        .stdout(contains("value+"))
        .stdout(contains("+value"))
        .stdout(contains("+value+"))
        .stdout(contains("Active Filter Value Patterns:"))
        .stdout(contains("<duration"))
        .stdout(contains(">duration"))
        .stdout(contains("duration"))
        .stdout(contains("Examples:"))
        .stdout(contains("Matches repositories"));
}

#[test]
fn test_help_completion() {
    let mut cmd = Command::cargo_bin("gitter").unwrap();
    cmd.args(&["help", "completion"])
        .assert()
        .stdout(contains("Quick Setup Commands"))
        .stdout(contains("gitter completion bash"))
        .stdout(contains("gitter completion zsh"))
        .stdout(contains("gitter completion fish"))
        .stdout(contains("gitter completion elvish"))
        .stdout(contains("gitter completion powershell"));
}
