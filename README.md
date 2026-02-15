# Git Time Machine

A command-line tool to visualize your repository's evolution through time.

## Requirements

- Rust 1.83+ (for latest dependencies)
- Or use the locked Cargo.lock for Rust 1.75+

## Features

- [x] List commit history with timestamps
- [ ] Interactive timeline TUI
- [ ] File tree visualization at any point in history
- [ ] Search: "When was this line introduced?"
- [ ] Heatmap of file changes over time
- [ ] Export snapshots of the repo at any commit

## Usage

```bash
# Show commit history for current directory
git-time-machine

# Show history for a specific repo
git-time-machine -p /path/to/repo
```

## Building

```bash
# With latest Rust
cargo build --release

# With Rust 1.75 (locked dependencies)
cargo build --release --locked
```

## Tech Stack

- Rust
- git2 (libgit2 bindings)
- clap (CLI parsing)
- chrono (date/time handling)

## License

MIT
