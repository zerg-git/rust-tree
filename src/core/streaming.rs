//! Single traversal core.
//!
//! `walk_core` is the one place directory traversal, sorting, and filtering
//! live. It emits a depth-first, pre-order stream of `StreamNode`s via a
//! callback (root's direct children at depth 1). Both the streaming formatter
//! and the in-memory `FsTree` builder consume this stream, so there is no
//! second traversal implementation to keep in sync.
//!
//! Peak memory is O(widest directory): only one directory's entries are
//! buffered at a time for sorting — not the whole tree.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::core::models::{FsNodeType, TreeError};
use crate::core::walker::{SortField, WalkConfig};

/// A node emitted by the traversal core.
#[derive(Debug, Clone)]
pub struct StreamNode {
    pub name: String,
    pub path: PathBuf,
    pub node_type: FsNodeType,
    pub size: u64,
    pub depth: usize,
    /// True if this is the last child of its parent (for tree drawing).
    pub is_last: bool,
}

/// A directory entry after a single stat, reused for sorting and emission.
struct Scanned {
    name: String,
    path: PathBuf,
    node_type: FsNodeType,
    size: u64,
}

/// Walk a directory tree, emitting each descendant node exactly once.
///
/// The callback receives nodes in depth-first pre-order. Root's direct
/// children are at depth 1; the root itself is not emitted (callers render or
/// build it themselves).
pub fn walk_core<F>(root: &Path, config: &WalkConfig, mut callback: F) -> Result<(), TreeError>
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

    walk_children(root, 1, config, &mut callback);
    Ok(())
}

/// Recursively emit the children of `dir` at the given `depth`.
fn walk_children<F>(dir: &Path, depth: usize, config: &WalkConfig, callback: &mut F)
where
    F: FnMut(&StreamNode),
{
    // Depth limit: children at depth D are emitted iff D <= max_depth. This
    // matches `depth >= max_depth => no children` on the parent side.
    if config.max_depth > 0 && depth > config.max_depth {
        return;
    }

    let mut scanned: Vec<Scanned> = Vec::new();

    let walker = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .follow_links(config.follow_symlinks)
        .into_iter();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        // file_type() is cached from readdir — no extra syscall.
        let file_type = entry.file_type();
        let is_dir = file_type.is_dir();

        if config.filter.should_exclude(entry.path(), is_dir) {
            continue;
        }

        let node_type = if file_type.is_symlink() {
            FsNodeType::Symlink
        } else if is_dir {
            FsNodeType::Directory
        } else {
            FsNodeType::File
        };

        // Only files need a size, so only files pay for a stat.
        let size = if node_type == FsNodeType::File {
            entry.metadata().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        scanned.push(Scanned {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_path_buf(),
            node_type,
            size,
        });
    }

    sort_scanned(&mut scanned, config);

    let total = scanned.len();
    for (i, item) in scanned.into_iter().enumerate() {
        let is_last = i + 1 == total;
        let is_dir = item.node_type == FsNodeType::Directory;
        let path = item.path.clone();

        callback(&StreamNode {
            name: item.name,
            path: item.path,
            node_type: item.node_type,
            size: item.size,
            depth,
            is_last,
        });

        if is_dir {
            walk_children(&path, depth + 1, config, callback);
        }
    }
}

/// File extension (without the dot) used for type sorting.
fn ext_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(idx) if idx > 0 => &name[idx + 1..],
        _ => "",
    }
}

/// Sort scanned entries: directories first, then by the configured field.
fn sort_scanned(entries: &mut [Scanned], config: &WalkConfig) {
    let dir_first = |a: &Scanned, b: &Scanned| {
        let a_dir = a.node_type == FsNodeType::Directory;
        let b_dir = b.node_type == FsNodeType::Directory;
        match (a_dir, b_dir) {
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    };

    match config.sort_by {
        SortField::Name => entries.sort_by(|a, b| {
            dir_first(a, b).unwrap_or_else(|| a.name.cmp(&b.name))
        }),
        SortField::Size => entries.sort_by(|a, b| {
            dir_first(a, b).unwrap_or_else(|| b.size.cmp(&a.size))
        }),
        SortField::Type => entries.sort_by(|a, b| {
            dir_first(a, b).unwrap_or_else(|| {
                ext_of(&a.name)
                    .cmp(ext_of(&b.name))
                    .then_with(|| a.name.cmp(&b.name))
            })
        }),
    }

    if config.reverse {
        entries.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_walk_core_empty() {
        let temp = TempDir::new().unwrap();
        let config = WalkConfig::default();

        let mut count = 0;
        let result = walk_core(temp.path(), &config, |_| count += 1);
        assert!(result.is_ok());
        assert_eq!(count, 0);
    }

    #[test]
    fn test_walk_core_children_start_at_depth_one() {
        let temp = TempDir::new().unwrap();
        std::fs::create_dir(temp.path().join("sub")).unwrap();
        std::fs::write(temp.path().join("sub/inner.txt"), b"hi").unwrap();

        let config = WalkConfig::default();
        let mut depths = Vec::new();
        walk_core(temp.path(), &config, |n| depths.push((n.name.clone(), n.depth))).unwrap();

        assert!(depths.contains(&("sub".to_string(), 1)));
        assert!(depths.contains(&("inner.txt".to_string(), 2)));
    }

    #[test]
    fn test_walk_core_max_depth_matches_walker() {
        let temp = TempDir::new().unwrap();
        std::fs::create_dir(temp.path().join("sub")).unwrap();
        std::fs::write(temp.path().join("sub/inner.txt"), b"hi").unwrap();

        // max_depth 1 => only depth-1 nodes, no grandchildren.
        let config = WalkConfig { max_depth: 1, ..Default::default() };
        let mut names = Vec::new();
        walk_core(temp.path(), &config, |n| names.push(n.name.clone())).unwrap();

        assert!(names.contains(&"sub".to_string()));
        assert!(!names.contains(&"inner.txt".to_string()));
    }
}
