//! 统一的遍历核心。
//!
//! `walk_core` 是目录遍历、排序和过滤的唯一实现所在地。它通过回调
//! 输出一个深度优先、先序遍历的 `StreamNode` 流（根节点的直接子节点
//! 位于深度 1）。流式格式化器和内存中的 `FsTree` 构建器都消费这个流，
//! 因此不存在需要保持同步的第二套遍历实现。
//!
//! 峰值内存为 O(最宽目录)：每次只为排序而缓冲单个目录的条目——而非
//! 整棵树。

use crate::core::models::{FsNodeType, TreeError};
use crate::core::walker::{SortField, WalkConfig};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 遍历核心输出的节点。
#[derive(Debug, Clone)]
pub struct StreamNode {
    pub name: String,
    pub path: PathBuf,
    pub node_type: FsNodeType,
    pub size: u64,
    pub depth: usize,
    /// 若该节点是其父节点的最后一个子节点则为真（用于绘制树）。
    pub is_last: bool,
}

/// 经过一次 stat 调用后的目录条目，在排序和输出时被复用。
struct Scanned {
    name: String,
    path: PathBuf,
    node_type: FsNodeType,
    size: u64,
}

/// 遍历目录树，每个后代节点只输出一次。
///
/// 回调按深度优先的先序顺序接收节点。根节点的直接子节点位于深度 1；
/// 根节点本身不会被输出（由调用者自行渲染或构建）。
pub fn walk_core<F>(root: &Path, config: &WalkConfig, mut callback: F) -> Result<(), TreeError>
where
    F: FnMut(&StreamNode),
{
    if !root.exists() {
        return Err(TreeError::PathNotFound(root.to_path_buf()));
    }

    let meta = std::fs::metadata(root)?;
    if !meta.is_dir() {
        return Err(TreeError::NotADirectory(root.to_path_buf()));
    }

    walk_children(root, 1, config, &mut callback);
    Ok(())
}

/// 递归地输出 `dir` 在指定 `depth` 处的子节点。
fn walk_children<F>(dir: &Path, depth: usize, config: &WalkConfig, callback: &mut F)
where
    F: FnMut(&StreamNode),
{
    // 深度限制：深度 D 处的子节点当且仅当 D <= max_depth 时才会被输出。这与
    // 父节点侧的 `depth >= max_depth => 无子节点` 相对应。
    if config.max_depth > 0 && depth > config.max_depth {
        return;
    }

    let mut scanned: Vec<Scanned> = Vec::new();

    let walker = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .follow_links(config.follow_symlinks)
        .into_iter();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        // file_type() 由 readdir 缓存——无需额外系统调用。
        let file_type = entry.file_type();
        let is_dir = file_type.is_dir();

        if config.filter.should_exclude(entry.path(), is_dir) {
            continue;
        }

        let node_type = if file_type.is_symlink() {
            FsNodeType::Symlink
        } else if is_dir {
            FsNodeType::Directory
        } else {
            FsNodeType::File
        };

        // 只有当调用者需要 size（显示 size 或内存路径的统计）或按 size 排序时，
        // 才对文件付出一次 stat 调用的代价；否则跳过，size 置 0。
        let need = config.need_size || config.sort_by == SortField::Size;
        let size = if need && node_type == FsNodeType::File {
            entry.metadata().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        scanned.push(Scanned {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_path_buf(),
            node_type,
            size,
        });
    }

    sort_scanned(&mut scanned, config);

    let total = scanned.len();
    for (i, item) in scanned.into_iter().enumerate() {
        let is_last = i + 1 == total;
        let is_dir = item.node_type == FsNodeType::Directory;
        let path = item.path.clone();

        callback(&StreamNode {
            name: item.name,
            path: item.path,
            node_type: item.node_type,
            size: item.size,
            depth,
            is_last,
        });

        if is_dir {
            walk_children(&path, depth + 1, config, callback);
        }
    }
}

/// 用于按类型排序的文件扩展名（不含点号）。
fn ext_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(idx) if idx > 0 => &name[idx + 1..],
        _ => "",
    }
}

/// 对扫描到的条目排序：目录在前，然后按配置的字段排序。
fn sort_scanned(entries: &mut [Scanned], config: &WalkConfig) {
    let dir_first = |a: &Scanned, b: &Scanned| {
        let a_dir = a.node_type == FsNodeType::Directory;
        let b_dir = b.node_type == FsNodeType::Directory;
        match (a_dir, b_dir) {
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    };

    match config.sort_by {
        SortField::Name => {
            entries.sort_by(|a, b| dir_first(a, b).unwrap_or_else(|| a.name.cmp(&b.name)))
        }
        SortField::Size => {
            entries.sort_by(|a, b| dir_first(a, b).unwrap_or_else(|| b.size.cmp(&a.size)))
        }
        SortField::Type => entries.sort_by(|a, b| {
            dir_first(a, b).unwrap_or_else(|| {
                ext_of(&a.name)
                    .cmp(ext_of(&b.name))
                    .then_with(|| a.name.cmp(&b.name))
            })
        }),
    }

    if config.reverse {
        entries.reverse();
    }
}
