//! Core functionality for directory traversal and statistics collection.

pub mod models;
pub mod walker;
pub mod collector;
pub mod progress;
pub mod filter;
pub mod streaming;

pub use models::{FsNode, FsTree, FsNodeType, TreeStats, FileTypeInfo, FileEntry, TreeError};
