//! rust-tree 工具的配置结构。

use crate::core::walker::{SortField, WalkConfig};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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

/// `--exclude-common` 受支持的语言集合。`validate` 用它做输入校验，
/// `to_walk_config` 的 match 负责把语言映射到具体排除模式。
pub const EXCLUDE_COMMON_LANGS: &[&str] =
    &["rust", "node", "nodejs", "javascript", "python", "common"];

/// rust-tree 的命令行参数。
#[derive(Parser, Debug)]
#[command(name = "rust-tree")]
#[command(author = "rust-tree contributors")]
#[command(version = "0.1.0")]
#[command(about = "A fast directory tree visualization tool", long_about = None)]
#[command(
    after_help = "Examples:\n  rust-tree                    # Show current directory\n  rust-tree -d 2 /path/to/dir  # Limit depth to 2\n  rust-tree -f json -S         # JSON output with stats\n  rust-tree -s -o size -r      # Show sizes, sort by size (descending)"
)]
pub struct Config {
    /// 目标目录路径（默认为当前目录）
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub path: PathBuf,

    /// 最大递归深度（0 表示无限制）
    #[arg(short = 'd', long = "depth", default_value = "0", value_name = "N")]
    pub max_depth: usize,

    /// 输出格式
    #[arg(
        short = 'f',
        long = "format",
        default_value = "tree",
        value_name = "FORMAT"
    )]
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
    #[arg(
        long = "progress",
        short = 'p',
        help = "Show progress bar during scanning"
    )]
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
    /// 校验命令行参数的合法性。
    ///
    /// 目前校验 `--exclude-common` 是否为受支持的语言；未知语言在此报错，
    /// 而非像 `to_walk_config` 那样静默跳过。
    pub fn validate(&self) -> Result<(), crate::core::models::TreeError> {
        if let Some(ref lang) = self.exclude_common {
            if !EXCLUDE_COMMON_LANGS.contains(&lang.as_str()) {
                return Err(crate::core::models::TreeError::Other(format!(
                    "unknown --exclude-common language '{}'; supported: {}",
                    lang,
                    EXCLUDE_COMMON_LANGS.join(", ")
                )));
            }
        }
        Ok(())
    }

    /// 转换为 WalkConfig，供 walker 模块使用。
    pub fn to_walk_config(&self) -> WalkConfig {
        use crate::core::filter::common_excludes;
        use crate::core::filter::FilterConfig;

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

        // 是否真正需要文件的字节大小：
        // - 显示 size（-s）时需要；
        // - 统计信息（-S / -f json / -f table）会用到 total_size / largest_files；
        // - 按 size 排序的需求由 walk_children 内部 OR `sort_by == Size` 兜底，
        //   无需在此置位。
        //
        // 关键优化：默认 `rust-tree`（无 -s / -S）既不显示 size 也不打印统计，
        // 此时 collect_stats 的结果会被丢弃——历史上内存路径无条件 need_size
        // 会让每个文件白做一次 stat（大目录下 ~19s 全是内核态 syscall）。
        // 改为按需后，默认调用与 streaming 默认路径持平。
        //
        // streaming 分支 should_show_stats() 恒为 false（该组合在 run() 中已被
        // 拒绝），故本公式对两种路径统一成立。
        let need_size = self.show_size || self.should_show_stats();

        WalkConfig {
            max_depth: self.max_depth,
            show_hidden: self.show_hidden,
            follow_symlinks: self.follow_symlinks,
            sort_by: self.sort_by.into(),
            reverse: self.reverse,
            filter,
            need_size,
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
