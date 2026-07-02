define_gitter_help_test!(
    test_help,
    args: ["help"],
    contains: [
        "Usage: gitter [COMMAND] [OPTIONS] [-- <RAW_ARGS>...]",
        "Commands:",
        "list", "[aliases: ls, l]",
        "git", "[aliases: g]",
        "exec", "[aliases: e]",
        "script", "[aliases: s]",
        "bash", "[aliases: b]",
        "completion",
        "help", "gitter help --help",
        "meta",
        "Arguments:",
        "[RAW_ARGS]...",
        "Options:",
        "-h, --help",
        "-V, --version",
    ]
);

define_gitter_help_test!(
    test_help_placeholder,
    args: ["help", "--placeholders"],
    contains: [
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
        "{_time:a_}",
        "{_dirty_}",
        "{_bare_}",
        "{_size_}",
        "{_language_}",
    ]
);

define_gitter_help_test!(
    test_help_gitterignore,
    args: ["help", "--gitterignore"],
    contains: [
        "Gitterignore File Format",
        "path/to/repo",
        "prefix*",
        "dir_name/*",
        "dir_prefix*/*",
    ]
);

define_gitter_help_test!(
    test_help_filter,
    args: ["help", "--filters"],
    contains: [
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
        "<duration",
        ">duration",
        "duration",
        "Examples:",
        "Matches repositories",
    ]
);

define_gitter_help_test!(
    test_help_completion,
    args: ["help", "--completions"],
    contains: [
        "Quick Setup Commands",
        "gitter completion --bash",
        "gitter completion --zsh",
        "gitter completion --fish",
        "gitter completion --elvish",
        "gitter completion --powershell",
    ]
);
