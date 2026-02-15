# Git Time Machine

A command-line tool to visualize your repository's evolution through time.

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
cargo build --release
```

## Tech Stack

- Rust
- git2 (libgit2 bindings)
- clap (CLI parsing)
- chrono (date/time handling)

## License

MIT
