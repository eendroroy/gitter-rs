# gitter-rs

A fast, concurrent CLI utility for running commands across multiple Git repositories.

`gitter-rs` scans a directory tree, discovers Git repositories, collects repository metadata,
and executes commands in each repository context.
It is designed for monorepo-adjacent workflows, workspace maintenance, and bulk Git operations.

## Features

- Automatically discovers Git repositories recursively
- Concurrent repository scanning and status collection using Tokio
- Placeholder system for dynamic command templating
- Colored and aligned repository status output
- Run:
    - Git commands
    - Arbitrary shell commands
    - Script files
    - Bash expressions
- Shell completion generation
- Configurable output templates

## Installation

### Using `cargo`

```bash
cargo install gitter-rs
```

### Using homebrew (Mac and Linux)

```bash
brew tap eendroroy/tools               # tap
brew trust eendroroy/tools             # trust
brew install eendroroy/tools/gitter-rs # install
```

### From source

```bash
git clone https://github.com/eendroroy/gitter-rs.git
cd gitter-rs
cargo install --path .
```

Or build manually:

```bash
cargo build --release
```

Binary:

```bash
target/release/gitter
```

### Releases

Prebuilt binaries available at [GitHub releases](https://github.com/eendroroy/gitter-rs/releases/)

## Manual

Use the help menu

```bash
gitter help          # help menu
gitter help filters  # topics: placeholders, gitterignore, filters, completions
```

## Examples

`git` is the default command, so `gitter <args>` is the same as `gitter git <args>`.

```bash
gitter list                              # list repositories (aliases: ls, l)
gitter ls -d 3 -f 'branch:master'        # deeper search, filtered

gitter pull                              # $(git pull) in every repo
gitter git checkout develop              # same as above, explicit
gitter -f '! name:gitter-rs' status -s   # gitter options go before git args
gitter -- -c color.ui=never log -1       # `--` for git flags clashing with gitter's

gitter exec cargo test                   # run any program
gitter exec -q basename '{_path:a_}'     # placeholders, hide stdout
gitter bash 'ls | wc -l'                 # bash -c

gitter script ./task.sh                  # run a script (uses $SHELL)
gitter script ./task.sh --shell zsh -P   # pick shell, process placeholders

gitter meta init                         # create .gitter.meta.toml
gitter meta add https://host/repo.git -b main --clone
gitter meta remove repo                  # alias: rm (never deletes from disk)
gitter meta save --merge --prune         # record repos found in the workspace
gitter meta restore -j 8 --dry-run       # preview parallel clones
gitter meta status                       # ok / missing / wrong-remote / wrong-branch / untracked
gitter meta list                         # alias: ls
gitter completion zsh > ~/.zfunc/_gitter # shell completion
gitter help placeholders                 # topics: placeholders, gitterignore, filters, completions
```

## License

The project is available as open source under the terms of
the [AGPL3 License](https://www.fsf.org/licensing/licenses/agpl.html).