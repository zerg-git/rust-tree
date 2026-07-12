//! JSON 输出格式化器。

use serde_json::json;
use crate::core::models::{FsTree, TreeStats, TreeError};

/// 将文件树及其统计信息格式化为 JSON。
///
/// # 参数
///
/// * `tree` - 要格式化的文件系统树
/// * `stats` - 要包含在输出中的统计信息
/// * `pretty` - 是否美化打印 JSON
///
/// # 返回
///
/// 表示树和统计信息的 JSON 字符串。
///
/// # 错误
///
/// 如果序列化失败，返回 `TreeError::Json`。
pub fn format_json(tree: &FsTree, stats: &TreeStats, pretty: bool) -> Result<String, TreeError> {
    let output = json!({
        "tree": {
            "root": tree.root,
            "max_depth": tree.max_depth
        },
        "stats": {
            "total_files": stats.total_files,
            "total_directories": stats.total_directories,
            "total_symlinks": stats.total_symlinks,
            "total_size": stats.total_size,
            "files_by_extension": stats.files_by_extension,
            "largest_files": stats.largest_files,
            "scan_duration_ms": stats.scan_duration.as_millis()
        }
    });

    if pretty {
        serde_json::to_string_pretty(&output).map_err(TreeError::from)
    } else {
        serde_json::to_string(&output).map_err(TreeError::from)
    }
}

/// 仅将树结构格式化为 JSON（不含统计信息）。
///
/// # 参数
///
/// * `tree` - 要格式化的文件系统树
/// * `pretty` - 是否美化打印 JSON
///
/// # 返回
///
/// 仅表示树结构的 JSON 字符串。
///
/// # 错误
///
/// 如果序列化失败，返回 `TreeError::Json`。
pub fn format_tree_only(tree: &FsTree, pretty: bool) -> Result<String, TreeError> {
    if pretty {
        serde_json::to_string_pretty(&tree.root).map_err(TreeError::from)
    } else {
        serde_json::to_string(&tree.root).map_err(TreeError::from)
    }
}

/// 仅将统计信息格式化为 JSON。
///
/// # 参数
///
/// * `stats` - 要格式化的统计信息
/// * `pretty` - 是否美化打印 JSON
///
/// # 返回
///
/// 仅表示统计信息的 JSON 字符串。
///
/// # 错误
///
/// 如果序列化失败，返回 `TreeError::Json`。
pub fn format_stats_only(stats: &TreeStats, pretty: bool) -> Result<String, TreeError> {
    if pretty {
        serde_json::to_string_pretty(&stats).map_err(TreeError::from)
    } else {
        serde_json::to_string(&stats).map_err(TreeError::from)
    }
}
