//! 从文件系统树中收集统计信息。

use std::collections::HashMap;
use std::time::Instant;
use crate::core::models::{FsTree, FsNode, TreeStats, FileTypeInfo, FileEntry};

/// 从文件系统树中收集统计信息。
///
/// # 参数
///
/// * `tree` - 待分析的文件系统树
/// * `start_time` - 扫描开始的时刻（用于计算耗时）
/// * `largest_limit` - 保留多少个最大文件（来自 `--top-files`）
///
/// # 返回
///
/// 一个包含所有已收集统计信息的 `TreeStats` 对象。
pub fn collect_stats(tree: &FsTree, start_time: Instant, largest_limit: usize) -> TreeStats {
    let mut stats = TreeStats::new();

    // 收集所有文件和目录
    let mut all_files: Vec<&FsNode> = Vec::new();
    count_nodes(&tree.root, &mut stats, &mut all_files);

    // 按扩展名分组
    stats.files_by_extension = analyze_by_extension(&all_files, stats.total_size);

    // 查找最大的文件
    stats.largest_files = find_largest_files(&all_files, largest_limit);

    // 计算扫描耗时
    stats.scan_duration = start_time.elapsed();

    stats
}

/// 递归地统计树中节点的数量。
fn count_nodes<'a>(node: &'a FsNode, stats: &mut TreeStats, all_files: &mut Vec<&'a FsNode>) {
    match node.node_type {
        crate::core::models::FsNodeType::Directory => {
            stats.total_directories += 1;
        }
        crate::core::models::FsNodeType::File => {
            stats.total_files += 1;
            stats.total_size += node.size;
            all_files.push(node);
        }
        crate::core::models::FsNodeType::Symlink => {
            stats.total_symlinks += 1;
        }
    }

    if let Some(children) = &node.children {
        for child in children {
            count_nodes(child, stats, all_files);
        }
    }
}

/// 按扩展名分析文件。
///
/// 返回一个将扩展名映射到文件类型信息的 HashMap。
#[doc(hidden)]
pub fn analyze_by_extension(files: &[&FsNode], total_size: u64) -> HashMap<String, FileTypeInfo> {
    let mut by_ext: HashMap<String, (usize, u64)> = HashMap::new();

    for file in files {
        let ext = file.extension().unwrap_or_else(|| "(no extension)".to_string());

        let entry = by_ext.entry(ext).or_insert((0, 0));
        entry.0 += 1;  // 数量
        entry.1 += file.size;  // 总大小
    }

    // 转换为带百分比的 FileTypeInfo
    by_ext.into_iter()
        .map(|(ext, (count, size))| {
            let percentage = if total_size > 0 {
                (size as f64 / total_size as f64) * 100.0
            } else {
                0.0
            };

            let info = FileTypeInfo {
                extension: ext.clone(),
                count,
                total_size: size,
                percentage,
            };

            (ext, info)
        })
        .collect()
}

/// 查找 N 个最大的文件。
///
/// # 参数
///
/// * `files` - 待分析的文件节点切片
/// * `limit` - 返回文件的最大数量
///
/// # 返回
///
/// 一个由 `FileEntry` 对象组成的向量，按大小排序（最大者在前）。
#[doc(hidden)]
pub fn find_largest_files(files: &[&FsNode], limit: usize) -> Vec<FileEntry> {
    if files.is_empty() {
        return Vec::new();
    }

    // 收集所有条目
    let mut entries: Vec<FileEntry> = files
        .iter()
        .map(|file| FileEntry::new(
            file.name.clone(),
            file.path.clone().unwrap_or_default(),
            file.size,
        ))
        .collect();

    // 按大小排序（降序）
    entries.sort_by_key(|e| std::cmp::Reverse(e.size));

    // 取前 N 个
    entries.truncate(limit);
    entries
}

/// 获取树中所有文件节点的扁平列表。
///
/// # 参数
///
/// * `tree` - 文件系统树
///
/// # 返回
///
/// 一个包含所有文件节点的向量。
pub fn get_all_files(tree: &FsTree) -> Vec<FsNode> {
    let mut files = Vec::new();
    collect_files_recursive(&tree.root, &mut files);
    files
}

/// 递归地收集文件节点。
fn collect_files_recursive(node: &FsNode, files: &mut Vec<FsNode>) {
    if node.is_file() {
        files.push(node.clone());
    }

    if let Some(children) = &node.children {
        for child in children {
            collect_files_recursive(child, files);
        }
    }
}

/// 获取树中所有目录节点的扁平列表。
///
/// # 参数
///
/// * `tree` - 文件系统树
///
/// # 返回
///
/// 一个包含所有目录节点的向量。
pub fn get_all_directories(tree: &FsTree) -> Vec<FsNode> {
    let mut dirs = Vec::new();
    collect_dirs_recursive(&tree.root, &mut dirs);
    dirs
}

/// 递归地收集目录节点。
fn collect_dirs_recursive(node: &FsNode, dirs: &mut Vec<FsNode>) {
    if node.is_directory() {
        dirs.push(node.clone());
    }

    if let Some(children) = &node.children {
        for child in children {
            collect_dirs_recursive(child, dirs);
        }
    }
}

/// 计算树中节点的总数。
pub fn total_node_count(tree: &FsTree) -> usize {
    count_nodes_recursive(&tree.root)
}

/// 递归地统计所有节点。
fn count_nodes_recursive(node: &FsNode) -> usize {
    let mut count = 1;

    if let Some(children) = &node.children {
        for child in children {
            count += count_nodes_recursive(child);
        }
    }

    count
}
