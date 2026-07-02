define_gitter_test!(
    test_repo_exec_echo_output,
    args: [
        "exec",
        "-d",
        "3",
        "-i",
        "never",
        "-c",
        "never",
        "echo",
        "\"{_name_} => {_commit:c_}\"",
    ],
    expected: [
        "repo_00 => 1",
        "repo_02 => 1",
        "repo_03 => 1",
        "repo_04 => 1",
        "repo_05 => 1",
        "repo_06 => 1",
        "repo_07 => 0",
        "repo_bare_00 => 1",
        "repo_bare_06 => 1",
        "repo_11 => 1",
    ]
);

define_gitter_test!(
    test_repo_exec_basename_output,
    args: [
        "exec",
        "-d",
        "3",
        "-i",
        "never",
        "-c",
        "never",
        "basename",
        "\"{_path:a_}{_name_}\"",
    ],
    expected: [
        "repo_00",
        "repo_02",
        "repo_03",
        "repo_04",
        "repo_05",
        "repo_06",
        "repo_07",
        "repo_bare_00",
        "repo_bare_06",
        "repo_11",
    ]
);
