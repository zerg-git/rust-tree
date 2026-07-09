//! Pattern filtering for directory traversal.

use glob::Pattern;
use std::path::Path;

/// Filter configuration.
#[derive(Debug, Clone, Default)]
pub struct FilterConfig {
    /// Patterns to exclude
    pub exclude_patterns: Vec<Pattern>,
    /// Pattern to include only (if set, only matching paths are included)
    pub include_pattern: Option<Pattern>,
    /// Exclude hidden files
    pub exclude_hidden: bool,
}

impl FilterConfig {
    /// Create a new filter config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an exclude pattern.
    pub fn add_exclude(&mut self, pattern: &str) -> Result<(), String> {
        Pattern::new(pattern)
            .map(|p| self.exclude_patterns.push(p))
            .map_err(|e| e.to_string())
    }

    /// Set include pattern.
    pub fn set_include(&mut self, pattern: &str) -> Result<(), String> {
        Pattern::new(pattern)
            .map(|p| self.include_pattern = Some(p))
            .map_err(|e| e.to_string())
    }

    /// Check if a path should be excluded.
    ///
    /// `is_dir` indicates whether the path is a directory. The `include_pattern`
    /// only filters files: directories always descend (unless hit by an exclude
    /// pattern or the hidden rule), otherwise an `--include-only "*.rs"` would
    /// prune every subdirectory and yield nothing.
    pub fn should_exclude(&self, path: &Path, is_dir: bool) -> bool {
        // Check hidden files
        if self.exclude_hidden {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.starts_with('.') {
                        return true;
                    }
                }
            }
        }

        // Check exclude patterns (apply to both files and directories)
        for pattern in &self.exclude_patterns {
            if pattern.matches_path(path) {
                return true;
            }
            // Also check against file name only
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if pattern.matches(name_str) {
                        return true;
                    }
                }
            }
        }

        // Check include pattern — files only. Directories always descend so
        // matching files deeper in the tree remain reachable.
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

/// Predefined common exclude patterns.
pub mod common_excludes {
    /// Common exclude patterns for Rust projects.
    pub fn rust_patterns() -> Vec<&'static str> {
        vec![".git", "target", "*.rlib", "*.rmeta"]
    }

    /// Common exclude patterns for Node.js projects.
    pub fn nodejs_patterns() -> Vec<&'static str> {
        vec![".git", "node_modules", "package-lock.json", "yarn.lock", "*.log"]
    }

    /// Common exclude patterns for Python projects.
    pub fn python_patterns() -> Vec<&'static str> {
        vec![".git", "__pycache__", "*.pyc", ".venv", "venv", "*.egg-info", ".pytest_cache"]
    }

    /// Common exclude patterns for general development.
    pub fn common_patterns() -> Vec<&'static str> {
        vec![".git", ".svn", ".hg", "*.log", "*.tmp", "*.bak", "*.swp", "*~"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use super::common_excludes::rust_patterns;

    #[test]
    fn test_filter_config_default() {
        let config = FilterConfig::default();
        assert!(!config.exclude_hidden);
        assert!(config.exclude_patterns.is_empty());
        assert!(config.include_pattern.is_none());
    }

    #[test]
    fn test_add_exclude_pattern() {
        let mut config = FilterConfig::new();
        assert!(config.add_exclude("*.log").is_ok());
        assert_eq!(config.exclude_patterns.len(), 1);
    }

    #[test]
    fn test_set_include_pattern() {
        let mut config = FilterConfig::new();
        assert!(config.set_include("*.rs").is_ok());
        assert!(config.include_pattern.is_some());
    }

    #[test]
    fn test_should_exclude_hidden() {
        let config = FilterConfig {
            exclude_hidden: true,
            ..Default::default()
        };
        assert!(config.should_exclude(Path::new(".git"), true));
        assert!(config.should_exclude(Path::new(".hidden"), false));
        assert!(!config.should_exclude(Path::new("visible"), false));
    }

    #[test]
    fn test_should_exclude_pattern() {
        let mut config = FilterConfig::new();
        config.add_exclude("*.log").unwrap();
        assert!(config.should_exclude(Path::new("test.log"), false));
        assert!(!config.should_exclude(Path::new("test.txt"), false));
    }

    #[test]
    fn test_include_only_pattern() {
        let mut config = FilterConfig::new();
        config.set_include("*.rs").unwrap();
        assert!(!config.should_exclude(Path::new("main.rs"), false));
        assert!(config.should_exclude(Path::new("main.py"), false));
    }

    #[test]
    fn test_include_only_does_not_prune_directories() {
        // A directory named "src" must still descend even though it does not
        // match "*.rs", so that main.rs inside it stays reachable.
        let mut config = FilterConfig::new();
        config.set_include("*.rs").unwrap();
        assert!(!config.should_exclude(Path::new("src"), true));
        assert!(!config.should_exclude(Path::new("main.rs"), false));
        assert!(config.should_exclude(Path::new("main.py"), false));
    }

    #[test]
    fn test_exclude_pattern_still_applies_to_directories() {
        let mut config = FilterConfig::new();
        config.add_exclude("target").unwrap();
        assert!(config.should_exclude(Path::new("target"), true));
        assert!(!config.should_exclude(Path::new("src"), true));
    }

    #[test]
    fn test_rust_patterns() {
        let patterns = rust_patterns();
        assert!(patterns.contains(&".git"));
        assert!(patterns.contains(&"target"));
    }
}
