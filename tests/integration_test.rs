//! rust-tree 的集成测试

use std::fs::{self, File};
use std::io::Write;

/// 创建一个临时的测试目录结构
fn create_test_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();

    // 创建目录结构
    fs::create_dir_all(path.join("src/core")).unwrap();
    fs::create_dir_all(path.join("tests")).unwrap();

    // 创建一些文件
    File::create(path.join("Cargo.toml")).unwrap();
    File::create(path.join("README.md")).unwrap();
    File::create(path.join("src/main.rs")).unwrap();
    File::create(path.join("src/lib.rs")).unwrap();
    File::create(path.join("src/core/models.rs")).unwrap();

    // 写入一些内容
    let mut file = File::create(path.join("src/main.rs")).unwrap();
    file.write_all(b"fn main() { println!(\"Hello\"); }").unwrap();

    dir
}

#[test]
fn test_walk_directory() {
    let test_dir = create_test_dir();
    let config = rust_tree::core::walker::WalkConfig::default();

    let result = rust_tree::core::walker::walk_directory(test_dir.path(), &config, None);
    assert!(result.is_ok());

    let tree = result.unwrap();
    assert_eq!(tree.root.name, test_dir.path().file_name().unwrap().to_str().unwrap());
    assert!(tree.root.children.is_some());
}

#[test]
fn test_collect_stats() {
    let test_dir = create_test_dir();
    let config = rust_tree::core::walker::WalkConfig::default();

    let tree = rust_tree::core::walker::walk_directory(test_dir.path(), &config, None).unwrap();
    let stats = rust_tree::core::collector::collect_stats(&tree, std::time::Instant::now(), 10);

    assert!(stats.total_files > 0);
    assert!(stats.total_directories > 0);
    assert!(stats.total_size > 0);
}

#[test]
fn test_format_output() {
    let test_dir = create_test_dir();
    let config = rust_tree::Config {
        path: test_dir.path().to_path_buf(),
        ..Default::default()
    };

    let result = rust_tree::run(config);
    assert!(result.is_ok());
}
