//! 目录遍历的模式过滤。

use glob::Pattern;
use std::path::Path;

/// 过滤器配置。
#[derive(Debug, Clone, Default)]
pub struct FilterConfig {
    /// 要排除的模式
    pub exclude_patterns: Vec<Pattern>,
    /// 仅包含的模式（若设置，则只包含匹配的路径）
    pub include_pattern: Option<Pattern>,
    /// 排除隐藏文件
    pub exclude_hidden: bool,
}

impl FilterConfig {
    /// 创建一个新的过滤器配置。
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加一个排除模式。
    pub fn add_exclude(&mut self, pattern: &str) -> Result<(), String> {
        Pattern::new(pattern)
            .map(|p| self.exclude_patterns.push(p))
            .map_err(|e| e.to_string())
    }

    /// 设置包含模式。
    pub fn set_include(&mut self, pattern: &str) -> Result<(), String> {
        Pattern::new(pattern)
            .map(|p| self.include_pattern = Some(p))
            .map_err(|e| e.to_string())
    }

    /// 检查某个路径是否应被排除。
    ///
    /// `is_dir` 指示该路径是否为目录。`include_pattern` 只过滤文件：
    /// 目录总是会下降（除非命中排除模式或隐藏规则），否则一个
    /// `--include-only "*.rs"` 会剪除每个子目录，从而什么都得不到。
    pub fn should_exclude(&self, path: &Path, is_dir: bool) -> bool {
        // 检查隐藏文件
        if self.exclude_hidden {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.starts_with('.') {
                        return true;
                    }
                }
            }
        }

        // 检查排除模式（同时作用于文件和目录）
        for pattern in &self.exclude_patterns {
            if pattern.matches_path(path) {
                return true;
            }
            // 同时仅对文件名进行检查
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if pattern.matches(name_str) {
                        return true;
                    }
                }
            }
        }

        // 检查包含模式——仅对文件。目录总是会下降，这样树更深处
        // 匹配的文件仍然可达。
        if !is_dir {
            if let Some(ref pattern) = self.include_pattern {
                let matches_path = pattern.matches_path(path);
                let matches_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| pattern.matches(n))
                    .unwrap_or(false);
                if !matches_path && !matches_name {
                    return true;
                }
            }
        }

        false
    }
}

/// 预定义的常用排除模式。
pub mod common_excludes {
    /// Rust 项目的常用排除模式。
    pub fn rust_patterns() -> Vec<&'static str> {
        vec![".git", "target", "*.rlib", "*.rmeta"]
    }

    /// Node.js 项目的常用排除模式。
    pub fn nodejs_patterns() -> Vec<&'static str> {
        vec![
            ".git",
            "node_modules",
            "package-lock.json",
            "yarn.lock",
            "*.log",
        ]
    }

    /// Python 项目的常用排除模式。
    pub fn python_patterns() -> Vec<&'static str> {
        vec![
            ".git",
            "__pycache__",
            "*.pyc",
            ".venv",
            "venv",
            "*.egg-info",
            ".pytest_cache",
        ]
    }

    /// 通用开发的常用排除模式。
    pub fn common_patterns() -> Vec<&'static str> {
        vec![
            ".git", ".svn", ".hg", "*.log", "*.tmp", "*.bak", "*.swp", "*~",
        ]
    }
}
