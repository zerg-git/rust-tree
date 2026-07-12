//! `core::walker`（内存中的树构建器）的测试。

use rust_tree::{walk_directory, WalkConfig};
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
    // 目录在前，文件在后。
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

    // max_depth 1："sub" 会出现，但其子节点被剪枝。
    let config = WalkConfig {
        max_depth: 1,
        ..Default::default()
    };
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
