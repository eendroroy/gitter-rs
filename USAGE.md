# Usage

`gitter` discovers Git repositories beneath a workspace directory and runs an operation in each one. Unless overridden with `--pwd`, the current directory is the workspace.

```text
gitter [OPTIONS] <COMMAND>
gitter [OPTIONS] <GIT_ARGS>...
```

Run `gitter --help` or `gitter <COMMAND> --help` for command-line help.

## Common options

These options can be used with repository commands and may appear before or after the command name.

| Option | Explanation |
| --- | --- |
| `-C`, `--pwd <DIRECTORY>` | Search this workspace directory instead of the current directory. |
| `-d`, `--max-depth <N>` | Maximum directory depth to scan. Defaults to `2`. |
| `-f`, `--filter <EXPRESSION>` | Include only repositories matching a filter expression. |
| `-t`, `--info-template <TEMPLATE>` | Set the repository information line. Supports placeholders. |
| `-s`, `--sort <TEMPLATE>` | Sort by evaluated template. Defaults to nesting, relative path, then name. |
| `-r`, `--reverse` | Reverse the selected sort order. |
| `-a`, `--align <WHEN>` | Align output columns: `always` or `never`. Defaults to `always`. |
| `-c`, `--show-command <WHEN>` | Show or hide the executed command: `always` or `never`. |
| `-i`, `--show-info <WHEN>` | Show or hide each repository's information line: `always` or `never`. |
| `-q`, `--quiet` | Hide the child command's standard output. |

`-q` does not hide standard error or the command and repository information lines; use `--show-command never` and/or `--show-info never` to hide those separately.

## Commands

### List repositories

```sh
gitter list
gitter ls
gitter l
```

Lists discovered repositories and their information. The aliases are `ls` and `l`.

```sh
gitter list --pwd ~/work --max-depth 4
gitter list --filter 'branch:main && ! dirty:'
gitter list --sort '{_name_}' --reverse
gitter list --info-template '{_path:r_}{_name_} ({_branch:n_})'
gitter list --align never
```

### Run Git commands

The `git` command runs Git once in each discovered repository. It has the alias `g`.

```sh
gitter git status --short
gitter g log -1 --oneline
gitter -- status --short
gitter -f 'branch:main' git pull
```

Git arguments can also be provided without a subcommand; they are treated as `git` arguments:

```sh
gitter status --short
gitter -C ~/work -d 4 log -1
```

Use `--` when a Git option conflicts with a `gitter` option or should be passed literally:

```sh
gitter -- -c color.ui=never log -1
```

### Run an arbitrary program

`exec` runs a program in each repository. The alias is `e`. Placeholders may be used in the program arguments.

```sh
gitter exec cargo test
gitter e -q basename '{_path:a_}'
gitter exec --filter 'name:service+' cargo test --workspace
```

### Evaluate a Bash snippet

`bash` evaluates a command string using `bash -c` in each repository. The alias is `b`. Use `script` for larger scripts or other shells.

```sh
gitter bash 'git status --short'
gitter b 'printf "%s\n" "{_name_}"'
gitter bash 'git status --short | head -5'
```

### Run a script file

`script` runs a script file in each repository using the selected shell. The alias is `s`. By default, the shell is taken from `$SHELL`, falling back to Bash. Supported shells are `bash`, `elvish`, `fish`, `powershell`, and `zsh`.

```sh
gitter script ./maintenance.sh
gitter s ./maintenance.sh --shell zsh
gitter script ./show-repo.sh --placeholder
gitter script ./show-repo.sh --shell fish --placeholder --quiet
```

Use `-P` or `--placeholder` to evaluate placeholders in the script contents before running the script. Without it, the script file is run as-is.

### Manage the workspace metafile

`meta` commands maintain `.gitter.meta.toml` in the workspace. Use `--file <FILE>` on a subcommand to select a different metafile. `-C`/`--pwd` selects the workspace.

```sh
gitter meta init [--force] [--dry-run] [--file FILE]
gitter meta add <URL> [-p|--path DIRECTORY] [-N|--name NAME] [-b|--branch BRANCH] [--clone] [--dry-run] [--file FILE]
gitter meta remove <REPO>... [--dry-run] [--file FILE]
gitter meta save [--merge] [--prune] [--dry-run] [--file FILE]
gitter meta restore [--no-checkout] [-j|--jobs N] [--dry-run] [--file FILE]
gitter meta status [--file FILE]
gitter meta list [--file FILE]
```

- `init` creates an empty metafile. `--force` overwrites an existing file; `--dry-run` prints the target without writing.
- `add` records a remote URL. `--path` sets its parent directory within the workspace; `--name` sets the repository directory name (otherwise inferred from the URL). `--branch` records a branch, and `--clone` clones immediately and checks out that branch when provided.
- `remove` forgets one or more recorded repositories by path or unambiguous name; it does not delete directories from disk. Alias: `rm`.
- `save` records repositories found in the workspace. `--merge` keeps and updates existing records; `--prune` (requires `--merge`) removes entries whose directories no longer exist.
- `restore` clones recorded repositories. `--jobs` sets parallel clone concurrency (default `4`, minimum `1`); `--no-checkout` skips checking out recorded branches.
- `status` compares recorded repositories with the workspace and reports missing, dirty, wrong-remote, wrong-branch, and untracked repositories.
- `list` prints recorded repositories. Alias: `ls`.
- `--dry-run` previews changes without writing or cloning, where supported.

Examples:

```sh
gitter meta init
gitter meta add https://example.com/team/api.git --branch main --clone
gitter meta add https://example.com/team/web.git --path apps --name web
gitter meta remove apps/web
gitter meta save --merge --prune
gitter meta restore --jobs 8 --dry-run
gitter meta status
gitter meta list --file workspace-repos.toml
```

### Generate shell completions

```sh
gitter completion [SHELL]
```

Generates completion definitions for `bash`, `elvish`, `fish`, `powershell`, or `zsh`. If no shell is specified, `gitter` uses `$SHELL` and falls back to Bash.

```sh
gitter completion zsh > ~/.zfunc/_gitter
gitter completion bash > ~/.local/share/bash-completion/completions/gitter
```

### Show help topics

```sh
gitter help
gitter help placeholders
gitter help gitterignore
gitter help filters
gitter help completions
```

The topics describe placeholders, `.gitterignore` files, filter expressions, and shell completion setup.

## Filter expressions

Use `-f`/`--filter` with one or more clauses. Combine clauses with `&&` (AND), `||` (OR), `!` (NOT), and parentheses.

```sh
gitter list -f 'name:api'
gitter list -f 'path:services+ && (branch:main || branch:release+)'
gitter list -f '!(name:test+ || name:temp+)'
```

Supported clause prefixes are `path`, `name`, `branch`, `language`, `dirty`, `bare`, and `active`. For text values:

| Pattern | Match |
| --- | --- |
| `value` | Exact match |
| `value+` | Starts with `value` |
| `+value` | Ends with `value` |
| `+value+` | Contains `value` |

`dirty:` and `bare:` test whether a repository has uncommitted changes or is bare; the text after the colon is ignored. `active:` compares the age of the last commit. Valid duration units are `y`, `mo`, `d`, `h`, `mi`, and `s`; multiple units can be combined:

```sh
gitter list -f 'active:<7d'
gitter list -f 'active:>1y3mo'
gitter list -f 'active:2d'
```

`<duration` means newer than the duration; `>duration` means older. An unprefixed duration matches commits approximately that age (within one minute).

## Placeholders

Placeholders use the `{_tag_}` form and can appear in info templates, sort templates, command arguments, Bash snippets, and—when `--placeholder` is enabled—script contents. See `gitter help placeholders` for their detailed descriptions.

| Placeholder | Value |
| --- | --- |
| `{_remote:n_}` | Remote name |
| `{_remote:f_}` | Fetch URL |
| `{_remote:p_}` | Push URL |
| `{_name_}` | Repository directory name |
| `{_path:r_}` | Relative repository parent path |
| `{_path:a_}` | Absolute repository path |
| `{_nesting_}` | Repository nesting depth |
| `{_branch:n_}` | Current branch name |
| `{_branch:c_}` | Number of branches |
| `{_hash:f_}` | Full commit hash |
| `{_hash:N_}` | Commit hash truncated to `N` characters, e.g. `{_hash:8_}` |
| `{_commit:c_}` | Commit count |
| `{_author:e_}` | Latest commit author's email |
| `{_author:n_}` | Latest commit author's name |
| `{_time:r_}` | Relative commit age |
| `{_time:rc_}` | Relative commit age with combined units |
| `{_time:a_}` | Absolute commit date and time |
| `{_dirty_}` | Dirty-worktree marker |
| `{_bare_}` | Bare-repository marker |
| `{_size_}` | Repository size |
| `{_language_}` | Detected top language |

## Ignore repositories

Place a `.gitterignore` file in a workspace directory to exclude matching repositories during discovery. Patterns are relative to the directory containing that file:

```text
# Ignore one repository path
services/legacy

# Ignore paths beginning with this text
experimental*

# Ignore repositories below a direct child directory
generated/*

# Ignore repositories below child directories with this prefix
vendor-*/*
```

See `gitter help gitterignore` for the supported pattern forms.
