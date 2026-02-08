//! Streaming directory traversal for memory efficiency.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::core::models::{FsNode, FsNodeType, TreeError};
use crate::core::walker::{SortField};
use crate::core::filter::FilterConfig;

/// A streaming tree node for output.
#[derive(Debug, Clone)]
pub struct StreamNode {
    pub name: String,
    pub path: PathBuf,
    pub node_type: FsNodeType,
    pub size: u64,
    pub depth: usize,
    pub has_children: bool,
    pub is_last: bool, // true if this is the last child of its parent
}

/// Configuration for streaming traversal.
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub max_depth: usize,
    pub show_hidden: bool,
    pub follow_symlinks: bool,
    pub sort_by: SortField,
    pub reverse: bool,
    pub filter: FilterConfig,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            max_depth: 0,
            show_hidden: false,
            follow_symlinks: false,
            sort_by: SortField::Name,
            reverse: false,
            filter: FilterConfig::default(),
        }
    }
}

/// Process a directory tree using streaming output.
///
/// This function visits each node in the tree and calls the provided callback
/// with information about the node. The callback can output the node immediately,
/// enabling constant memory usage regardless of tree size.
pub fn walk_streaming<F>(
    root: &Path,
    config: StreamConfig,
    mut callback: F,
) -> Result<(), TreeError>
where
    F: FnMut(&StreamNode),
{
    if !root.exists() {
        return Err(TreeError::PathNotFound(root.to_path_buf()));
    }

    let meta = std::fs::metadata(root)?;
    if !meta.is_dir() {
        return Err(TreeError::NotADirectory(root.to_path_buf()));
    }

    // Start recursive traversal
    walk_recursive_streaming(root, 0, true, &config, &mut callback);

    Ok(())
}

/// Recursively walk a directory with streaming output.
fn walk_recursive_streaming<F>(
    path: &Path,
    depth: usize,
    is_last: bool,
    config: &StreamConfig,
    callback: &mut F,
) where
    F: FnMut(&StreamNode),
{
    // Check depth limit
    if config.max_depth > 0 && depth > config.max_depth {
        return;
    }

    // Collect entries
    let mut entries = Vec::new();

    let walker = WalkDir::new(path)
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

                entries.push(entry);
            }
            Err(_) => continue,
        }
    }

    // Sort entries
    sort_entries(&mut entries, config);

    let total = entries.len();

    // Process each entry
    for (i, entry) in entries.into_iter().enumerate() {
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        let meta = match std::fs::metadata(entry_path)
            .or_else(|_| std::fs::symlink_metadata(entry_path)) {
                Ok(m) => m,
                Err(_) => continue,
            };

        let node_type = if meta.is_symlink() {
            FsNodeType::Symlink
        } else if meta.is_dir() {
            FsNodeType::Directory
        } else {
            FsNodeType::File
        };

        let size = meta.len();
        let is_last = i == total - 1;

        // Recurse into directories
        if node_type == FsNodeType::Directory {
            // First, check if it has children (without loading them all)
            let has_children = has_children(entry_path, config);

            // Emit current directory
            callback(&StreamNode {
                name: name.clone(),
                path: entry_path.to_path_buf(),
                node_type,
                size,
                depth,
                has_children,
                is_last,
            });

            // Then recurse into children
            walk_recursive_streaming(entry_path, depth + 1, is_last, config, callback);
        } else {
            // Emit file/symlink
            callback(&StreamNode {
                name,
                path: entry_path.to_path_buf(),
                node_type,
                size,
                depth,
                has_children: false,
                is_last,
            });
        }
    }
}

/// Check if a directory has any (non-filtered) children.
fn has_children(path: &Path, config: &StreamConfig) -> bool {
    let walker = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .follow_links(config.follow_symlinks)
        .into_iter();

    for entry in walker {
        if let Ok(entry) = entry {
            if !config.filter.should_exclude(entry.path()) {
                return true;
            }
        }
    }

    false
}

/// Sort entries based on configuration.
fn sort_entries(entries: &mut Vec<walkdir::DirEntry>, config: &StreamConfig) {
    match config.sort_by {
        SortField::Name => {
            entries.sort_by(|a, b| {
                let a_is_dir = a.path().is_dir();
                let b_is_dir = b.path().is_dir();

                if a_is_dir && !b_is_dir {
                    return std::cmp::Ordering::Less;
                }
                if !a_is_dir && b_is_dir {
                    return std::cmp::Ordering::Greater;
                }

                a.file_name().cmp(b.file_name())
            });
        }
        SortField::Size => {
            entries.sort_by(|a, b| {
                let a_is_dir = a.path().is_dir();
                let b_is_dir = b.path().is_dir();

                if a_is_dir && !b_is_dir {
                    return std::cmp::Ordering::Less;
                }
                if !a_is_dir && b_is_dir {
                    return std::cmp::Ordering::Greater;
                }

                let a_size = a.metadata().map(|m| m.len()).unwrap_or(0);
                let b_size = b.metadata().map(|m| m.len()).unwrap_or(0);
                b_size.cmp(&a_size)
            });
        }
        SortField::Type => {
            entries.sort_by(|a, b| {
                let a_is_dir = a.path().is_dir();
                let b_is_dir = b.path().is_dir();

                if a_is_dir && !b_is_dir {
                    return std::cmp::Ordering::Less;
                }
                if !a_is_dir && b_is_dir {
                    return std::cmp::Ordering::Greater;
                }

                let a_ext = a.path().extension().and_then(|s| s.to_str()).unwrap_or("");
                let b_ext = b.path().extension().and_then(|s| s.to_str()).unwrap_or("");

                a_ext.cmp(b_ext)
                    .then_with(|| a.file_name().cmp(b.file_name()))
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_stream_config_default() {
        let config = StreamConfig::default();
        assert_eq!(config.max_depth, 0);
        assert!(!config.show_hidden);
    }

    #[test]
    fn test_walk_streaming() {
        let temp = TempDir::new().unwrap();
        let config = StreamConfig::default();

        let result = walk_streaming(temp.path(), config, |_| {});
        assert!(result.is_ok());
    }
}
