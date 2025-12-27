//! Configuration structures for the rust-tree tool.

use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use crate::core::walker::{WalkConfig, SortField};

/// Output format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Tree-style output with Unicode characters
    Tree,
    /// JSON format (includes both tree and statistics)
    Json,
    /// Table format showing statistics
    Table,
}

/// Sort field options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortBy {
    /// Sort by file/directory name
    Name,
    /// Sort by file size
    Size,
    /// Sort by file type/extension
    Type,
}

impl From<SortBy> for SortField {
    fn from(sort_by: SortBy) -> Self {
        match sort_by {
            SortBy::Name => SortField::Name,
            SortBy::Size => SortField::Size,
            SortBy::Type => SortField::Type,
        }
    }
}

/// Command-line arguments for rust-tree.
#[derive(Parser, Debug)]
#[command(name = "rust-tree")]
#[command(author = "rust-tree contributors")]
#[command(version = "0.1.0")]
#[command(about = "A fast directory tree visualization tool", long_about = None)]
#[command(after_help = "Examples:\n  rust-tree                    # Show current directory\n  rust-tree -d 2 /path/to/dir  # Limit depth to 2\n  rust-tree -f json -S         # JSON output with stats\n  rust-tree -s -o size -r      # Show sizes, sort by size (descending)")]
pub struct Config {
    /// Target directory path (defaults to current directory)
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub path: PathBuf,

    /// Maximum recursion depth (0 means unlimited)
    #[arg(short = 'd', long = "depth", default_value = "0", value_name = "N")]
    pub max_depth: usize,

    /// Output format
    #[arg(short = 'f', long = "format", default_value = "tree", value_name = "FORMAT")]
    pub format: OutputFormat,

    /// Show file sizes
    #[arg(short = 's', long = "size")]
    pub show_size: bool,

    /// Show hidden files (starting with .)
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// Sort by field (name, size, type)
    #[arg(short = 'o', long = "sort", default_value = "name", value_name = "BY")]
    pub sort_by: SortBy,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// Show statistics summary (for tree format) or always include stats (for json/table)
    #[arg(short = 'S', long = "stats")]
    pub show_stats: bool,

    /// Follow symbolic links
    #[arg(short = 'L', long = "follow")]
    pub follow_symlinks: bool,

    /// Number of largest files to show in statistics
    #[arg(long = "top-files", default_value = "10", value_name = "N")]
    pub top_files: usize,
}

impl Config {
    /// Convert to a WalkConfig for use by the walker module.
    pub fn to_walk_config(&self) -> WalkConfig {
        WalkConfig {
            max_depth: self.max_depth,
            show_hidden: self.show_hidden,
            follow_symlinks: self.follow_symlinks,
            sort_by: self.sort_by.into(),
            reverse: self.reverse,
        }
    }

    /// Check if statistics should be displayed.
    pub fn should_show_stats(&self) -> bool {
        self.show_stats || matches!(self.format, OutputFormat::Json | OutputFormat::Table)
    }

    /// Get the effective top files count.
    pub fn top_files_count(&self) -> usize {
        self.top_files.max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_conversion() {
        assert_eq!(SortField::from(SortBy::Name), SortField::Name);
        assert_eq!(SortField::from(SortBy::Size), SortField::Size);
        assert_eq!(SortField::from(SortBy::Type), SortField::Type);
    }

    #[test]
    fn test_output_format_values() {
        let formats = [OutputFormat::Tree, OutputFormat::Json, OutputFormat::Table];
        assert_eq!(formats.len(), 3);
    }
}
