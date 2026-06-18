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
gitter help        # help menu
gitter help --help # help topics
```

## Examples

```bash
gitter git -- pull        # $(git pull)
gitter exec -- cargo test # $(cargo test)
gitter checkout develop   # $(git checkout development)
```

## License

The project is available as open source under the terms of
the [AGPL3 License](https://www.fsf.org/licensing/licenses/agpl.html).