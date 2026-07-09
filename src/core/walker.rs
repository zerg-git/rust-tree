//! In-memory tree builder.
//!
//! This module owns the `WalkConfig` / `SortField` configuration and the
//! `walk_directory` entry point. Traversal, sorting, and filtering all live in
//! `crate::core::streaming::walk_core`; `walk_directory` is a thin consumer of
//! that stream that materializes an `FsTree` for callers needing the whole tree
//! in memory (JSON, statistics, largest-files).

use std::path::Path;
use crate::core::models::{FsNode, FsTree, FsNodeType, TreeError};
use crate::core::filter::FilterConfig;
use crate::core::streaming::walk_core;

/// Configuration for directory walking. Shared by both the in-memory builder
/// and the streaming formatter.
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

/// Walk a directory and build a complete in-memory file tree.
///
/// # Errors
///
/// Returns `TreeError` if the path doesn't exist, isn't a directory,
/// or permission is denied on the root.
pub fn walk_directory(path: &Path, config: &WalkConfig) -> Result<FsTree, TreeError> {
    if !path.exists() {
        return Err(TreeError::PathNotFound(path.to_path_buf()));
    }

    let meta = std::fs::metadata(path)?;
    if !meta.is_dir() {
        return Err(TreeError::NotADirectory(path.to_path_buf()));
    }

    let root_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".")
        .to_string();

    // Stack of open directory frames; stack[0] is always the root. A frame is
    // attached to its parent when it is popped, which happens exactly when the
    // next sibling (or uncle) arrives — preserving stream (sorted) order.
    let mut stack: Vec<FsNode> =
        vec![FsNode::new_directory(root_name, path.to_path_buf(), 0, Vec::new())];
    let mut max_depth = 0usize;

    walk_core(path, config, |node| {
        if node.depth > max_depth {
            max_depth = node.depth;
        }

        // Close every frame deeper than this node's parent.
        while stack.len() > node.depth {
            let finished = stack.pop().unwrap();
            attach(&mut stack, finished);
        }

        match node.node_type {
            FsNodeType::Directory => {
                stack.push(FsNode::new_directory(
                    node.name.clone(),
                    node.path.clone(),
                    node.depth,
                    Vec::new(),
                ));
            }
            _ => {
                let leaf = FsNode::new(
                    node.name.clone(),
                    node.path.clone(),
                    node.node_type.clone(),
                    node.size,
                    node.depth,
                );
                if let Some(parent) = stack.last_mut() {
                    parent.children.get_or_insert_with(Vec::new).push(leaf);
                }
            }
        }
    })?;

    // Close all remaining frames down to the root.
    while stack.len() > 1 {
        let finished = stack.pop().unwrap();
        attach(&mut stack, finished);
    }

    let mut root = stack.pop().unwrap();
    normalize_empty_children(&mut root);

    Ok(FsTree::new(root, max_depth))
}

/// Attach a finished node to its parent (the current stack top).
fn attach(stack: &mut [FsNode], mut finished: FsNode) {
    normalize_empty_children(&mut finished);
    if let Some(parent) = stack.last_mut() {
        parent
            .children
            .get_or_insert_with(Vec::new)
            .push(finished);
    }
}

/// A directory with no children carries `children == None` (not `Some([])`),
/// matching how leaf/empty directories are represented elsewhere.
fn normalize_empty_children(node: &mut FsNode) {
    if let Some(children) = &node.children {
        if children.is_empty() {
            node.children = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_walk_config_default() {
        let config = WalkConfig::default();
        assert_eq!(config.max_depth, 0);
        assert!(!config.show_hidden);
        assert!(!config.follow_symlinks);
    }

    #[test]
    fn test_walk_directory_builds_tree() {
        let temp = TempDir::new().unwrap();
        std::fs::create_dir(temp.path().join("sub")).unwrap();
        std::fs::write(temp.path().join("sub/inner.txt"), b"hi").unwrap();
        std::fs::write(temp.path().join("top.txt"), b"hello").unwrap();

        let tree = walk_directory(temp.path(), &WalkConfig::default()).unwrap();

        let children = tree.root.children.as_ref().unwrap();
        // Directory first, then file.
        assert_eq!(children[0].name, "sub");
        assert!(children[0].is_directory());
        let inner = children[0].children.as_ref().unwrap();
        assert_eq!(inner[0].name, "inner.txt");
        assert!(children.iter().any(|c| c.name == "top.txt"));
        assert_eq!(tree.max_depth, 2);
    }

    #[test]
    fn test_walk_directory_max_depth() {
        let temp = TempDir::new().unwrap();
        std::fs::create_dir(temp.path().join("sub")).unwrap();
        std::fs::write(temp.path().join("sub/inner.txt"), b"hi").unwrap();

        // max_depth 1: "sub" appears but its children are pruned.
        let config = WalkConfig { max_depth: 1, ..Default::default() };
        let tree = walk_directory(temp.path(), &config).unwrap();
        let sub = tree
            .root
            .children
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.name == "sub")
            .unwrap();
        assert!(sub.children.is_none());
        assert_eq!(tree.max_depth, 1);
    }

    #[test]
    fn test_walk_directory_empty() {
        let temp = TempDir::new().unwrap();
        let tree = walk_directory(temp.path(), &WalkConfig::default()).unwrap();
        assert!(tree.root.children.is_none());
        assert_eq!(tree.max_depth, 0);
    }
}
