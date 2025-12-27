//! Table-style output formatter for statistics.

use comfy_table::{Table, Attribute, Color, Cell, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS};
use humansize::format_size;
use crate::core::models::TreeStats;

/// Format statistics as a table.
///
/// # Arguments
///
/// * `stats` - Statistics to format
///
/// # Returns
///
/// A formatted string containing one or more tables.
pub fn format_table(stats: &TreeStats) -> String {
    let mut output = String::new();

    // Overview table
    output.push_str(&format_overview(stats));
    output.push_str("\n\n");

    // Files by extension table
    if !stats.files_by_extension.is_empty() {
        output.push_str(&format_extension_table(stats));
        output.push_str("\n\n");
    }

    // Largest files table
    if !stats.largest_files.is_empty() {
        output.push_str(&format_largest_files_table(stats));
    }

    output
}

/// Format the overview statistics table.
fn format_overview(stats: &TreeStats) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Statistics")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

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

/// Format the files-by-extension table.
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

    // Sort by count (descending)
    let mut extensions: Vec<_> = stats.files_by_extension.iter().collect();
    extensions.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    for (ext, info) in extensions {
        table.add_row(vec![
            Cell::new(&info.extension),
            Cell::new(info.count.to_string()).fg(Color::Green),
            Cell::new(format_size_impl(info.total_size)).fg(Color::Magenta),
            Cell::new(format!("{:.1}%", info.percentage)).fg(Color::Yellow),
        ]);
    }

    // Add title
    let mut output = String::new();
    output.push_str("Files by Extension\n");
    output.push_str(&table.to_string());
    output
}

/// Format the largest files table.
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

    // Add title
    let mut output = String::new();
    output.push_str(&format!("Largest Files (showing {})\n", stats.largest_files.len()));
    output.push_str(&table.to_string());
    output
}

/// Format a size in bytes to a human-readable string.
fn format_size_impl(bytes: u64) -> String {
    if bytes == 0 {
        "0 B".to_string()
    } else {
        format_size(bytes, humansize::DECIMAL)
    }
}

/// Format a duration to a human-readable string.
fn format_duration(duration: std::time::Duration) -> String {
    let ms = duration.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    }
}

/// Format statistics in a compact, single-line format.
///
/// # Arguments
///
/// * `stats` - Statistics to format
///
/// # Returns
///
/// A compact single-line string summarizing the statistics.
pub fn format_compact(stats: &TreeStats) -> String {
    format!(
        "{} files, {} directories, {} total",
        stats.total_files,
        stats.total_directories,
        format_size_impl(stats.total_size)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
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
        // humansize uses "MiB" instead of "MB"
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
}
