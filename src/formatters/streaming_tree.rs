//! 用于内存高效输出的流式树格式化器。

use std::io::Write;
use crate::core::streaming::{walk_core, StreamNode};
use crate::core::walker::WalkConfig;
use crate::config::{ColorMode, ColorScheme};
use crate::config::color::should_use_colors;
use humansize::format_size;

/// 使用流式核心格式化树（峰值内存为 O(最宽目录的宽度)）。
pub fn format_tree_streaming<W: Write>(
    root: &std::path::Path,
    writer: &mut W,
    show_size: bool,
    color_mode: ColorMode,
    color_scheme: ColorScheme,
    config: WalkConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let use_color = should_use_colors(color_mode);

    // 先输出根目录
    let root_name = root.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".")
        .to_string();

    let root_colored = if use_color {
        colorize_by_type_and_ext(&root_name, &crate::core::models::FsNodeType::Directory, color_scheme)
            .to_string()
    } else {
        root_name.clone()
    };

    writeln!(writer, "{}/", root_colored)?;

    // prefix_stack[d] 保存当前路径上深度为 d 的节点的 is_last 标志
    // 子节点从深度 1 开始。
    let mut prefix_stack: Vec<bool> = Vec::new();

    walk_core(root, &config, |node| {
        while prefix_stack.len() <= node.depth {
            prefix_stack.push(false);
        }
        prefix_stack[node.depth] = node.is_last;

        let prefix = build_prefix(&prefix_stack, node.depth);
        let label = build_label(node, show_size, use_color, color_scheme);
        let _ = writeln!(writer, "{}{}", prefix, label);
    })?;

    Ok(())
}

/// 为深度为 `depth` (>= 1) 的节点构建树形前缀。
///
/// 祖先层 1..depth 根据该祖先是否为其父节点的最后一个子节点，
/// 绘制空白间隔或竖线；节点自身所在层绘制分支连接符。
#[doc(hidden)]
pub fn build_prefix(prefix_stack: &[bool], depth: usize) -> String {
    let mut prefix = String::new();

    for level in 1..depth {
        let ancestor_is_last = prefix_stack.get(level).copied().unwrap_or(false);
        prefix.push_str(if ancestor_is_last { "    " } else { "│   " });
    }

    let is_last = prefix_stack.get(depth).copied().unwrap_or(false);
    prefix.push_str(if is_last { "└── " } else { "├── " });

    prefix
}

/// 构建节点标签。
fn build_label(
    node: &StreamNode,
    show_size: bool,
    use_color: bool,
    color_scheme: ColorScheme,
) -> String {
    let name = if use_color {
        colorize_by_type_and_ext(&node.name, &node.node_type, color_scheme).to_string()
    } else {
        node.name.clone()
    };

    let mut label = name;

    // 添加目录指示符
    if node.node_type == crate::core::models::FsNodeType::Directory {
        label.push('/');
    } else if node.node_type == crate::core::models::FsNodeType::Symlink {
        label.push_str(" -> ");
        if let Ok(target) = std::fs::read_link(&node.path) {
            label.push_str(&target.to_string_lossy());
        }
    }

    // 添加大小
    if show_size && node.node_type == crate::core::models::FsNodeType::File && node.size > 0 {
        label.push_str(&format!(" ({})", format_size(node.size, humansize::DECIMAL)));
    }

    label
}

/// 根据节点类型和扩展名对名称着色。
fn colorize_by_type_and_ext(
    name: &str,
    node_type: &crate::core::models::FsNodeType,
    scheme: ColorScheme,
) -> colored::ColoredString {
    use colored::Colorize;
    use crate::core::models::FsNodeType;

    match node_type {
        FsNodeType::Directory => name.blue().bold(),
        FsNodeType::Symlink => name.cyan().italic(),
        FsNodeType::File => {
            let ext = name.rsplit('.').next().unwrap_or("");
            match scheme {
                ColorScheme::None => name.normal(),
                ColorScheme::Basic => basic_file_color(name, ext),
                ColorScheme::Extended => extended_file_color(name, ext),
            }
        }
    }
}

/// 基础文件配色方案。
fn basic_file_color(name: &str, ext: &str) -> colored::ColoredString {
    use colored::Colorize;
    match ext {
        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "go" | "rb" | "php" => name.green(),
        "toml" | "yaml" | "yml" | "json" | "xml" => name.yellow(),
        "md" | "txt" | "rst" => name.white(),
        "lock" => name.bright_black(),
        _ => name.normal(),
    }
}

/// 扩展文件配色方案。
fn extended_file_color(name: &str, ext: &str) -> colored::ColoredString {
    use colored::Colorize;
    match ext {
        "rs" => name.bright_green(),
        "py" => name.green(),
        "js" | "ts" | "tsx" | "jsx" => name.yellow(),
        "java" | "c" | "cpp" | "h" | "hpp" => name.blue(),
        "go" => name.cyan(),
        "rb" | "php" => name.magenta(),
        "toml" | "yaml" | "yml" => name.bright_yellow(),
        "json" | "xml" => name.yellow(),
        "ini" | "cfg" | "conf" => name.bright_black(),
        "md" | "rst" | "adoc" => name.white(),
        "txt" => name.bright_white(),
        "lock" => name.bright_black(),
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" => name.bright_magenta(),
        "zip" | "tar" | "gz" | "rar" | "7z" => name.red(),
        _ => name.normal(),
    }
}
