//! `core::filter`（包含/排除模式过滤）的测试。

use rust_tree::core::filter::common_excludes::rust_patterns;
use rust_tree::core::filter::FilterConfig;
use std::path::Path;

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
    // 名为 "src" 的目录即便不匹配 "*.rs" 也必须继续下钻，
    // 这样其中的 main.rs 才能保持可达。
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
