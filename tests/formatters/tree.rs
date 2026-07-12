//! `formatters::tree`（Unicode 树状输出）的测试。

use rust_tree::formatters::tree::format_size_impl;
use rust_tree::{format_tree, ColorMode, ColorScheme, FsNode, FsNodeType};

#[test]
fn test_format_tree_simple() {
    let file1 = FsNode::new(
        "file.txt".into(),
        "/test/file.txt".into(),
        FsNodeType::File,
        1024,
        1,
    );
    let mut dir1 = FsNode::new(
        "subdir".into(),
        "/test/subdir".into(),
        FsNodeType::Directory,
        0,
        1,
    );
    dir1.children = Some(vec![]);

    let mut root = FsNode::new("root".into(), "/test".into(), FsNodeType::Directory, 0, 0);
    root.children = Some(vec![dir1, file1]);

    let output = format_tree(&root, false, ColorMode::Never, ColorScheme::None);

    assert!(output.contains("root/"));
    assert!(output.contains("subdir/"));
    assert!(output.contains("file.txt"));
}

#[test]
fn test_format_size() {
    // humansize 使用 "KiB" 而非 "KB"
    let s1 = format_size_impl(1024);
    assert!(s1.contains("K") || s1.contains("k"));
    let s2 = format_size_impl(1048576);
    assert!(s2.contains("M") || s2.contains("m"));
}
