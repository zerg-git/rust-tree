# Rust-Tree User Manual

Complete guide for using the rust-tree command-line tool.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Command-Line Options](#command-line-options)
- [Output Formats](#output-formats)
- [Examples](#examples)
- [Use Cases](#use-cases)
- [Troubleshooting](#troubleshooting)

## Introduction

rust-tree is a fast command-line tool for visualizing directory structures and analyzing file statistics. It provides multiple output formats and comprehensive information about your files.

### Key Features

- **Multiple output formats**: Tree, JSON, and Table
- **Detailed statistics**: File counts, sizes, and extension breakdown
- **Flexible filtering**: Depth limits, hidden files, sorting options
- **Fast performance**: Written in Rust for optimal speed

## Installation

### Method 1: Using cargo

```bash
cargo install rust-tree
```

### Method 2: From source

```bash
git clone <repository-url>
cd rust-tree
cargo install --path .
```

### Method 3: Pre-built binaries

Download the appropriate binary for your platform from the [releases page](https://github.com/user/rust-tree/releases) and place it in your PATH.

## Quick Start

### Basic Usage

```bash
# Display current directory
rust-tree

# Display a specific directory
rust-tree /path/to/directory

# Limit depth to 2 levels
rust-tree -d 2
```

### Common Patterns

```bash
# Show file sizes
rust-tree -s

# Display statistics
rust-tree -S

# Output as JSON
rust-tree -f json
```

## Command-Line Options

### Synopsis

```bash
rust-tree [OPTIONS] [DIRECTORY]
```

### Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `DIRECTORY` | Target directory path | Current directory |

### Options

| Short | Long | Description | Default |
|-------|------|-------------|---------|
| `-d` | `--depth <N>` | Maximum recursion depth (0 = unlimited) | 0 |
| `-f` | `--format <FORMAT>` | Output format | tree |
| `-s` | `--size` | Show file sizes | false |
| `-a` | `--all` | Show hidden files | false |
| `-o` | `--sort <BY>` | Sort by field | name |
| `-r` | `--reverse` | Reverse sort order | false |
| `-S` | `--stats` | Show statistics | false |
| `-L` | `--follow` | Follow symbolic links | false |
| `--top-files <N>` | Number of largest files to show | 10 |
| `-h` | `--help` | Print help | - |
| `-V` | `--version` | Print version | - |

### Output Format Values

| Value | Description |
|-------|-------------|
| `tree` | Tree-style output with Unicode characters |
| `json` | JSON format (includes tree and statistics) |
| `table` | Table format showing statistics |

### Sort By Values

| Value | Description |
|-------|-------------|
| `name` | Sort by file/directory name (default) |
| `size` | Sort by file size |
| `type` | Sort by file type/extension |

## Output Formats

### Tree Format

The default format shows the directory hierarchy using Unicode box-drawing characters.

```
project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── test.rs
└── Cargo.toml
```

**With sizes (`-s`)**:
```
project/
├── src/ (2 files)
│   ├── main.rs (1.2 KB)
│   └── lib.rs (3.4 KB)
└── Cargo.toml (256 B)
```

**With statistics (`-S`)**:
```
project/
└── ...

12 files, 3 directories, 15.2 KB total
```

### JSON Format

Structured output for programmatic processing.

```json
{
  "tree": {
    "root": {
      "name": "project",
      "type": "directory",
      "children": [...]
    },
    "max_depth": 3
  },
  "stats": {
    "total_files": 12,
    "total_directories": 3,
    "total_symlinks": 0,
    "total_size": 15563,
    "files_by_extension": {
      ".rs": {
        "count": 8,
        "total_size": 12000,
        "percentage": 77.1
      }
    },
    "largest_files": [...],
    "scan_duration_ms": 5
  }
}
```

### Table Format

Statistics displayed as formatted tables.

```
╭───────────────────╬──────────╮
│ Statistics        ║          │
╞═══════════════════╪══════════╡
│ Total Files       ║ 12       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Total Directories ║ 3        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Total Size        ║ 15.2 KB  │
╰───────────────────┴──────────╯

Files by Extension
╭─────────────╬───────╬─────────╬───────────╮
│ Extension   ║ Count ║ Size    ║ Percentage│
╞═════════════╪═══════╪═════════╪═══════════╡
│ .rs         ║ 8     ║ 12 KB   ║ 77.1%     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┌┤
│ .toml       ║ 1     ║ 256 B   ║ 1.6%      │
╰─────────────┴───────┴─────────┴───────────╯
```

## Examples

### Analyzing a Project

```bash
# Show project structure with file sizes
rust-tree -s ~/projects/my-app

# Show statistics in table format
rust-tree -f table -S ~/projects/my-app

# Find the largest files
rust-tree -f json -S ~/projects/my-app | jq '.stats.largest_files'
```

### Quick Directory Overview

```bash
# Show only 2 levels deep
rust-tree -d 2

# Include hidden files, sorted by size
rust-tree -a -o size -r

# Compact statistics
rust-tree -f table -S
```

### Export for Analysis

```bash
# Export to JSON
rust-tree -f json -S > project-stats.json

# Export tree to text file
rust-tree > directory-tree.txt

# Get file count
rust-tree -f json | jq '.stats.total_files'
```

### Comparing Directories

```bash
# Compare two directories
rust-tree -S dir1
rust-tree -S dir2

# Export both for comparison
rust-tree -f json dir1 > stats1.json
rust-tree -f json dir2 > stats2.json
```

## Use Cases

### 1. Project Documentation

Generate a directory tree for README files:

```bash
rust-tree -d 2 --output README-tree.txt
```

### 2. Disk Usage Analysis

Find large files in a directory:

```bash
rust-tree -f table -S -o size -r
```

### 3. Code Review

Get an overview of a project structure:

```bash
rust-tree -d 3 ~/projects/review-project
```

### 4. Backup Planning

See what will be included in a backup:

```bash
rust-tree -a -S ~/documents
```

### 5. CI/CD Integration

Generate statistics in build pipelines:

```bash
rust-tree -f json -S > build-stats.json
```

## Troubleshooting

### Permission Denied

If you encounter permission errors:

```bash
# Use sudo (Linux/macOS)
sudo rust-tree /root/directory

# Or exclude system directories
rust-tree ~/projects
```

### Too Much Output

Limit the output:

```bash
# Limit depth
rust-tree -d 2

# Show only specific format
rust-tree -f table
```

### Hidden Files Not Showing

Use the `-a` flag:

```bash
rust-tree -a
```

### Slow Performance on Large Directories

Limit depth and use statistics:

```bash
rust-tree -d 3 -f table -S
```

### Symbolic Links Causing Loops

The tool detects symlink loops by default. To follow symlinks:

```bash
rust-tree -L
```

### JSON Parsing Errors

Ensure your JSON parser handles the output:

```bash
# Use jq for pretty printing
rust-tree -f json | jq '.'

# Use python
rust-tree -f json | python -m json.tool
```

## Tips and Tricks

### Combining with Other Tools

```bash
# Count files by extension
rust-tree -f json | jq '.stats.files_by_extension | keys'

# Find total size
rust-tree -f json | jq '.stats.total_size'

# Get largest file path
rust-tree -f json | jq '.stats.largest_files[0].path'
```

### Creating Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
alias tree='rust-tree'
alias trees='rust-tree -s'
alias tree-stats='rust-tree -f table -S'
```

### Shell Integration

```bash
# cd into a directory from tree output
cd $(rust-tree -f json | jq -r '.tree.root.children[0].path')
```

## Support

For issues, questions, or contributions, please visit:
- GitHub: https://github.com/user/rust-tree
- Documentation: https://docs.rs/rust-tree
