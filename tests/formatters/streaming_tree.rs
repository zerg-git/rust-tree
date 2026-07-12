//! `formatters::streaming_tree`（制表符前缀构建器）的测试。

use rust_tree::formatters::streaming_tree::build_prefix;

// prefix_stack[d] = 路径上深度为 d 的节点的 is_last 标志。
// 索引 0 未使用（根节点单独绘制）；子节点从深度 1 开始。

#[test]
fn test_build_prefix() {
    // 深度为 1 的节点，且不是其父节点的最后一个子节点。
    let prefix_stack = vec![false, false];
    let prefix = build_prefix(&prefix_stack, 1);
    assert_eq!(prefix, "├── ");
}

#[test]
fn test_build_prefix_last() {
    // 深度为 1 的节点，且是其父节点的最后一个子节点。
    let prefix_stack = vec![false, true];
    let prefix = build_prefix(&prefix_stack, 1);
    assert_eq!(prefix, "└── ");
}

#[test]
fn test_build_prefix_nested() {
    // 深度为 2 的节点，最后一个子节点；其深度为 1 的祖先不是最后一个 => "│   "。
    let prefix_stack = vec![false, false, true];
    let prefix = build_prefix(&prefix_stack, 2);
    assert_eq!(prefix, "│   └── ");
}

#[test]
fn test_build_prefix_nested_ancestor_last() {
    // 深度为 2 的节点，最后一个子节点；其深度为 1 的祖先是最后一个 => "    "。
    let prefix_stack = vec![false, true, true];
    let prefix = build_prefix(&prefix_stack, 2);
    assert_eq!(prefix, "    └── ");
}
