//! `core::collector`（统计聚合辅助函数）的测试。

use rust_tree::core::collector::{analyze_by_extension, find_largest_files};
use rust_tree::{FsNode, FsNodeType};

#[test]
fn test_find_largest_files() {
    let files = [
        FsNode::new("a.txt".into(), "/a.txt".into(), FsNodeType::File, 100, 0),
        FsNode::new("b.txt".into(), "/b.txt".into(), FsNodeType::File, 500, 0),
        FsNode::new("c.txt".into(), "/c.txt".into(), FsNodeType::File, 200, 0),
        FsNode::new("d.txt".into(), "/d.txt".into(), FsNodeType::File, 1000, 0),
        FsNode::new("e.txt".into(), "/e.txt".into(), FsNodeType::File, 50, 0),
    ];

    let refs: Vec<&FsNode> = files.iter().collect();
    let largest = find_largest_files(&refs, 3);

    assert_eq!(largest.len(), 3);
    assert_eq!(largest[0].size, 1000);
    assert_eq!(largest[1].size, 500);
    assert_eq!(largest[2].size, 200);
}

#[test]
fn test_analyze_by_extension() {
    let files = [
        FsNode::new(
            "file.rs".into(),
            "/file.rs".into(),
            FsNodeType::File,
            100,
            0,
        ),
        FsNode::new("doc.md".into(), "/doc.md".into(), FsNodeType::File, 50, 0),
        FsNode::new(
            "main.rs".into(),
            "/main.rs".into(),
            FsNodeType::File,
            200,
            0,
        ),
    ];

    let refs: Vec<&FsNode> = files.iter().collect();
    let by_ext = analyze_by_extension(&refs, 350);

    assert_eq!(by_ext.len(), 2);
    assert_eq!(by_ext.get(".rs").unwrap().count, 2);
    assert_eq!(by_ext.get(".md").unwrap().count, 1);
}
