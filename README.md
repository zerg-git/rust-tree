# rust-tree

[English](README.md) | [简体中文](README.zh-CN.md)

A fast directory tree visualization tool written in Rust. It displays directory structures in multiple formats and provides comprehensive statistics about file distribution.

## Features

- **Multiple Output Formats**
  - Tree-style output with Unicode box-drawing characters
  - JSON format for programmatic processing
  - Table format for statistical summaries

- **Comprehensive Statistics**
  - File and directory counts
  - Total size calculation
  - Files grouped by extension
  - Largest files listing

- **Flexible Options**
  - Depth-limited scanning
  - Sort by name, size, or type
  - Show hidden files
  - Display file sizes
  - Follow symbolic links

## Installation

### From Source

```bash
cargo install --path .
```

### Build from Source

```bash
git clone <repository-url>
cd rust-tree
cargo build --release
```

The binary will be available at `target/release/rust-tree`.

## Usage

### Basic Usage

```bash
# Show current directory tree
rust-tree

# Show a specific directory
rust-tree /path/to/directory

# Limit depth to 2 levels
rust-tree -d 2 /path/to/directory
```

### Output Formats

```bash
# Tree format (default)
rust-tree

# JSON format
rust-tree -f json

# Table format with statistics
rust-tree -f table -S
```

### Display Options

```bash
# Show file sizes
rust-tree -s

# Show hidden files
rust-tree -a

# Sort by file size (descending)
rust-tree -o size -r

# Sort by file type
rust-tree -o type
```

### Examples

```bash
# Display tree with sizes and statistics
rust-tree -s -S

# JSON output with full statistics
rust-tree -f json -S > stats.json

# Show only 3 levels deep, sorted by size
rust-tree -d 3 -o size -r

# Table format showing all files including hidden
rust-tree -f table -a -S
```

## Command-Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-d, --depth <N>` | Maximum recursion depth (0 = unlimited) | 0 |
| `-f, --format <FORMAT>` | Output format (tree/json/table) | tree |
| `-s, --size` | Show file sizes | false |
| `-a, --all` | Show hidden files | false |
| `-o, --sort <BY>` | Sort by (name/size/type) | name |
| `-r, --reverse` | Reverse sort order | false |
| `-S, --stats` | Show statistics summary | false |
| `-L, --follow` | Follow symbolic links | false |
| `--top-files <N>` | Number of largest files to show | 10 |
| `-h, --help` | Print help | - |
| `-V, --version` | Print version | - |

## Output Examples

### Tree Format

```
src/
├── core/
│   ├── models.rs
│   ├── walker.rs
│   └── collector.rs
├── formatters/
│   ├── tree.rs
│   ├── json.rs
│   └── table.rs
├── config.rs
├── lib.rs
└── main.rs
```

### JSON Format

```json
{
  "tree": {
    "root": {
      "name": "src",
      "type": "directory",
      "children": [...]
    }
  },
  "stats": {
    "total_files": 11,
    "total_directories": 3,
    "total_size": 42944,
    "files_by_extension": {
      ".rs": {
        "count": 11,
        "total_size": 42944,
        "percentage": 100.0
      }
    }
  }
}
```

### Table Format

```
╭──────────────────╬────────╮
│ Statistics       ║        │
╞══════════════════╪════════╡
│ Total Files      ║ 11     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ Total Directories╩ 3      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ Total Size       ║ 42 KB  │
╰──────────────────┴────────╯
```

## Development

### Project Structure

```
rust-tree/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library interface
│   ├── config.rs        # Configuration
│   ├── core/            # Core functionality
│   │   ├── models.rs    # Data structures
│   │   ├── walker.rs    # Directory traversal
│   │   └── collector.rs # Statistics collection
│   └── formatters/      # Output formatters
│       ├── tree.rs      # Tree format
│       ├── json.rs      # JSON format
│       └── table.rs     # Table format
├── docs/                # Documentation
└── tests/               # Tests
```

### Running Tests

```bash
cargo test
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Documentation

- [DEVELOPMENT.md](docs/DEVELOPMENT.md) - Technical implementation details
- [USER_MANUAL.md](docs/USER_MANUAL.md) - Comprehensive user guide
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture design

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
