# rgrep (Rust Grep Clone)

`rgrep` is a small `grep`-like CLI tool built in Rust.  
It searches for a text pattern inside a file or recursively inside a directory.

This project is designed to practice real-world Rust skills such as:
- file I/O
- buffered reading
- CLI flags
- error handling
- recursive directory traversal
- colored terminal output

---

## Features

- Search for a pattern in a file
- Recursive search inside directories (`-r`)
- Case-insensitive search (`-i`)
- Show line numbers (`-n`)
- Count matching lines per file (`-c`)
- Print only file names with matches (`-l`)
- Highlight first match in red (`--color`)
- Exit codes like real grep:
  - `0` = match found
  - `1` = no match found
  - `2` = error

---

## Tech Stack

- Rust
- `clap` (CLI parsing)
- `anyhow` (error handling)
- `walkdir` (recursive file walking)
- `termcolor` (colored terminal output)

---

## Installation

### Build
```bash
cargo build
