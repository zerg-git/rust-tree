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
  - **Color support** for better readability
  - **Exclude patterns** for filtering files
  - **Progress indication** for large directories
  - **Streaming mode** for memory-efficient scanning

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

### Color Support

```bash
# Always use colors
rust-tree --color=always

# Never use colors (useful for piping to files)
rust-tree --color=never

# Auto-detect terminal support (default)
rust-tree --color=auto

# Use extended color scheme with more file type colors
rust-tree --color-scheme=extended
```

### Filtering

```bash
# Exclude files matching a pattern
rust-tree --exclude "*.log"

# Exclude multiple patterns
rust-tree -e "*.log" -e "node_modules" -e ".git"

# Include only files matching a pattern
rust-tree --include-only "*.rs"

# Use common exclude patterns for specific languages
rust-tree --exclude-common=rust      # Rust projects
rust-tree --exclude-common=python    # Python projects
rust-tree --exclude-common=nodejs    # Node.js projects
rust-tree --exclude-common=common    # Common development files
```

### Progress Indication

```bash
# Show progress bar during scanning
rust-tree --progress
```

### Streaming Mode

```bash
# Use streaming mode for low memory usage on large directories
rust-tree --streaming
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
# Display tree with colors and sizes
rust-tree --color=always -s

# Scan with progress indication for large directories
rust-tree --progress /large/directory

# Exclude build artifacts and dependencies
rust-tree --exclude-common=rust --exclude "*.rlib"

# Memory-efficient scanning with streaming
rust-tree --streaming -d 5 /very/large/directory

# JSON output with full statistics
rust-tree -f json -S > stats.json

# Show only Rust source files
rust-tree --include-only "*.rs"

# Colored tree with statistics
rust-tree --color-scheme=extended -s -S
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
| `--color <WHEN>` | Color mode (always/never/auto) | auto |
| `--color-scheme <SCHEME>` | Color scheme (none/basic/extended) | basic |
| `-p, --progress` | Show progress bar | false |
| `-e, --exclude <PATTERN>` | Exclude files matching pattern | - |
| `--include-only <PATTERN>` | Include only files matching pattern | - |
| `--exclude-common <LANGUAGE>` | Use common exclude patterns | - |
| `--streaming` | Use streaming mode for low memory | false |
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
│   ├── config.rs        # Configuration & CLI parsing
│   ├── core/            # Core functionality
│   │   ├── models.rs    # Data structures
│   │   ├── walker.rs    # Directory traversal
│   │   ├── collector.rs # Statistics collection
│   │   ├── filter.rs    # Pattern filtering
│   │   ├── progress.rs  # Progress indication
│   │   └── streaming.rs # Memory-efficient streaming
│   └── formatters/      # Output formatters
│       ├── tree.rs      # Tree format
│       ├── json.rs      # JSON format
│       ├── table.rs     # Table format
│       └── streaming_tree.rs # Streaming tree format
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
