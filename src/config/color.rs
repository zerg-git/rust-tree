//! Color configuration for tree output.

use colored::Colorize;
use clap::ValueEnum;
use crate::core::models::FsNode;

/// Color scheme options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ColorScheme {
    /// No colors
    #[default]
    None,
    /// Basic color scheme
    Basic,
    /// Extended color scheme (more file types)
    Extended,
}

/// When to use colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ColorMode {
    /// Always use colors
    Always,
    /// Never use colors
    Never,
    /// Auto-detect based on terminal (default)
    #[default]
    Auto,
}

/// Apply color to a node name based on its type.
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

/// Apply color to a file name based on extension.
fn colorize_file(name: &str, scheme: ColorScheme) -> colored::ColoredString {
    let ext = name.rsplit('.').next().unwrap_or("");

    match scheme {
        ColorScheme::None => name.normal(),
        ColorScheme::Basic => basic_file_color(name, ext),
        ColorScheme::Extended => extended_file_color(name, ext),
    }
}

/// Basic file color scheme.
fn basic_file_color(name: &str, ext: &str) -> colored::ColoredString {
    match ext {
        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "go" | "rb" | "php" => name.green(),
        "toml" | "yaml" | "yml" | "json" | "xml" => name.yellow(),
        "md" | "txt" | "rst" => name.white(),
        "lock" => name.bright_black(),
        _ => name.normal(),
    }
}

/// Extended file color scheme with more file type support.
fn extended_file_color(name: &str, ext: &str) -> colored::ColoredString {
    match ext {
        // Source code files
        "rs" => name.bright_green(),
        "py" => name.green(),
        "js" | "ts" | "tsx" | "jsx" => name.yellow(),
        "java" | "c" | "cpp" | "h" | "hpp" => name.blue(),
        "go" => name.cyan(),
        "rb" | "php" => name.magenta(),

        // Config files
        "toml" | "yaml" | "yml" => name.bright_yellow(),
        "json" | "xml" => name.yellow(),
        "ini" | "cfg" | "conf" => name.bright_black(),

        // Documentation
        "md" | "rst" | "adoc" => name.white(),
        "txt" => name.bright_white(),

        // Build/lock files
        "lock" => name.bright_black(),

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" => name.bright_magenta(),

        // Archives
        "zip" | "tar" | "gz" | "rar" | "7z" => name.red(),

        _ => name.normal(),
    }
}

/// Check if colors should be used based on the mode.
pub fn should_use_colors(mode: ColorMode) -> bool {
    match mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(atty::Stream::Stdout),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{FsNodeType, FsNode};

    #[test]
    fn test_colorize_directory() {
        let node = FsNode::new(
            "test_dir".into(),
            "/test".into(),
            FsNodeType::Directory,
            0,
            0,
        );
        let colored = colorize_node(&node, ColorScheme::Basic);
        // The colored string should contain the name
        assert!(colored.to_string().contains("test_dir"));
    }

    #[test]
    fn test_colorize_file_by_extension() {
        let rust_file = FsNode::new(
            "main.rs".into(),
            "/test/main.rs".into(),
            FsNodeType::File,
            100,
            0,
        );
        let colored = colorize_node(&rust_file, ColorScheme::Basic);
        assert!(colored.to_string().contains("main.rs"));
    }

    #[test]
    fn test_colorize_symlink() {
        let symlink = FsNode::new(
            "link".into(),
            "/test/link".into(),
            FsNodeType::Symlink,
            0,
            0,
        );
        let colored = colorize_node(&symlink, ColorScheme::Basic);
        assert!(colored.to_string().contains("link"));
    }

    #[test]
    fn test_no_color_scheme() {
        let node = FsNode::new(
            "test.rs".into(),
            "/test.rs".into(),
            FsNodeType::File,
            100,
            0,
        );
        let colored = colorize_node(&node, ColorScheme::None);
        assert!(colored.to_string().contains("test.rs"));
    }
}
