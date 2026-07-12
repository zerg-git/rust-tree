//! `core::streaming`（唯一的遍历核心）的测试。

use rust_tree::core::streaming::walk_core;
use rust_tree::WalkConfig;
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
    walk_core(temp.path(), &config, |n| {
        depths.push((n.name.clone(), n.depth))
    })
    .unwrap();

    assert!(depths.contains(&("sub".to_string(), 1)));
    assert!(depths.contains(&("inner.txt".to_string(), 2)));
}

#[test]
fn test_walk_core_max_depth_matches_walker() {
    let temp = TempDir::new().unwrap();
    std::fs::create_dir(temp.path().join("sub")).unwrap();
    std::fs::write(temp.path().join("sub/inner.txt"), b"hi").unwrap();

    // max_depth 1：仅深度为 1 的节点，没有孙节点。
    let config = WalkConfig {
        max_depth: 1,
        ..Default::default()
    };
    let mut names = Vec::new();
    walk_core(temp.path(), &config, |n| names.push(n.name.clone())).unwrap();

    assert!(names.contains(&"sub".to_string()));
    assert!(!names.contains(&"inner.txt".to_string()));
}
