//! JSON output formatter.

use serde_json::json;
use crate::core::models::{FsTree, TreeStats, TreeError};

/// Format a file tree and its statistics as JSON.
///
/// # Arguments
///
/// * `tree` - The file system tree to format
/// * `stats` - Statistics to include in the output
/// * `pretty` - Whether to pretty-print the JSON
///
/// # Returns
///
/// A JSON string representing the tree and statistics.
///
/// # Errors
///
/// Returns `TreeError::Json` if serialization fails.
pub fn format_json(tree: &FsTree, stats: &TreeStats, pretty: bool) -> Result<String, TreeError> {
    let output = json!({
        "tree": {
            "root": tree.root,
            "max_depth": tree.max_depth
        },
        "stats": {
            "total_files": stats.total_files,
            "total_directories": stats.total_directories,
            "total_symlinks": stats.total_symlinks,
            "total_size": stats.total_size,
            "files_by_extension": stats.files_by_extension,
            "largest_files": stats.largest_files,
            "scan_duration_ms": stats.scan_duration.as_millis()
        }
    });

    if pretty {
        serde_json::to_string_pretty(&output).map_err(TreeError::from)
    } else {
        serde_json::to_string(&output).map_err(TreeError::from)
    }
}

/// Format only the tree structure as JSON (without statistics).
///
/// # Arguments
///
/// * `tree` - The file system tree to format
/// * `pretty` - Whether to pretty-print the JSON
///
/// # Returns
///
/// A JSON string representing the tree structure only.
///
/// # Errors
///
/// Returns `TreeError::Json` if serialization fails.
pub fn format_tree_only(tree: &FsTree, pretty: bool) -> Result<String, TreeError> {
    if pretty {
        serde_json::to_string_pretty(&tree.root).map_err(TreeError::from)
    } else {
        serde_json::to_string(&tree.root).map_err(TreeError::from)
    }
}

/// Format only the statistics as JSON.
///
/// # Arguments
///
/// * `stats` - Statistics to format
/// * `pretty` - Whether to pretty-print the JSON
///
/// # Returns
///
/// A JSON string representing the statistics only.
///
/// # Errors
///
/// Returns `TreeError::Json` if serialization fails.
pub fn format_stats_only(stats: &TreeStats, pretty: bool) -> Result<String, TreeError> {
    if pretty {
        serde_json::to_string_pretty(&stats).map_err(TreeError::from)
    } else {
        serde_json::to_string(&stats).map_err(TreeError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{FsNode, FsNodeType};
    use std::time::Duration;

    #[test]
    fn test_format_json() {
        let root = FsNode::new(
            "test".into(),
            "/test".into(),
            FsNodeType::Directory,
            0,
            0,
        );
        let tree = FsTree::new(root, 0);
        let stats = TreeStats {
            total_files: 10,
            total_directories: 2,
            total_symlinks: 0,
            total_size: 1024,
            files_by_extension: Default::default(),
            largest_files: vec![],
            scan_duration: Duration::from_millis(100),
        };

        let json = format_json(&tree, &stats, true).unwrap();

        assert!(json.contains("\"total_files\": 10"));
        assert!(json.contains("\"total_directories\": 2"));
    }

    #[test]
    fn test_format_tree_only() {
        let root = FsNode::new(
            "test".into(),
            "/test".into(),
            FsNodeType::Directory,
            0,
            0,
        );
        let tree = FsTree::new(root, 0);

        let json = format_tree_only(&tree, true).unwrap();
        assert!(json.contains("\"name\": \"test\""));
    }
}
