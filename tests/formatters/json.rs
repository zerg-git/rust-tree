//! `formatters::json`（JSON 输出）的测试。

use rust_tree::formatters::json::format_tree_only;
use rust_tree::{format_json, FsNode, FsNodeType, FsTree, TreeStats};
use std::time::Duration;

#[test]
fn test_format_json() {
    let root = FsNode::new("test".into(), "/test".into(), FsNodeType::Directory, 0, 0);
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
    let root = FsNode::new("test".into(), "/test".into(), FsNodeType::Directory, 0, 0);
    let tree = FsTree::new(root, 0);

    let json = format_tree_only(&tree, true).unwrap();
    assert!(json.contains("\"name\": \"test\""));
}
