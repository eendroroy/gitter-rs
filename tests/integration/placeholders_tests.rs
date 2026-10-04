#[test]
fn script_prints_all_repository_placeholders() {
    gitter_test!(
        args: {
            "script",
            "--filter", "name:repo_bare_06",
            "--placeholder",
            "./scripts/print_placeholders.sh",
        }
        stdout: {
            r"^\.local/repo_bare_06 bare on detached\s+\[[0-9a-f]{8}\] by indrajit \d+ (s|mi|h|d|mo|y)\s*$",
            r"^\$ zsh .*/scripts/print_placeholders.sh # Modified In-Memory",
            r"\{ _remote:n_ \} -- origin",
            r"\{ _remote:f_ \} -- .*/\.local/repo_06",
            r"\{ _remote:p_ \} -- .*/\.local/repo_06",
            r"\{ _name_ \}     -- repo_bare_06",
            r"\{ _path:r_ \}   -- .local/",
            r"\{ _path:a_ \}   -- .*/\.local/",
            r"\{ _nesting_ \}  -- 1",
            r"\{ _branch:n_ \} -- detached",
            r"\{ _branch:c_ \} -- 1",
            r"\{ _hash:f_ \}   -- [0-9a-f]{40}",
            r"\{ _hash:8_ \}   -- [0-9a-f]{8}",
            r"\{ _hash:80_ \}  -- [0-9a-f]{40}",
            r"\{ _commit:c_ \} -- 1",
            r"\{ _author:e_ \} -- .*",
            r"\{ _author:n_ \} -- .*",
            r"\{ _time:r_ \}   -- 7 d",
            r"\{ _time:rc_ \}  -- 7 d",
            r"\{ _time:a_ \}   -- \d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}",
            r"\{ _dirty_ \}    --",
            r"\{ _bare_ \}     -- bare",
            r"\{ _size_ \}     -- \d+K\s*",
            r"\{ _language_ \} --",
        }
        stderr: { }
    );
}

#[test]
fn list_renders_all_repository_placeholders() {
    gitter_test!(
        args: {
            "list",
            "--filter", "name:repo_bare_06",
            "--info-template",
            "____{_remote:n_}____{_remote:f_}____{_remote:p_}____{_name_}\
            ____{_path:r_}____{_path:a_}____{_nesting_}____{_branch:n_}\
            ____{_branch:c_}____{_hash:f_}____{_hash:8_}____{_hash:80_}____\
            {_commit:c_}____{_author:e_}____{_author:n_}____{_time:r_}____\
            {_time:a_}____{_dirty_}____{_bare_}____{_size_}____{_language_}____",
        }
        stdout: {
            concat!(
                r"^____origin____.*/\.local/repo_06____.*/\.local/repo_06____",
                r"repo_bare_06____\.local/____.*/\.local/____1____detached",
                r"____1____[0-9a-f]{40}____[0-9a-f]{8}____[0-9a-f]{40}____1____",
                r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}____",
                r"indrajit____7 d ____\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}",
                r"________bare____\d+K________$"
            ),
        }
        stderr: { }
    );
}
