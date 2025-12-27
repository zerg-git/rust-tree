//! rust-tree - A fast directory tree visualization tool.
//!
//! This library provides functionality for traversing directory structures,
//! collecting statistics, and formatting output in various styles.
//!
//! # Example
//!
//! ```no_run
//! use rust_tree::{Config, run};
//!
//! let config = Config {
//!     path: "/path/to/directory".into(),
//!     max_depth: 3,
//!     ..Default::default()
//! };
//!
//! run(config).expect("Failed to run");
//! ```

pub mod config;
pub mod core;
pub mod formatters;

// Re-export commonly used types
pub use config::{Config, OutputFormat, SortBy};
pub use core::{
    models::{FsNode, FsTree, FsNodeType, TreeStats, FileTypeInfo, FileEntry, TreeError},
    walker::{walk_directory, WalkConfig, SortField},
    collector::{collect_stats, get_all_files, get_all_directories},
};
pub use formatters::{format_tree, format_json, format_table};

use std::io::{self, Write};
use std::time::Instant;

/// Run the rust-tree tool with the given configuration.
///
/// This is the main entry point for the library. It performs the following steps:
/// 1. Walk the directory tree
/// 2. Collect statistics
/// 3. Format and output the results
///
/// # Errors
///
/// Returns `TreeError` if directory traversal fails or output formatting fails.
pub fn run(config: Config) -> Result<(), TreeError> {
    let start_time = Instant::now();

    // Walk the directory
    let tree = walk_directory(&config.path, &config.to_walk_config())?;

    // Collect statistics
    let stats = collect_stats(&tree, start_time);

    // Format output based on selected format
    let output = match config.format {
        OutputFormat::Tree => {
            let mut result = format_tree(&tree.root, config.show_size);

            // Add statistics if requested
            if config.show_stats {
                result.push_str("\n\n");
                result.push_str(&crate::formatters::table::format_compact(&stats));
                result.push('\n');
            }

            result
        }
        OutputFormat::Json => {
            format_json(&tree, &stats, true)?
        }
        OutputFormat::Table => {
            format_table(&stats)
        }
    };

    // Print output
    print!("{}", output);
    io::stdout().flush().map_err(|e| TreeError::Other(e.to_string()))?;

    Ok(())
}

/// Default implementation for creating a basic Config.
impl Default for Config {
    fn default() -> Self {
        Config {
            path: ".".into(),
            max_depth: 0,
            format: OutputFormat::Tree,
            show_size: false,
            show_hidden: false,
            sort_by: SortBy::Name,
            reverse: false,
            show_stats: false,
            follow_symlinks: false,
            top_files: 10,
        }
    }
}
