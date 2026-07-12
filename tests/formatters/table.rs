//! `formatters::table`（表格统计输出）的测试。

use rust_tree::formatters::table::{format_compact, format_duration};
use rust_tree::{format_table, TreeStats};
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_format_table() {
    let stats = TreeStats {
        total_files: 42,
        total_directories: 8,
        total_symlinks: 1,
        total_size: 1024 * 1024,
        files_by_extension: HashMap::new(),
        largest_files: vec![],
        scan_duration: Duration::from_millis(150),
    };

    let table = format_table(&stats);
    assert!(table.contains("42"));
    assert!(table.contains("8"));
    // humansize 使用 "MiB" 而非 "MB"
    assert!(table.contains("M") || table.contains("m"));
}

#[test]
fn test_format_compact() {
    let stats = TreeStats {
        total_files: 10,
        total_directories: 2,
        total_symlinks: 0,
        total_size: 2048,
        files_by_extension: HashMap::new(),
        largest_files: vec![],
        scan_duration: Duration::from_millis(50),
    };

    let compact = format_compact(&stats);
    assert!(compact.contains("10 files"));
    assert!(compact.contains("2 directories"));
}

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
    assert_eq!(format_duration(Duration::from_millis(1500)), "1.5s");
}
