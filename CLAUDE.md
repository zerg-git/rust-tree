# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Build
cargo build              # Debug build
cargo build --release    # Release build (binary at target/release/rust-tree)

# Testing
cargo test               # Run all tests
cargo test -- --nocapture  # Run tests with output
cargo test <test_name>   # Run specific test

# Linting & Formatting
cargo fmt                # Format code
cargo clippy             # Run linter

# Installation
cargo install --path .   # Install locally
```

## Architecture Overview

This is a library-first CLI tool for directory tree visualization. The codebase follows a clean separation of concerns:

```
User Input → Config → Walker → FsTree → Collector → TreeStats → Formatter → Output
```

### Key Modules

- **`src/main.rs`**: Minimal CLI entry point - parses args and calls `lib.rs::run()`
- **`src/lib.rs`**: Orchestration layer and public API
- **`src/config.rs`**: CLI parsing with `clap` derive macros; converts to `WalkConfig` for internal use
- **`src/core/`**: Core business logic
  - `models.rs`: Data structures (`FsNode`, `FsTree`, `TreeStats`, `TreeError`)
  - `walker.rs`: Directory traversal using `walkdir` crate
  - `collector.rs`: Statistics aggregation (counts, sizes, extension grouping)
- **`src/formatters/`**: Output formatting (tree, JSON, table) - each is independent

### Design Patterns

1. **Library-First**: Can be used as a library (`use rust_tree::{walk_directory, collect_stats}`) or binary
2. **Memory-Tree Approach**: Loads entire tree into memory for processing/sorting
3. **Independent Formatters**: Each formatter returns a `String`, no shared state
4. **Error Handling**: Uses `thiserror` for `TreeError` enum; all fallible operations return `Result`

### Adding New Features

- **New output format**: Add to `src/formatters/` and update `OutputFormat` enum in `config.rs`
- **New statistics**: Extend `TreeStats` struct in `core/models.rs`
- **New sort option**: Add to `SortField` enum in `core/models.rs`

### Dependencies

- `clap` (derive): CLI argument parsing
- `walkdir`: Efficient directory traversal
- `serde`/`serde_json`: JSON serialization
- `comfy-table`: Table formatting
- `humansize`: Human-readable file sizes
- `thiserror`/`anyhow`: Error handling
- `tempfile` (dev): Integration testing

### Testing

Integration tests are in `tests/integration_test.rs`. Unit tests are in `#[cfg(test)]` modules within each source file.
