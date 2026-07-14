//! rust-tree - 一个快速的目录树可视化工具。
//!
//! 本库提供了遍历目录结构、收集统计信息，并以多种风格格式化输出的功能。
//!
//! # 示例
//!
//! ```no_run
//! use rust_tree::{Config, run};
//!
//! let config = Config {
//!     path: "/path/to/directory".into(),
//!     max_depth: 3,
//!     ..Default::default()
//! };
//!
//! run(config).expect("Failed to run");
//! ```

pub mod config;
pub mod core;
pub mod formatters;

// 重新导出常用类型
pub use config::{ColorMode, ColorScheme, Config, OutputFormat, SortBy};
pub use core::{
    collector::{collect_stats, get_all_directories, get_all_files},
    models::{FileEntry, FileTypeInfo, FsNode, FsNodeType, FsTree, TreeError, TreeStats},
    walker::{walk_directory, SortField, WalkConfig},
};
pub use formatters::{format_json, format_table, format_tree};

use crate::core::progress::{
    create_progress_bar, finish_progress, update_progress, ProgressConfig,
};
use std::io::{self, Write};
use std::time::Instant;

/// 使用给定配置运行 rust-tree 工具。
///
/// 这是本库的主入口。它执行以下步骤：
/// 1. 遍历目录树
/// 2. 收集统计信息
/// 3. 格式化并输出结果
///
/// # 错误
///
/// 如果目录遍历失败或输出格式化失败，则返回 `TreeError`。
pub fn run(config: Config) -> Result<(), TreeError> {
    let start_time = Instant::now();

    // 流式模式在访问节点时即输出，并不会将整棵树具体化，
    // 因此统计信息（需要完整树）无法计算。这里显式拒绝
    // 该组合，而不是静默丢弃统计信息。
    if config.streaming && config.should_show_stats() {
        return Err(TreeError::Other(
            "streaming mode does not support statistics; drop --stats or --streaming \
             (and note -f json / -f table imply stats)"
                .to_string(),
        ));
    }

    // 校验参数（如 --exclude-common 的未知语言）。
    config.validate()?;

    // 检查是否启用了流式模式
    if config.streaming {
        return run_streaming(config);
    }

    // 传统模式
    // 如有需要则创建进度条
    let progress_config = ProgressConfig {
        enabled: config.show_progress,
        ..Default::default()
    };
    let progress = create_progress_bar(&progress_config);

    // 遍历目录
    update_progress(&progress, &format!("Scanning: {}", config.path.display()));
    let tree = walk_directory(&config.path, &config.to_walk_config(), progress.as_ref())?;
    finish_progress(&progress, "Scan complete");

    // 收集统计信息：仅当统计会被使用时（-S、-f json、-f table）才收集。
    // 默认 tree 视图无 -s/-S 时统计结果会被丢弃，跳过可省去一次全树遍历；
    // 且此时 need_size=false 已使文件 size 为 0，即便收集也是零值。
    // scan_duration 仅在统计块中展示，跳过时也无需计算。
    let stats = if config.should_show_stats() {
        collect_stats(&tree, start_time, config.top_files_count())
    } else {
        crate::core::models::TreeStats::new()
    };

    // 根据所选格式格式化输出
    let output = match config.format {
        OutputFormat::Tree => {
            let mut result = format_tree(
                &tree.root,
                config.show_size,
                config.color_mode,
                config.color_scheme,
            );

            // 如有需要则追加统计信息
            if config.show_stats {
                result.push_str("\n\n");
                result.push_str(&crate::formatters::table::format_compact(&stats));
                result.push('\n');
            }

            result
        }
        OutputFormat::Json => format_json(&tree, &stats, true)?,
        OutputFormat::Table => format_table(&stats),
    };

    // 打印输出
    print!("{}", output);
    io::stdout()
        .flush()
        .map_err(|e| TreeError::Other(e.to_string()))?;

    Ok(())
}

/// 以流式模式运行（峰值内存为 O(最宽目录的宽度)）。
fn run_streaming(config: Config) -> Result<(), TreeError> {
    use crate::formatters::streaming_tree::format_tree_streaming;

    let walk_config = config.to_walk_config();

    // 流式模式也支持 --progress：真实进度条在遍历回调里推进。
    let progress_config = ProgressConfig {
        enabled: config.show_progress,
        ..Default::default()
    };
    let progress = create_progress_bar(&progress_config);
    update_progress(&progress, &format!("Scanning: {}", config.path.display()));

    // 流式模式直接使用 stdout
    let mut stdout = io::stdout().lock();

    format_tree_streaming(
        &config.path,
        &mut stdout,
        config.show_size,
        config.color_mode,
        config.color_scheme,
        walk_config,
        progress.as_ref(),
    )
    .map_err(|e| TreeError::Other(e.to_string()))?;

    finish_progress(&progress, "Scan complete");
    io::stdout()
        .flush()
        .map_err(|e| TreeError::Other(e.to_string()))?;

    Ok(())
}

/// 用于创建基础 Config 的默认实现。
impl Default for Config {
    fn default() -> Self {
        Config {
            path: ".".into(),
            max_depth: 0,
            format: OutputFormat::Tree,
            show_size: false,
            show_hidden: false,
            sort_by: SortBy::Name,
            reverse: false,
            show_stats: false,
            follow_symlinks: false,
            top_files: 10,
            color_mode: config::ColorMode::Auto,
            color_scheme: config::ColorScheme::Basic,
            show_progress: false,
            exclude: Vec::new(),
            include_only: None,
            exclude_common: None,
            streaming: false,
        }
    }
}
