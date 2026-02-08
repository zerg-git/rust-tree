//! Directory traversal functionality using walkdir.

use std::path::Path;
use walkdir::WalkDir;
use crate::core::models::{FsNode, FsTree, FsNodeType, TreeError};
use crate::core::filter::FilterConfig;

/// Configuration for directory walking.
#[derive(Debug, Clone)]
pub struct WalkConfig {
    /// Maximum depth to traverse (0 for unlimited)
    pub max_depth: usize,
    /// Show hidden files (starting with .)
    pub show_hidden: bool,
    /// Follow symbolic links
    pub follow_symlinks: bool,
    /// Sort by field
    pub sort_by: SortField,
    /// Reverse sort order
    pub reverse: bool,
    /// Filter configuration
    pub filter: FilterConfig,
}

/// Sort field for directory entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    /// Sort by name (default)
    Name,
    /// Sort by file size
    Size,
    /// Sort by file type/extension
    Type,
}

impl Default for WalkConfig {
    fn default() -> Self {
        Self {
            max_depth: 0, // Unlimited
            show_hidden: false,
            follow_symlinks: false,
            sort_by: SortField::Name,
            reverse: false,
            filter: FilterConfig::default(),
        }
    }
}

/// Walk a directory and build a file tree.
///
/// # Arguments
///
/// * `path` - Root directory path to walk
/// * `config` - Configuration options for walking
///
/// # Returns
///
/// A `FsTree` containing the complete directory structure.
///
/// # Errors
///
/// Returns `TreeError` if the path doesn't exist, isn't a directory,
/// or permission is denied.
pub fn walk_directory(path: &Path, config: &WalkConfig) -> Result<FsTree, TreeError> {
    // Validate the path
    if !path.exists() {
        return Err(TreeError::PathNotFound(path.to_path_buf()));
    }

    let meta = std::fs::metadata(path)?;
    if !meta.is_dir() {
        return Err(TreeError::NotADirectory(path.to_path_buf()));
    }

    // Build the tree recursively
    let root_node = walk_recursive(path, 0, config)?;
    let max_depth = calculate_max_depth(&root_node);

    Ok(FsTree::new(root_node, max_depth))
}

/// Recursively walk a directory and build a node.
fn walk_recursive(path: &Path, depth: usize, config: &WalkConfig) -> Result<FsNode, TreeError> {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let meta = std::fs::metadata(path)
        .or_else(|_| std::fs::symlink_metadata(path))?;

    let node_type = if meta.is_symlink() {
        FsNodeType::Symlink
    } else if meta.is_dir() {
        FsNodeType::Directory
    } else {
        FsNodeType::File
    };

    let size = meta.len();

    // If it's a directory, collect children
    let children = if node_type == FsNodeType::Directory {
        // Check depth limit
        if config.max_depth > 0 && depth >= config.max_depth {
            None
        } else {
            let entries = collect_children(path, depth + 1, config)?;
            if entries.is_empty() {
                None
            } else {
                Some(entries)
            }
        }
    } else {
        None
    };

    let mut node = FsNode::new(name, path.to_path_buf(), node_type, size, depth);

    if let Some(children) = children {
        node.children = Some(children);
    }

    Ok(node)
}

/// Collect and sort child entries of a directory.
fn collect_children(path: &Path, depth: usize, config: &WalkConfig) -> Result<Vec<FsNode>, TreeError> {
    let mut entries = Vec::new();

    // Use WalkDir for efficient traversal
    let mut walker = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .follow_links(config.follow_symlinks)
        .into_iter();

    for entry in walker {
        match entry {
            Ok(entry) => {
                // Apply filter
                if config.filter.should_exclude(entry.path()) {
                    continue;
                }

                match walk_recursive(entry.path(), depth, config) {
                    Ok(node) => entries.push(node),
                    Err(_) => {
                        // Skip entries we can't access
                        continue;
                    }
                }
            }
            Err(_) => {
                // Skip IO errors (permission denied, etc.)
                continue;
            }
        }
    }

    // Sort entries
    sort_entries(&mut entries, config);

    Ok(entries)
}

/// Sort directory entries based on configuration.
fn sort_entries(entries: &mut [FsNode], config: &WalkConfig) {
    match config.sort_by {
        SortField::Name => {
            entries.sort_by(|a, b| {
                // Directories first
                if a.is_directory() && !b.is_directory() {
                    return std::cmp::Ordering::Less;
                }
                if !a.is_directory() && b.is_directory() {
                    return std::cmp::Ordering::Greater;
                }
                // Then by name
                a.name.cmp(&b.name)
            });
        }
        SortField::Size => {
            entries.sort_by(|a, b| {
                // Directories first
                if a.is_directory() && !b.is_directory() {
                    return std::cmp::Ordering::Less;
                }
                if !a.is_directory() && b.is_directory() {
                    return std::cmp::Ordering::Greater;
                }
                // Then by size
                b.size.cmp(&a.size)
            });
        }
        SortField::Type => {
            entries.sort_by(|a, b| {
                // Directories first
                if a.is_directory() && !b.is_directory() {
                    return std::cmp::Ordering::Less;
                }
                if !a.is_directory() && b.is_directory() {
                    return std::cmp::Ordering::Greater;
                }
                // Then by extension
                let a_ext = a.extension().unwrap_or_default();
                let b_ext = b.extension().unwrap_or_default();
                a_ext.cmp(&b_ext)
                    .then_with(|| a.name.cmp(&b.name))
            });
        }
    }

    // Reverse if requested
    if config.reverse {
        entries.reverse();
    }
}

/// Calculate the maximum depth of a node tree.
fn calculate_max_depth(node: &FsNode) -> usize {
    let mut max_depth = node.depth;

    if let Some(children) = &node.children {
        for child in children {
            let child_depth = calculate_max_depth(child);
            if child_depth > max_depth {
                max_depth = child_depth;
            }
        }
    }

    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_config_default() {
        let config = WalkConfig::default();
        assert_eq!(config.max_depth, 0);
        assert!(!config.show_hidden);
        assert!(!config.follow_symlinks);
    }
}
