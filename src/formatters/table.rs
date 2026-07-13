//! 统计信息的表格输出格式化器。

use crate::core::models::TreeStats;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Color, Table,
};
use humansize::format_size;

/// 将统计信息格式化为表格。
///
/// # 参数
///
/// * `stats` - 要格式化的统计信息
///
/// # 返回
///
/// 包含一个或多个表格的格式化字符串。
pub fn format_table(stats: &TreeStats) -> String {
    let mut output = String::new();

    // 概览表
    output.push_str(&format_overview(stats));
    output.push_str("\n\n");

    // 按扩展名分组的文件表
    if !stats.files_by_extension.is_empty() {
        output.push_str(&format_extension_table(stats));
        output.push_str("\n\n");
    }

    // 最大文件表
    if !stats.largest_files.is_empty() {
        output.push_str(&format_largest_files_table(stats));
    }

    output
}

/// 格式化统计概览表。
fn format_overview(stats: &TreeStats) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![Cell::new("Statistics")
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan)]);

    table.add_row(vec![
        Cell::new("Total Files"),
        Cell::new(stats.total_files.to_string()).fg(Color::Green),
    ]);

    table.add_row(vec![
        Cell::new("Total Directories"),
        Cell::new(stats.total_directories.to_string()).fg(Color::Blue),
    ]);

    table.add_row(vec![
        Cell::new("Total Symlinks"),
        Cell::new(stats.total_symlinks.to_string()).fg(Color::Yellow),
    ]);

    table.add_row(vec![
        Cell::new("Total Size"),
        Cell::new(format_size_impl(stats.total_size)).fg(Color::Magenta),
    ]);

    table.add_row(vec![
        Cell::new("Scan Duration"),
        Cell::new(format_duration(stats.scan_duration)).fg(Color::Grey),
    ]);

    table.to_string()
}

/// 格式化按扩展名分组的文件表。
fn format_extension_table(stats: &TreeStats) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Extension")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Count")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Size")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Percentage")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    // 按数量排序（降序）
    let mut extensions: Vec<_> = stats.files_by_extension.iter().collect();
    extensions.sort_by_key(|e| std::cmp::Reverse(e.1.count));

    for (_ext, info) in extensions {
        table.add_row(vec![
            Cell::new(&info.extension),
            Cell::new(info.count.to_string()).fg(Color::Green),
            Cell::new(format_size_impl(info.total_size)).fg(Color::Magenta),
            Cell::new(format!("{:.1}%", info.percentage)).fg(Color::Yellow),
        ]);
    }

    // 添加标题
    let mut output = String::new();
    output.push_str("Files by Extension\n");
    output.push_str(&table.to_string());
    output
}

/// 格式化最大文件表。
fn format_largest_files_table(stats: &TreeStats) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("File")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Size")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    for file in &stats.largest_files {
        table.add_row(vec![
            Cell::new(&file.name),
            Cell::new(format_size_impl(file.size)).fg(Color::Magenta),
        ]);
    }

    // 添加标题
    let mut output = String::new();
    output.push_str(&format!(
        "Largest Files (showing {})\n",
        stats.largest_files.len()
    ));
    output.push_str(&table.to_string());
    output
}

/// 将字节数格式化为人类可读的字符串。
fn format_size_impl(bytes: u64) -> String {
    if bytes == 0 {
        "0 B".to_string()
    } else {
        format_size(bytes, humansize::DECIMAL)
    }
}

/// 将时长格式化为人类可读的字符串。
#[doc(hidden)]
pub fn format_duration(duration: std::time::Duration) -> String {
    let ms = duration.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    }
}

/// 以精简的单行格式格式化统计信息。
///
/// # 参数
///
/// * `stats` - 要格式化的统计信息
///
/// # 返回
///
/// 汇总统计信息的精简单行字符串。
pub fn format_compact(stats: &TreeStats) -> String {
    format!(
        "{} files, {} directories, {} total",
        stats.total_files,
        stats.total_directories,
        format_size_impl(stats.total_size)
    )
}
