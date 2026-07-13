//! 表示文件系统树和统计信息的核心数据结构。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// 文件系统节点类型分类。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FsNodeType {
    /// 常规文件
    #[serde(rename = "file")]
    File,
    /// 目录
    #[serde(rename = "directory")]
    Directory,
    /// 符号链接
    #[serde(rename = "symlink")]
    Symlink,
}

/// 文件系统树中的一个节点。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsNode {
    /// 文件或目录的名称
    pub name: String,

    /// 文件或目录的完整路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,

    /// 文件系统节点的类型
    #[serde(rename = "type")]
    pub node_type: FsNodeType,

    /// 字节大小（目录为 0）
    pub size: u64,

    /// 在树中的深度（根节点为 0）
    pub depth: usize,

    /// 子节点（仅用于目录）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FsNode>>,
}

impl FsNode {
    /// 创建一个新的文件系统节点。
    pub fn new(
        name: String,
        path: PathBuf,
        node_type: FsNodeType,
        size: u64,
        depth: usize,
    ) -> Self {
        Self {
            name,
            path: Some(path),
            node_type,
            size,
            depth,
            children: None,
        }
    }

    /// 创建一个带子节点的新目录节点。
    pub fn new_directory(name: String, path: PathBuf, depth: usize, children: Vec<FsNode>) -> Self {
        Self {
            name,
            path: Some(path),
            node_type: FsNodeType::Directory,
            size: 0,
            depth,
            children: Some(children),
        }
    }

    /// 检查该节点是否为目录。
    pub fn is_directory(&self) -> bool {
        self.node_type == FsNodeType::Directory
    }

    /// 检查该节点是否为文件。
    pub fn is_file(&self) -> bool {
        self.node_type == FsNodeType::File
    }

    /// 检查该节点是否为符号链接。
    pub fn is_symlink(&self) -> bool {
        self.node_type == FsNodeType::Symlink
    }

    /// 获取文件扩展名（如果有）。
    ///
    /// 点文件（如 `.gitignore`）和以点号结尾的名字（如 `file.`）视为无扩展名。
    /// 仅当最后一个点既不在首位也不在末位时，才取从该点开始的后缀（含点号）。
    pub fn extension(&self) -> Option<String> {
        if self.is_directory() {
            return None;
        }
        let name = &self.name;
        let pos = name.rfind('.')?;
        // pos == 0  ⇒ 点在首位（.gitignore），无扩展名；
        // pos == name.len()-1 ⇒ 点在末位（file.），无扩展名。
        if pos == 0 || pos == name.len() - 1 {
            return None;
        }
        Some(name[pos..].to_string())
    }
}

/// 文件系统树的表示。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsTree {
    /// 树的根节点
    pub root: FsNode,

    /// 树的最大深度
    pub max_depth: usize,
}

impl FsTree {
    /// 创建一棵新的文件系统树。
    pub fn new(root: FsNode, max_depth: usize) -> Self {
        Self { root, max_depth }
    }
}

/// 关于特定文件类型的信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeInfo {
    /// 文件扩展名（例如 ".rs"、".txt"）
    pub extension: String,

    /// 具有该扩展名的文件数量
    pub count: usize,

    /// 具有该扩展名的所有文件的总大小
    pub total_size: u64,

    /// 占总大小的百分比
    pub percentage: f64,
}

/// 用于排序清单的文件条目。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileEntry {
    /// 文件名
    pub name: String,

    /// 文件路径
    pub path: PathBuf,

    /// 文件字节大小
    pub size: u64,
}

impl FileEntry {
    /// 创建一个新的文件条目。
    pub fn new(name: String, path: PathBuf, size: u64) -> Self {
        Self { name, path, size }
    }
}

/// 扫描目录树所收集的统计信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeStats {
    /// 文件总数
    pub total_files: usize,

    /// 目录总数
    pub total_directories: usize,

    /// 符号链接总数
    pub total_symlinks: usize,

    /// 所有文件的总字节大小
    pub total_size: u64,

    /// 按扩展名分组的文件
    pub files_by_extension: HashMap<String, FileTypeInfo>,

    /// 最大的文件（前 N 个）
    pub largest_files: Vec<FileEntry>,

    /// 扫描目录所花费的时间
    pub scan_duration: Duration,
}

impl TreeStats {
    /// 创建一个新的空统计信息对象。
    pub fn new() -> Self {
        Self {
            total_files: 0,
            total_directories: 0,
            total_symlinks: 0,
            total_size: 0,
            files_by_extension: HashMap::new(),
            largest_files: Vec::new(),
            scan_duration: Duration::default(),
        }
    }
}

impl Default for TreeStats {
    fn default() -> Self {
        Self::new()
    }
}

/// 树操作的错误类型。
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    /// 发生 IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 路径不存在
    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    /// 不是目录
    #[error("Not a directory: {0}")]
    NotADirectory(PathBuf),

    /// 权限被拒绝
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// JSON 序列化错误
    #[error("JSON error: {0}")]
    Json(String),

    /// 通用错误消息
    #[error("{0}")]
    Other(String),
}

// 将 serde_json 错误转换为我们自己的 TreeError
impl From<serde_json::Error> for TreeError {
    fn from(err: serde_json::Error) -> Self {
        TreeError::Json(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(name: &str) -> FsNode {
        FsNode::new(name.into(), PathBuf::from(name), FsNodeType::File, 0, 0)
    }

    #[test]
    fn extension_normal() {
        assert_eq!(file("a.txt").extension().as_deref(), Some(".txt"));
    }

    #[test]
    fn extension_multiple_dots() {
        // 多个点：取最后一个点之后的部分
        assert_eq!(file("archive.tar.gz").extension().as_deref(), Some(".gz"));
    }

    #[test]
    fn extension_dotfile_is_none() {
        // 点文件不应被当作有扩展名
        assert_eq!(file(".gitignore").extension(), None);
        assert_eq!(file(".vimrc").extension(), None);
    }

    #[test]
    fn extension_trailing_dot_is_none() {
        assert_eq!(file("file.").extension(), None);
    }

    #[test]
    fn extension_no_dot_is_none() {
        assert_eq!(file("README").extension(), None);
    }

    #[test]
    fn extension_directory_is_none() {
        let dir = FsNode::new_directory("dir".into(), PathBuf::from("dir"), 0, Vec::new());
        assert_eq!(dir.extension(), None);
    }
}
