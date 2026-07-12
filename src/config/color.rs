//! 树形输出的颜色配置。

use colored::Colorize;
use clap::ValueEnum;
use crate::core::models::FsNode;

/// 颜色方案选项。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ColorScheme {
    /// 无颜色
    #[default]
    None,
    /// 基础颜色方案
    Basic,
    /// 扩展颜色方案（支持更多文件类型）
    Extended,
}

/// 何时使用颜色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ColorMode {
    /// 始终使用颜色
    Always,
    /// 从不使用颜色
    Never,
    /// 根据终端自动检测（默认）
    #[default]
    Auto,
}

/// 根据节点类型为节点名称着色。
pub fn colorize_node(node: &FsNode, scheme: ColorScheme) -> colored::ColoredString {
    match node.node_type {
        crate::core::models::FsNodeType::Directory => {
            node.name.clone().blue().bold()
        }
        crate::core::models::FsNodeType::File => {
            colorize_file(&node.name, scheme)
        }
        crate::core::models::FsNodeType::Symlink => {
            node.name.clone().cyan().italic()
        }
    }
}

/// 根据扩展名为文件名着色。
fn colorize_file(name: &str, scheme: ColorScheme) -> colored::ColoredString {
    let ext = name.rsplit('.').next().unwrap_or("");

    match scheme {
        ColorScheme::None => name.normal(),
        ColorScheme::Basic => basic_file_color(name, ext),
        ColorScheme::Extended => extended_file_color(name, ext),
    }
}

/// 基础的文件颜色方案。
fn basic_file_color(name: &str, ext: &str) -> colored::ColoredString {
    match ext {
        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "go" | "rb" | "php" => name.green(),
        "toml" | "yaml" | "yml" | "json" | "xml" => name.yellow(),
        "md" | "txt" | "rst" => name.white(),
        "lock" => name.bright_black(),
        _ => name.normal(),
    }
}

/// 扩展的文件颜色方案，支持更多文件类型。
fn extended_file_color(name: &str, ext: &str) -> colored::ColoredString {
    match ext {
        // 源代码文件
        "rs" => name.bright_green(),
        "py" => name.green(),
        "js" | "ts" | "tsx" | "jsx" => name.yellow(),
        "java" | "c" | "cpp" | "h" | "hpp" => name.blue(),
        "go" => name.cyan(),
        "rb" | "php" => name.magenta(),

        // 配置文件
        "toml" | "yaml" | "yml" => name.bright_yellow(),
        "json" | "xml" => name.yellow(),
        "ini" | "cfg" | "conf" => name.bright_black(),

        // 文档
        "md" | "rst" | "adoc" => name.white(),
        "txt" => name.bright_white(),

        // 构建/锁文件
        "lock" => name.bright_black(),

        // 图片
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" => name.bright_magenta(),

        // 归档文件
        "zip" | "tar" | "gz" | "rar" | "7z" => name.red(),

        _ => name.normal(),
    }
}

/// 根据模式判断是否应使用颜色。
pub fn should_use_colors(mode: ColorMode) -> bool {
    match mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
    }
}
