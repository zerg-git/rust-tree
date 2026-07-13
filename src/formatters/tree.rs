//! 树形输出格式化器。

use crate::config::color::{colorize_node, should_use_colors};
use crate::config::{ColorMode, ColorScheme};
use crate::core::models::FsNode;
use humansize::format_size;

/// 使用 Unicode 制表符将文件树格式化为树形结构。
///
/// # 参数
///
/// * `node` - 树的根节点
/// * `show_size` - 是否显示文件大小
/// * `color_mode` - 何时使用颜色
/// * `color_scheme` - 使用的配色方案
///
/// # 返回
///
/// 表示树形结构的格式化字符串。
pub fn format_tree(
    node: &FsNode,
    show_size: bool,
    color_mode: ColorMode,
    color_scheme: ColorScheme,
) -> String {
    let mut output = String::new();

    // 打印根目录并着色
    let root_name = if should_use_colors(color_mode) {
        colorize_node(node, color_scheme).to_string()
    } else {
        node.name.clone()
    };

    let size_str = if show_size && node.is_directory() {
        format!(" ({} files)", count_files_recursive(node))
    } else if show_size && node.size > 0 {
        format!(" ({})", format_size_impl(node.size))
    } else {
        String::new()
    };

    output.push_str(&format!("{}{}/\n", root_name, size_str));

    // 打印子节点并附带树形前缀
    if let Some(children) = &node.children {
        let last_index = children.len().saturating_sub(1);
        for (i, child) in children.iter().enumerate() {
            format_node_recursive(
                child,
                "",
                i == last_index,
                show_size,
                color_mode,
                color_scheme,
                &mut output,
            );
        }
    }

    output
}

/// 递归地格式化节点并附带相应的树形前缀。
fn format_node_recursive(
    node: &FsNode,
    prefix: &str,
    is_last: bool,
    show_size: bool,
    color_mode: ColorMode,
    color_scheme: ColorScheme,
    output: &mut String,
) {
    // 确定连接符和下一个前缀
    let (connector, next_prefix_base) = if is_last {
        ("└── ", "    ")
    } else {
        ("├── ", "│   ")
    };

    let next_prefix = format!("{}{}", prefix, next_prefix_base);

    // 构建节点标签并着色
    let use_color = should_use_colors(color_mode);
    let name = if use_color {
        colorize_node(node, color_scheme).to_string()
    } else {
        node.name.clone()
    };

    let mut label = name;

    // 添加目录指示符
    if node.is_directory() {
        label.push('/');
    } else if node.is_symlink() {
        label.push_str(" -> ");
        if let Some(path) = &node.path {
            if let Ok(target) = std::fs::read_link(path) {
                label.push_str(&target.to_string_lossy());
            }
        }
    }

    // 如有需要，添加大小信息
    if show_size && node.is_file() && node.size > 0 {
        label.push_str(&format!(" ({})", format_size_impl(node.size)));
    } else if show_size && node.is_directory() {
        let file_count = count_files_recursive(node);
        if file_count > 0 {
            label.push_str(&format!(" ({} files)", file_count));
        }
    }

    output.push_str(&format!("{}{}{}\n", prefix, connector, label));

    // 打印子节点
    if let Some(children) = &node.children {
        let last_index = children.len().saturating_sub(1);
        for (i, child) in children.iter().enumerate() {
            format_node_recursive(
                child,
                &next_prefix,
                i == last_index,
                show_size,
                color_mode,
                color_scheme,
                output,
            );
        }
    }
}

/// 将字节数格式化为人类可读的字符串。
#[doc(hidden)]
pub fn format_size_impl(bytes: u64) -> String {
    format_size(bytes, humansize::DECIMAL)
}

/// 统计子树中的所有文件（递归）。
fn count_files_recursive(node: &FsNode) -> usize {
    let mut count = 0;

    if let Some(children) = &node.children {
        for child in children {
            if child.is_file() {
                count += 1;
            } else if child.is_directory() {
                count += count_files_recursive(child);
            }
        }
    }

    count
}
