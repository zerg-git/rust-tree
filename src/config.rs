//! rust-tree 工具的配置结构。

use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use crate::core::walker::{WalkConfig, SortField};

pub mod color;
pub use color::{ColorMode, ColorScheme};

/// 输出格式选项。
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// 使用 Unicode 字符的树形输出
    Tree,
    /// JSON 格式（同时包含树和统计信息）
    Json,
    /// 显示统计信息的表格格式
    Table,
}

/// 排序字段选项。
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortBy {
    /// 按文件/目录名称排序
    Name,
    /// 按文件大小排序
    Size,
    /// 按文件类型/扩展名排序
    Type,
}

impl From<SortBy> for SortField {
    fn from(sort_by: SortBy) -> Self {
        match sort_by {
            SortBy::Name => SortField::Name,
            SortBy::Size => SortField::Size,
            SortBy::Type => SortField::Type,
        }
    }
}

/// rust-tree 的命令行参数。
#[derive(Parser, Debug)]
#[command(name = "rust-tree")]
#[command(author = "rust-tree contributors")]
#[command(version = "0.1.0")]
#[command(about = "A fast directory tree visualization tool", long_about = None)]
#[command(after_help = "Examples:\n  rust-tree                    # Show current directory\n  rust-tree -d 2 /path/to/dir  # Limit depth to 2\n  rust-tree -f json -S         # JSON output with stats\n  rust-tree -s -o size -r      # Show sizes, sort by size (descending)")]
pub struct Config {
    /// 目标目录路径（默认为当前目录）
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub path: PathBuf,

    /// 最大递归深度（0 表示无限制）
    #[arg(short = 'd', long = "depth", default_value = "0", value_name = "N")]
    pub max_depth: usize,

    /// 输出格式
    #[arg(short = 'f', long = "format", default_value = "tree", value_name = "FORMAT")]
    pub format: OutputFormat,

    /// 显示文件大小
    #[arg(short = 's', long = "size")]
    pub show_size: bool,

    /// 显示隐藏文件（以 . 开头的文件）
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// 按字段排序（name、size、type）
    #[arg(short = 'o', long = "sort", default_value = "name", value_name = "BY")]
    pub sort_by: SortBy,

    /// 反向排序
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// 显示统计摘要（用于 tree 格式），或在 json/table 格式下始终包含统计信息
    #[arg(short = 'S', long = "stats")]
    pub show_stats: bool,

    /// 跟随符号链接
    #[arg(short = 'L', long = "follow")]
    pub follow_symlinks: bool,

    /// 统计信息中显示的最大文件数量
    #[arg(long = "top-files", default_value = "10", value_name = "N")]
    pub top_files: usize,

    /// 颜色模式（always、never、auto）
    #[arg(long = "color", default_value = "auto", value_name = "WHEN")]
    pub color_mode: ColorMode,

    /// 颜色方案（none、basic、extended）
    #[arg(long = "color-scheme", default_value = "basic", value_name = "SCHEME")]
    pub color_scheme: ColorScheme,

    /// 扫描时显示进度条
    #[arg(long = "progress", short = 'p', help = "Show progress bar during scanning")]
    pub show_progress: bool,

    /// 排除匹配模式的文件（可多次使用）
    #[arg(short = 'e', long = "exclude", value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// 仅包含匹配模式的文件
    #[arg(long = "include-only", value_name = "PATTERN")]
    pub include_only: Option<String>,

    /// 使用某种语言常用的排除模式
    #[arg(long = "exclude-common", value_name = "LANGUAGE")]
    pub exclude_common: Option<String>,

    /// 使用流式模式以降低内存占用
    #[arg(long = "streaming", help = "Use streaming mode for low memory usage")]
    pub streaming: bool,
}

impl Config {
    /// 转换为 WalkConfig，供 walker 模块使用。
    pub fn to_walk_config(&self) -> WalkConfig {
        use crate::core::filter::FilterConfig;
        use crate::core::filter::common_excludes;

        let mut filter = FilterConfig::new();
        filter.exclude_hidden = !self.show_hidden;

        // 添加排除模式
        for pattern in &self.exclude {
            let _ = filter.add_exclude(pattern);
        }

        // 添加包含模式
        if let Some(ref pattern) = self.include_only {
            let _ = filter.set_include(pattern);
        }

        // 添加常用排除项
        if let Some(ref lang) = self.exclude_common {
            match lang.as_str() {
                "rust" => {
                    for pattern in common_excludes::rust_patterns() {
                        let _ = filter.add_exclude(pattern);
                    }
                }
                "node" | "nodejs" | "javascript" => {
                    for pattern in common_excludes::nodejs_patterns() {
                        let _ = filter.add_exclude(pattern);
                    }
                }
                "python" => {
                    for pattern in common_excludes::python_patterns() {
                        let _ = filter.add_exclude(pattern);
                    }
                }
                "common" => {
                    for pattern in common_excludes::common_patterns() {
                        let _ = filter.add_exclude(pattern);
                    }
                }
                _ => {}
            }
        }

        WalkConfig {
            max_depth: self.max_depth,
            show_hidden: self.show_hidden,
            follow_symlinks: self.follow_symlinks,
            sort_by: self.sort_by.into(),
            reverse: self.reverse,
            filter,
        }
    }

    /// 检查是否应显示统计信息。
    pub fn should_show_stats(&self) -> bool {
        self.show_stats || matches!(self.format, OutputFormat::Json | OutputFormat::Table)
    }

    /// 获取生效的最大文件显示数量。
    pub fn top_files_count(&self) -> usize {
        self.top_files.max(1)
    }
}
