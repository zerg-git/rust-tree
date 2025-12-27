//! Core functionality for directory traversal and statistics collection.

pub mod models;
pub mod walker;
pub mod collector;

pub use models::{FsNode, FsTree, FsNodeType, TreeStats, FileTypeInfo, FileEntry, TreeError};
