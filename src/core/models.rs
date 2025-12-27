//! Core data structures for representing file system trees and statistics.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};

/// File system node type classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FsNodeType {
    /// Regular file
    #[serde(rename = "file")]
    File,
    /// Directory
    #[serde(rename = "directory")]
    Directory,
    /// Symbolic link
    #[serde(rename = "symlink")]
    Symlink,
}

/// A node in the file system tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsNode {
    /// Name of the file or directory
    pub name: String,

    /// Full path to the file or directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,

    /// Type of the file system node
    #[serde(rename = "type")]
    pub node_type: FsNodeType,

    /// Size in bytes (0 for directories)
    pub size: u64,

    /// Depth in the tree (0 for root)
    pub depth: usize,

    /// Child nodes (only for directories)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FsNode>>,
}

impl FsNode {
    /// Create a new file system node.
    pub fn new(
        name: String,
        path: PathBuf,
        node_type: FsNodeType,
        size: u64,
        depth: usize,
    ) -> Self {
        Self {
            name,
            path: Some(path),
            node_type,
            size,
            depth,
            children: None,
        }
    }

    /// Create a new directory node with children.
    pub fn new_directory(
        name: String,
        path: PathBuf,
        depth: usize,
        children: Vec<FsNode>,
    ) -> Self {
        Self {
            name,
            path: Some(path),
            node_type: FsNodeType::Directory,
            size: 0,
            depth,
            children: Some(children),
        }
    }

    /// Check if this node is a directory.
    pub fn is_directory(&self) -> bool {
        self.node_type == FsNodeType::Directory
    }

    /// Check if this node is a file.
    pub fn is_file(&self) -> bool {
        self.node_type == FsNodeType::File
    }

    /// Check if this node is a symlink.
    pub fn is_symlink(&self) -> bool {
        self.node_type == FsNodeType::Symlink
    }

    /// Get the file extension (if any).
    pub fn extension(&self) -> Option<String> {
        if self.is_directory() {
            return None;
        }
        self.name
            .rsplit('.')
            .next()
            .filter(|ext| !ext.is_empty() && self.name.contains('.'))
            .map(|s| format!(".{}", s))
    }
}

/// A file system tree representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsTree {
    /// Root node of the tree
    pub root: FsNode,

    /// Maximum depth of the tree
    pub max_depth: usize,
}

impl FsTree {
    /// Create a new file system tree.
    pub fn new(root: FsNode, max_depth: usize) -> Self {
        Self { root, max_depth }
    }
}

/// Information about a specific file type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeInfo {
    /// File extension (e.g., ".rs", ".txt")
    pub extension: String,

    /// Number of files with this extension
    pub count: usize,

    /// Total size of all files with this extension
    pub total_size: u64,

    /// Percentage of total size
    pub percentage: f64,
}

/// A file entry for sorted listings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileEntry {
    /// File name
    pub name: String,

    /// File path
    pub path: PathBuf,

    /// File size in bytes
    pub size: u64,
}

impl FileEntry {
    /// Create a new file entry.
    pub fn new(name: String, path: PathBuf, size: u64) -> Self {
        Self { name, path, size }
    }
}

/// Statistics collected from scanning a directory tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeStats {
    /// Total number of files
    pub total_files: usize,

    /// Total number of directories
    pub total_directories: usize,

    /// Total number of symbolic links
    pub total_symlinks: usize,

    /// Total size of all files in bytes
    pub total_size: u64,

    /// Files grouped by extension
    pub files_by_extension: HashMap<String, FileTypeInfo>,

    /// Largest files (top N)
    pub largest_files: Vec<FileEntry>,

    /// Time taken to scan the directory
    pub scan_duration: Duration,
}

impl TreeStats {
    /// Create a new empty statistics object.
    pub fn new() -> Self {
        Self {
            total_files: 0,
            total_directories: 0,
            total_symlinks: 0,
            total_size: 0,
            files_by_extension: HashMap::new(),
            largest_files: Vec::new(),
            scan_duration: Duration::default(),
        }
    }
}

impl Default for TreeStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for tree operations.
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    /// IO error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Path does not exist
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    /// Not a directory
    #[error("Not a directory: {0}")]
    NotADirectory(PathBuf),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    Json(String),

    /// Generic error message
    #[error("{0}")]
    Other(String),
}

// Convert serde_json error to our TreeError
impl From<serde_json::Error> for TreeError {
    fn from(err: serde_json::Error) -> Self {
        TreeError::Json(err.to_string())
    }
}
