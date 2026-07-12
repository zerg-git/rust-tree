//! `config`（CLI 配置类型）及 `config::color` 的测试。

#[path = "config/color.rs"]
mod color;

use rust_tree::{OutputFormat, SortBy, SortField};

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
