# Rust-Tree Architecture

This document describes the architecture and design principles of the rust-tree project.

## Table of Contents

- [Design Principles](#design-principles)
- [System Architecture](#system-architecture)
- [Module Design](#module-design)
- [Data Flow](#data-flow)
- [Error Handling](#error-handling)
- [Performance Considerations](#performance-considerations)
- [Future Improvements](#future-improvements)

## Design Principles

### 1. Separation of Concerns

The project is organized into distinct modules with clear responsibilities:

- **Core**: Data structures and business logic
- **Formatters**: Output generation only
- **Config**: CLI parsing and configuration
- Each module has no knowledge of others' internals

### 2. Library-First Design

The project is designed as a library first, with a CLI binary on top:

```rust
// Can be used as a library
use rust_tree::{walk_directory, collect_stats};

let tree = walk_directory(&path, &config)?;
let stats = collect_stats(&tree, start_time);
```

### 3. Extensibility

Adding new features should require minimal changes:

- New output formats: Add to `formatters/` module
- New statistics: Extend `TreeStats` structure
- New sort options: Add to `SortField` enum

### 4. Testability

Each module is independently testable:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_module_function() {
        // Test code
    }
}
```

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                    CLI Layer                         │
│  ┌─────────────────────────────────────────────┐   │
│  │  main.rs (Binary Entry Point)               │   │
│  │  - Parse CLI arguments                      │   │
│  │  - Call lib.rs run()                        │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│                  Library Layer                       │
│  ┌─────────────────────────────────────────────┐   │
│  │  lib.rs (Library Interface)                 │   │
│  │  - Orchestrate modules                      │   │
│  │  - Public API                               │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
┌──────────────┐  ┌─────────────┐  ┌─────────────┐
│   Config     │  │    Core     │  │ Formatters  │
│              │  │             │  │             │
│ - CLI args   │  │ - Models    │  │ - Tree      │
│ - WalkConfig │  │ - Walker    │  │ - JSON      │
│              │  │ - Collector │  │ - Table     │
└──────────────┘  └─────────────┘  └─────────────┘
```

### Component Interaction

```
User Input (CLI)
       │
       ▼
┌─────────────┐
│   Config    │ Parse arguments
└─────┬───────┘
      │
      ▼
┌─────────────┐
│   Walker    │ Traverse filesystem
└─────┬───────┘
      │
      ▼ FsTree
┌─────────────┐
│  Collector  │ Collect statistics
└─────┬───────┘
      │
      ▼ TreeStats
┌─────────────┐
│ Formatters  │ Generate output
└─────┬───────┘
      │
      ▼ String Output
    Display
```

## Module Design

### Config Module

**Responsibility**: Parse and store configuration

```rust
pub struct Config {
    pub path: PathBuf,
    pub max_depth: usize,
    pub format: OutputFormat,
    pub show_size: bool,
    // ... other fields
}
```

**Design Decisions**:
- Uses `clap` derive macros for type-safe parsing
- Converts to `WalkConfig` for internal use
- Implements `Default` trait

### Core Models Module

**Responsibility**: Define data structures

```rust
pub struct FsNode {
    pub name: String,
    pub node_type: FsNodeType,
    pub size: u64,
    pub children: Option<Vec<FsNode>>,
}
```

**Design Decisions**:
- Uses `Option` for nullable fields (path, children)
- Implements `Serialize` for JSON output
- Provides helper methods (`is_file()`, `is_directory()`)

### Core Walker Module

**Responsibility**: Traverse directory structures

```rust
pub fn walk_directory(path: &Path, config: &WalkConfig) -> Result<FsTree, TreeError>
```

**Design Decisions**:
- Uses `walkdir` crate for efficiency
- Recursive implementation for clarity
- Filters and sorts during traversal
- Handles errors gracefully

### Core Collector Module

**Responsibility**: Collect statistics from trees

```rust
pub fn collect_stats(tree: &FsTree, start_time: Instant) -> TreeStats
```

**Design Decisions**:
- Separate from walker for single responsibility
- Uses HashMap for extension grouping
- Sorts and limits results after collection

### Formatters Module

**Responsibility**: Generate output in various formats

```rust
pub fn format_tree(node: &FsNode, show_size: bool) -> String
pub fn format_json(tree: &FsTree, stats: &TreeStats, pretty: bool) -> Result<String, TreeError>
pub fn format_table(stats: &TreeStats) -> String
```

**Design Decisions**:
- Each formatter is independent
- No shared state
- Returns strings for easy testing
- Use external libraries for formatting (comfy-table, serde_json)

## Data Flow

### Main Flow

```
1. CLI Input
   └─> Config::parse()
       └─> Config { path, format, ... }

2. Directory Traversal
   └─> walk_directory(path, config)
       └─> FsTree { root, max_depth }

3. Statistics Collection
   └─> collect_stats(tree, start_time)
       └─> TreeStats { total_files, ... }

4. Format Selection
   └─> match config.format {
           Tree => format_tree(...)
           Json => format_json(...)
           Table => format_table(...)
       }
       └─> String

5. Output
   └─> print!("{}", output)
```

### Walker Data Flow

```
walk_directory(path, config)
    │
    ▼
validate path
    │
    ▼
walk_recursive(path, 0, config)
    │
    ├─> read metadata
    ├─> create FsNode
    ├─> if directory:
    │   └─> collect_children()
    │       ├─> WalkDir::new(path)
    │       ├─> for each entry:
    │       │   ├─> filter hidden
    │       │   ├─> walk_recursive(entry)
    │       │   └─> add to children
    │       └─> sort_entries()
    └─> return FsNode
    │
    ▼
calculate_max_depth()
    │
    ▼
FsTree::new(root, max_depth)
```

## Error Handling

### Error Types

```rust
pub enum TreeError {
    Io(std::io::Error),
    PathNotFound(PathBuf),
    NotADirectory(PathBuf),
    PermissionDenied(PathBuf),
    Json(String),
    Other(String),
}
```

### Error Handling Strategy

1. **Use Result<T, TreeError>**: All fallible operations return Result
2. **Use ? operator**: Propagate errors up the call stack
3. **Convert errors**: Use From trait for error conversion
4. **Context at boundaries**: Add context only at API boundaries

### Error Propagation

```
Filesystem Operation
    │
    ▼ std::io::Error
    │
    ▼ ? operator
    │
    ▼ TreeError::Io
    │
    ▼ walk_directory() returns Result
    │
    ▼ lib.rs handles error
    │
    ▼ main() prints error and exits
```

## Performance Considerations

### Current Optimizations

1. **walkdir**: Efficient directory traversal
2. **Lazy sorting**: Sort only when displaying
3. **HashMap lookups**: O(1) for extension grouping
4. **Minimal clones**: Use references where possible

### Performance Characteristics

| Operation | Complexity |
|-----------|------------|
| Directory traversal | O(n) |
| Sorting | O(n log n) |
| Extension grouping | O(n) |
| Largest files | O(n log k) |

### Bottlenecks

1. **Filesystem I/O**: Main bottleneck for deep trees
2. **Memory usage**: Loading entire tree into memory
3. **Sorting**: For directories with many entries

### Future Optimizations

1. **Parallel traversal**: Use rayon for concurrent scanning
2. **Streaming output**: Output as we traverse
3. **Incremental mode**: Cache results for re-scanning

## Future Improvements

### Planned Features

1. **Filtering**
   ```rust
   pub struct FilterConfig {
       pub extensions: Vec<String>,
       pub min_size: Option<u64>,
       pub max_size: Option<u64>,
   }
   ```

2. **Parallel Scanning**
   ```rust
   use rayon::prelude::*;
   entries.par_iter().map(|e| process(e))
   ```

3. **Incremental Updates**
   ```rust
   pub struct ScanCache {
       pub last_scan: Instant,
       pub file_hashes: HashMap<PathBuf, u64>,
   }
   ```

### Potential Enhancements

1. **Git integration**: Show file status
2. **.gitignore support**: Respect ignore files
3. **HTML output**: Generate interactive HTML
4. **Graph output**: Visual representation
5. **Export formats**: CSV, XML

## Design Trade-offs

### Tree vs. Streaming

**Choice**: Load entire tree into memory

**Rationale**:
- Simpler implementation
- Enables sorting and filtering
- Acceptable for typical directory sizes

**Alternative**: Stream output as we traverse

### Recursive vs. Iterative

**Choice**: Recursive directory traversal

**Rationale**:
- More readable code
- Natural for tree structures
- Depth limit prevents stack overflow

**Alternative**: Iterative with explicit stack

### String vs. Structured Output

**Choice**: Formatters return strings

**Rationale**:
- Simple API
- Easy to test
- Flexible for consumers

**Alternative**: Return structured types

## Conclusion

The architecture prioritizes:
- **Clarity**: Code is easy to understand
- **Maintainability**: Changes are localized
- **Extensibility**: New features fit naturally
- **Performance**: Sufficient for typical use cases

The design balances simplicity with functionality, making the codebase accessible to contributors while providing powerful features for users.
