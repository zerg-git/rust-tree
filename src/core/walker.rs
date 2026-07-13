//! 内存树构建器。
//!
//! 本模块拥有 `WalkConfig` / `SortField` 配置以及 `walk_directory` 入口。
//! 遍历、排序和过滤全部位于 `crate::core::streaming::walk_core` 之中；
//! `walk_directory` 只是该流的一个轻量消费者，为需要整棵树常驻内存的调用者
//! （JSON、统计信息、最大文件）物化出一棵 `FsTree`。

use crate::core::filter::FilterConfig;
use crate::core::models::{FsNode, FsNodeType, FsTree, TreeError};
use crate::core::streaming::walk_core;
use std::path::Path;

/// 目录遍历的配置。由内存树构建器和流式格式化器共享。
#[derive(Debug, Clone)]
pub struct WalkConfig {
    /// 遍历的最大深度（0 表示不限）
    pub max_depth: usize,
    /// 显示隐藏文件（以 . 开头）
    pub show_hidden: bool,
    /// 跟随符号链接
    pub follow_symlinks: bool,
    /// 按字段排序
    pub sort_by: SortField,
    /// 反转排序顺序
    pub reverse: bool,
    /// 过滤器配置
    pub filter: FilterConfig,
    /// 是否需要文件的字节大小。
    ///
    /// 为 false 时遍历核心会跳过对文件的 `stat` 调用（size 置 0），
    /// 适用于流式输出且不显示 size 的场景。`sort_by == Size` 总是隐式需要 size，
    /// 由遍历核心内部兜底，无需调用者在此置位。
    pub need_size: bool,
}

/// 目录条目的排序字段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    /// 按名称排序（默认）
    Name,
    /// 按文件大小排序
    Size,
    /// 按文件类型/扩展名排序
    Type,
}

impl Default for WalkConfig {
    fn default() -> Self {
        Self {
            max_depth: 0, // 不限
            show_hidden: false,
            follow_symlinks: false,
            sort_by: SortField::Name,
            reverse: false,
            filter: FilterConfig::default(),
            need_size: true,
        }
    }
}

/// 遍历一个目录并构建完整的内存文件树。
///
/// # 错误
///
/// 如果路径不存在、不是目录，或在根节点上权限被拒绝，则返回 `TreeError`。
pub fn walk_directory(
    path: &Path,
    config: &WalkConfig,
    progress: Option<&indicatif::ProgressBar>,
) -> Result<FsTree, TreeError> {
    if !path.exists() {
        return Err(TreeError::PathNotFound(path.to_path_buf()));
    }

    let meta = std::fs::metadata(path)?;
    if !meta.is_dir() {
        return Err(TreeError::NotADirectory(path.to_path_buf()));
    }

    let root_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".")
        .to_string();

    // 打开目录的栈帧栈；stack[0] 始终是根节点。一个栈帧在被弹出时会挂接到
    // 其父节点上，而弹出恰好发生在下一个兄弟节点（或叔伯节点）到达时——
    // 从而保持流（已排序）的顺序。
    let mut stack: Vec<FsNode> = vec![FsNode::new_directory(
        root_name,
        path.to_path_buf(),
        0,
        Vec::new(),
    )];
    let mut max_depth = 0usize;

    walk_core(path, config, |node| {
        if node.depth > max_depth {
            max_depth = node.depth;
        }

        // 关闭所有比该节点父节点更深的栈帧。
        while stack.len() > node.depth {
            let finished = stack.pop().unwrap();
            attach(&mut stack, finished);
        }

        match node.node_type {
            FsNodeType::Directory => {
                stack.push(FsNode::new_directory(
                    node.name.clone(),
                    node.path.clone(),
                    node.depth,
                    Vec::new(),
                ));
            }
            _ => {
                let leaf = FsNode::new(
                    node.name.clone(),
                    node.path.clone(),
                    node.node_type.clone(),
                    node.size,
                    node.depth,
                );
                if let Some(parent) = stack.last_mut() {
                    parent.children.get_or_insert_with(Vec::new).push(leaf);
                }
            }
        }

        // 真实进度：每个节点计数加一，目录节点更新当前路径消息。
        if let Some(pb) = progress {
            pb.inc(1);
            if node.node_type == FsNodeType::Directory {
                pb.set_message(node.path.display().to_string());
            }
        }
    })?;

    // 关闭所有剩余栈帧，直到根节点。
    while stack.len() > 1 {
        let finished = stack.pop().unwrap();
        attach(&mut stack, finished);
    }

    let mut root = stack.pop().unwrap();
    normalize_empty_children(&mut root);

    Ok(FsTree::new(root, max_depth))
}

/// 将一个已完成的节点挂接到其父节点（当前栈顶）上。
fn attach(stack: &mut [FsNode], mut finished: FsNode) {
    normalize_empty_children(&mut finished);
    if let Some(parent) = stack.last_mut() {
        parent.children.get_or_insert_with(Vec::new).push(finished);
    }
}

/// 没有子节点的目录其 `children == None`（而非 `Some([])`），
/// 这与叶子/空目录在其他地方的表示方式保持一致。
fn normalize_empty_children(node: &mut FsNode) {
    if let Some(children) = &node.children {
        if children.is_empty() {
            node.children = None;
        }
    }
}
