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

#[test]
fn test_find_largest_files_zero_limit() {
    let files = [FsNode::new(
        "a.txt".into(),
        "/a.txt".into(),
        FsNodeType::File,
        100,
        0,
    )];
    let refs: Vec<&FsNode> = files.iter().collect();
    assert!(find_largest_files(&refs, 0).is_empty());
}

#[test]
fn test_find_largest_files_limit_exceeds_count() {
    // limit 超过文件数时应返回全部并保持降序
    let files = [
        FsNode::new("a.txt".into(), "/a.txt".into(), FsNodeType::File, 100, 0),
        FsNode::new("b.txt".into(), "/b.txt".into(), FsNodeType::File, 500, 0),
    ];
    let refs: Vec<&FsNode> = files.iter().collect();
    let largest = find_largest_files(&refs, 10);

    assert_eq!(largest.len(), 2);
    assert_eq!(largest[0].size, 500);
    assert_eq!(largest[1].size, 100);
}

#[test]
fn test_analyze_by_extension_ignores_dotfiles() {
    // 点文件应归入“(no extension)”，而非被当成扩展名 ".gitignore"
    let files = [
        FsNode::new(
            ".gitignore".into(),
            "/.gitignore".into(),
            FsNodeType::File,
            10,
            0,
        ),
        FsNode::new("a.txt".into(), "/a.txt".into(), FsNodeType::File, 40, 0),
        FsNode::new("b.txt".into(), "/b.txt".into(), FsNodeType::File, 60, 0),
    ];

    let refs: Vec<&FsNode> = files.iter().collect();
    let by_ext = analyze_by_extension(&refs, 110);

    // ".txt" 一桶，点文件归入 "(no extension)" 一桶；未产生 ".gitignore" 分类
    assert_eq!(by_ext.len(), 2);
    assert_eq!(by_ext.get(".txt").unwrap().count, 2);
    assert_eq!(by_ext.get("(no extension)").unwrap().count, 1);
    assert!(by_ext.get(".gitignore").is_none());
}
