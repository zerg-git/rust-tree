//! `config`（CLI 配置类型）及 `config::color` 的测试。

#[path = "config/color.rs"]
mod color;

use rust_tree::{Config, OutputFormat, SortBy, SortField};

#[test]
fn test_sort_by_conversion() {
    assert_eq!(SortField::from(SortBy::Name), SortField::Name);
    assert_eq!(SortField::from(SortBy::Size), SortField::Size);
    assert_eq!(SortField::from(SortBy::Type), SortField::Type);
}

#[test]
fn test_output_format_values() {
    let formats = [OutputFormat::Tree, OutputFormat::Json, OutputFormat::Table];
    assert_eq!(formats.len(), 3);
}

#[test]
fn test_validate_exclude_common_known_language() {
    for lang in ["rust", "node", "nodejs", "javascript", "python", "common"] {
        let cfg = Config {
            exclude_common: Some(lang.into()),
            ..Default::default()
        };
        assert!(
            cfg.validate().is_ok(),
            "language '{}' should be valid",
            lang
        );
    }
}

#[test]
fn test_validate_exclude_common_unknown_language() {
    for lang in ["java", "go", "c", ""] {
        let cfg = Config {
            exclude_common: Some(lang.into()),
            ..Default::default()
        };
        let err = cfg.validate().unwrap_err().to_string();
        assert!(
            err.contains("unknown --exclude-common language"),
            "expected unknown-language error for '{}', got: {}",
            lang,
            err
        );
    }
}

#[test]
fn test_validate_exclude_common_none_is_ok() {
    let cfg = Config {
        exclude_common: None,
        ..Default::default()
    };
    assert!(cfg.validate().is_ok());
}
