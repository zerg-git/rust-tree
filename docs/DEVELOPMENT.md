# Rust-Tree Development Documentation

This document provides technical details for developers working on the rust-tree project.

## Table of Contents

- [Development Environment](#development-environment)
- [Project Architecture](#project-architecture)
- [Core Modules](#core-modules)
- [Data Structures](#data-structures)
- [Algorithms](#algorithms)
- [Testing](#testing)
- [Building](#building)
- [Release Process](#release-process)

## Development Environment

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Setting Up

```bash
# Clone the repository
git clone <repository-url>
cd rust-tree

# Build the project
cargo build

# Run tests
cargo test

# Install for development
cargo install --path .
```

### Recommended Tools

- **rustfmt**: Code formatting (`cargo fmt`)
- **clippy**: Linting (`cargo clippy`)
- **IDE**: VS Code with rust-analyzer extension

## Project Architecture

### Module Organization

```
rust-tree/
├── src/
│   ├── main.rs          # Binary entry point
│   ├── lib.rs           # Library interface
│   ├── config.rs        # CLI configuration
│   ├── core/
│   │   ├── mod.rs       # Module exports
│   │   ├── models.rs    # Core data structures
│   │   ├── walker.rs    # Directory traversal
│   │   └── collector.rs # Statistics collection
│   └── formatters/
│       ├── mod.rs       # Module exports
│       ├── tree.rs      # Tree output
│       ├── json.rs      # JSON output
│       └── table.rs     # Table output
```

### Dependency Graph

```
main.rs
  └─> lib.rs
       ├─> config.rs
       ├─> core/
       │    ├─> models.rs
       │    ├─> walker.rs
       │    └─> collector.rs
       └─> formatters/
            ├─> tree.rs
            ├─> json.rs
            └─> table.rs
```

## Core Modules

### models.rs

Defines all core data structures:

- **FsNode**: Represents a file/directory/symlink node
- **FsTree**: Container for the entire directory tree
- **TreeStats**: Collected statistics
- **FileTypeInfo**: Information about files by extension
- **FileEntry**: File entry for sorted listings
- **TreeError**: Error type for operations

### walker.rs

Handles directory traversal:

```rust
pub fn walk_directory(path: &Path, config: &WalkConfig) -> Result<FsTree, TreeError>
```

Key features:
- Uses `walkdir` crate for efficient traversal
- Supports depth limiting
- Handles hidden files
- Follows symlinks (optional)
- Sorts entries by name, size, or type

### collector.rs

Collects statistics from a tree:

```rust
pub fn collect_stats(tree: &FsTree, start_time: Instant) -> TreeStats
```

Key features:
- Counts files, directories, symlinks
- Calculates total size
- Groups files by extension
- Finds largest files
- Measures scan duration

### formatters/

#### tree.rs

Produces tree-style output with Unicode characters:
```
├── file.txt
└── directory/
    └── file.rs
```

#### json.rs

Serializes tree and stats to JSON:
```json
{
  "tree": {...},
  "stats": {...}
}
```

#### table.rs

Creates formatted tables using `comfy-table`:
```
╭──────────┬──────╮
│ Extension │ Count│
╞══════════╪══════╡
│ .rs       │ 11   │
╰──────────┴──────╯
```

## Data Structures

### FsNode

```rust
pub struct FsNode {
    pub name: String,
    pub path: Option<PathBuf>,
    pub node_type: FsNodeType,
    pub size: u64,
    pub depth: usize,
    pub children: Option<Vec<FsNode>>,
}
```

### TreeStats

```rust
pub struct TreeStats {
    pub total_files: usize,
    pub total_directories: usize,
    pub total_symlinks: usize,
    pub total_size: u64,
    pub files_by_extension: HashMap<String, FileTypeInfo>,
    pub largest_files: Vec<FileEntry>,
    pub scan_duration: Duration,
}
```

## Algorithms

### Directory Traversal

The walker uses a recursive approach:

1. Start at root directory
2. For each entry:
   - Check if hidden (skip if not showing)
   - Create FsNode for entry
   - If directory and within depth limit: recurse
3. Sort children based on configuration
4. Return complete tree

### Statistics Collection

1. Traverse the entire tree
2. Count each type (file, directory, symlink)
3. Accumulate file sizes
4. Group by extension using HashMap
5. Sort and select top N files by size

### Sorting

Entries are sorted with the following priority:
1. Directories before files
2. Primary sort field (name, size, or type)
3. Reverse flag applied if set

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_format_tree
```

### Test Structure

Each module contains unit tests in a `#[cfg(test)]` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use rust_tree::*;

#[test]
fn test_full_workflow() {
    // Test complete workflow
}
```

## Building

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Cross-Compilation

```bash
# Build for Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Build for macOS
cargo build --release --target x86_64-apple-darwin

# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc
```

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG
3. Commit changes
4. Create git tag
5. Build release binaries
6. Publish to crates.io (optional)

```bash
# Update version
# Edit Cargo.toml

# Commit
git add -A
git commit -m "Release vX.X.X"

# Tag
git tag vX.X.X

# Publish
cargo publish
```

## Code Style

Follow Rust conventions:
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Document public APIs with rustdoc
- Write tests for all public functions

## Performance Considerations

- Use `walkdir` for efficient directory traversal
- Lazy evaluation for large trees
- Minimize clones where possible
- Use `HashMap` for O(1) lookups
- Consider parallel processing for large directories (future)
