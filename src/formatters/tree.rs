//! Tree-style output formatter.

use crate::core::models::FsNode;
use humansize::format_size;

/// Format a file tree as a tree structure using Unicode box-drawing characters.
///
/// # Arguments
///
/// * `node` - The root node of the tree
/// * `show_size` - Whether to display file sizes
///
/// # Returns
///
/// A formatted string representing the tree structure.
pub fn format_tree(node: &FsNode, show_size: bool) -> String {
    let mut output = String::new();

    // Print root directory
    let size_str = if show_size && node.is_directory() {
        format!(" ({} files)", count_files_recursive(node))
    } else if show_size && node.size > 0 {
        format!(" ({})", format_size_impl(node.size))
    } else {
        String::new()
    };

    output.push_str(&format!("{}{}/\n", node.name, size_str));

    // Print children with tree prefixes
    if let Some(children) = &node.children {
        let last_index = children.len().saturating_sub(1);
        for (i, child) in children.iter().enumerate() {
            format_node_recursive(
                child,
                "",
                i == last_index,
                show_size,
                &mut output,
            );
        }
    }

    output
}

/// Recursively format a node with appropriate tree prefixes.
fn format_node_recursive(
    node: &FsNode,
    prefix: &str,
    is_last: bool,
    show_size: bool,
    output: &mut String,
) {
    // Determine the connector and next prefix
    let (connector, next_prefix_base) = if prefix.is_empty() {
        if is_last { ("└── ", "    ") } else { ("├── ", "│   ") }
    } else {
        if is_last { ("└── ", "    ") } else { ("├── ", "│   ") }
    };

    let next_prefix = format!("{}{}", prefix, next_prefix_base);

    // Build the node label
    let mut label = node.name.clone();

    // Add directory indicator
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

    // Add size information if requested
    if show_size && node.is_file() && node.size > 0 {
        label.push_str(&format!(" ({})", format_size_impl(node.size)));
    } else if show_size && node.is_directory() {
        let file_count = count_files_recursive(node);
        if file_count > 0 {
            label.push_str(&format!(" ({} files)", file_count));
        }
    }

    output.push_str(&format!("{}{}{}\n", prefix, connector, label));

    // Print children
    if let Some(children) = &node.children {
        let last_index = children.len().saturating_sub(1);
        for (i, child) in children.iter().enumerate() {
            format_node_recursive(
                child,
                &next_prefix,
                i == last_index,
                show_size,
                output,
            );
        }
    }
}

/// Format a size in bytes to a human-readable string.
fn format_size_impl(bytes: u64) -> String {
    format_size(bytes, humansize::DECIMAL)
}

/// Count all files in a subtree (recursively).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{FsNodeType, FsNode};

    #[test]
    fn test_format_tree_simple() {
        let mut file1 = FsNode::new(
            "file.txt".into(),
            "/test/file.txt".into(),
            FsNodeType::File,
            1024,
            1,
        );
        let mut dir1 = FsNode::new(
            "subdir".into(),
            "/test/subdir".into(),
            FsNodeType::Directory,
            0,
            1,
        );
        dir1.children = Some(vec![]);

        let mut root = FsNode::new(
            "root".into(),
            "/test".into(),
            FsNodeType::Directory,
            0,
            0,
        );
        root.children = Some(vec![dir1, file1]);

        let output = format_tree(&root, false);

        assert!(output.contains("root/"));
        assert!(output.contains("subdir/"));
        assert!(output.contains("file.txt"));
    }

    #[test]
    fn test_format_size() {
        // humansize uses "KiB" instead of "KB"
        let s1 = format_size_impl(1024);
        assert!(s1.contains("K") || s1.contains("k"));
        let s2 = format_size_impl(1048576);
        assert!(s2.contains("M") || s2.contains("m"));
    }
}
