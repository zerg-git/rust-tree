//! Streaming tree formatter for memory-efficient output.

use std::io::Write;
use crate::core::streaming::{walk_streaming, StreamNode, StreamConfig};
use crate::config::{ColorMode, ColorScheme};
use crate::config::color::should_use_colors;
use humansize::format_size;

/// Format a tree using streaming (constant memory).
pub fn format_tree_streaming<W: Write>(
    root: &std::path::Path,
    writer: &mut W,
    show_size: bool,
    color_mode: ColorMode,
    color_scheme: ColorScheme,
    config: StreamConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let use_color = should_use_colors(color_mode);

    // Emit root directory first
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

    // Track prefix state for tree drawing
    let mut prefix_stack: Vec<bool> = Vec::new();

    // Walk tree and output nodes as we visit them
    walk_streaming(root, config, |node| {
        // Update prefix stack for current depth
        while prefix_stack.len() <= node.depth {
            prefix_stack.push(false);
        }

        // Set the "is last" flag for parent depth
        if node.depth > 0 {
            prefix_stack[node.depth - 1] = node.is_last;
        }

        // Build prefix based on depth
        let prefix = build_prefix(&prefix_stack, node.depth);

        // Build label
        let label = build_label(node, show_size, use_color, color_scheme);

        // Write line
        let _ = writeln!(writer, "{}{}", prefix, label);
    })?;

    Ok(())
}

/// Build the tree prefix for current depth.
fn build_prefix(prefix_stack: &[bool], current_depth: usize) -> String {
    if current_depth == 0 {
        return String::new();
    }

    let mut prefix = String::new();

    for (i, is_last) in prefix_stack.iter().enumerate() {
        if i >= current_depth {
            break;
        }

        if i == current_depth - 1 {
            prefix.push_str(if *is_last { "└── " } else { "├── " });
        } else {
            prefix.push_str(if *is_last { "    " } else { "│   " });
        }
    }

    prefix
}

/// Build the node label.
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

    // Add directory indicator
    if node.node_type == crate::core::models::FsNodeType::Directory {
        label.push('/');
    } else if node.node_type == crate::core::models::FsNodeType::Symlink {
        label.push_str(" -> ");
        if let Ok(target) = std::fs::read_link(&node.path) {
            label.push_str(&target.to_string_lossy());
        }
    }

    // Add size
    if show_size && node.node_type == crate::core::models::FsNodeType::File && node.size > 0 {
        label.push_str(&format!(" ({})", format_size(node.size, humansize::DECIMAL)));
    }

    label
}

/// Colorize a name based on node type and extension.
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

/// Basic file color scheme.
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

/// Extended file color scheme.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prefix() {
        let prefix_stack = vec![false, false];
        let prefix = build_prefix(&prefix_stack, 1);
        assert_eq!(prefix, "├── ");
    }

    #[test]
    fn test_build_prefix_last() {
        let prefix_stack = vec![true, false];
        let prefix = build_prefix(&prefix_stack, 1);
        assert_eq!(prefix, "└── ");
    }

    #[test]
    fn test_build_prefix_nested() {
        let prefix_stack = vec![false, true, false];
        let prefix = build_prefix(&prefix_stack, 2);
        assert_eq!(prefix, "│   └── ");
    }
}
