//! `config::color`（节点着色）的测试。

use rust_tree::config::color::colorize_node;
use rust_tree::{ColorScheme, FsNode, FsNodeType};

#[test]
fn test_colorize_directory() {
    let node = FsNode::new(
        "test_dir".into(),
        "/test".into(),
        FsNodeType::Directory,
        0,
        0,
    );
    let colored = colorize_node(&node, ColorScheme::Basic);
    // 着色后的字符串应包含该名称
    assert!(colored.to_string().contains("test_dir"));
}

#[test]
fn test_colorize_file_by_extension() {
    let rust_file = FsNode::new(
        "main.rs".into(),
        "/test/main.rs".into(),
        FsNodeType::File,
        100,
        0,
    );
    let colored = colorize_node(&rust_file, ColorScheme::Basic);
    assert!(colored.to_string().contains("main.rs"));
}

#[test]
fn test_colorize_symlink() {
    let symlink = FsNode::new(
        "link".into(),
        "/test/link".into(),
        FsNodeType::Symlink,
        0,
        0,
    );
    let colored = colorize_node(&symlink, ColorScheme::Basic);
    assert!(colored.to_string().contains("link"));
}

#[test]
fn test_no_color_scheme() {
    let node = FsNode::new(
        "test.rs".into(),
        "/test.rs".into(),
        FsNodeType::File,
        100,
        0,
    );
    let colored = colorize_node(&node, ColorScheme::None);
    assert!(colored.to_string().contains("test.rs"));
}
