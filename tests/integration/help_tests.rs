#[test]
fn help_displays_command_usage() {
    gitter_test_partial!(
        args: { "help" }
        stdout: {
            "Usage: gitter [OPTIONS] <COMMAND>",
            "gitter [OPTIONS] <GIT_ARGS>...",
            "Commands:",
            "list", "[aliases: ls, l]",
            "git", "[alias: g]",
            "exec", "[alias: e]",
            "script", "[alias: s]",
            "bash", "[alias: b]",
            "completion",
            "help", "Show help topics",
            "meta",
            "Arguments:",
            "<GIT_ARGS>...",
            "Options:",
            "-h, --help",
            "-V, --version",
        }
        stderr: { }
    );
}

#[test]
fn help_lists_available_placeholders() {
    gitter_test_partial!(
        args: { "help", "placeholders" }
        stdout: {
            "{_remote:n_}",
            "{_remote:f_}",
            "{_remote:p_}",
            "{_name_}",
            "{_path:r_}",
            "{_path:a_}",
            "{_nesting_}",
            "{_branch:n_}",
            "{_branch:c_}",
            "{_hash:f_}",
            "{_hash:<n>_}",
            "{_commit:c_}",
            "{_author:e_}",
            "{_author:n_}",
            "{_time:r_}",
            "{_time:rc_}",
            "{_time:a_}",
            "{_dirty_}",
            "{_bare_}",
            "{_size_}",
            "{_language_}",
        }
        stderr: { }
    );
}

#[test]
fn help_describes_gitterignore_patterns() {
    gitter_test_partial!(
        args: { "help", "gitterignore" }
        stdout: {
            "Gitterignore File Format",
            "path/to/repo",
            "prefix*",
            "dir_name/*",
            "dir_prefix*/*",
        }
        stderr: { }
    );
}

#[test]
fn help_describes_filter_syntax() {
    gitter_test_partial!(
        args: { "help", "filters" }
        stdout: {
            "Description:",
            "General Syntax:",
            "<filter_clause> && <filter_clause>",
            "<filter_clause> || <filter_clause>",
            "! <filter_clause>",
            "(<expression>)",
            "Filter Clause Format:",
            "[!] <prefix>:<value_pattern>",
            "Prefixes:",
            "path",
            "name",
            "branch",
            "dirty",
            "bare",
            "language",
            "active",
            "Value Patterns:",
            "value",
            "value+",
            "+value",
            "+value+",
            "Active Filter Value Patterns:",
            "Duration units:",
            "\x1b[1m\x1b[34my\x1b[0m (years)",
            "\x1b[1m\x1b[34mmo\x1b[0m (months)",
            "\x1b[1m\x1b[34md\x1b[0m (days)",
            "\x1b[1m\x1b[34mh\x1b[0m (hours)",
            "\x1b[1m\x1b[34mm\x1b[0m (minutes)",
            "\x1b[1m\x1b[34ms\x1b[0m (seconds).",
            "<duration",
            ">duration",
            "duration",
            "Examples:",
            "Matches repositories",
        }
        stderr: { }
    );
}

#[test]
fn help_documents_shell_completion_setup() {
    gitter_test_partial!(
        args: { "help", "completions" }
        stdout: {
            "Quick Setup Commands",
            "gitter completion bash",
            "gitter completion zsh",
            "gitter completion fish",
            "gitter completion elvish",
            "gitter completion powershell",
        }
        stderr: { }
    );
}
