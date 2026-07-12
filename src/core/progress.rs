//! 目录遍历的进度报告。

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// 进度报告器配置。
#[derive(Debug, Clone)]
pub struct ProgressConfig {
    /// 显示进度条
    pub enabled: bool,
    /// 进度条样式模板
    pub template: String,
    /// 完成时清除进度条
    pub clear_on_finish: bool,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            template: "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}".to_string(),
            clear_on_finish: true,
        }
    }
}

/// 创建一个新的进度条。
pub fn create_progress_bar(config: &ProgressConfig) -> Option<ProgressBar> {
    if !config.enabled {
        return None;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    Some(pb)
}

/// 更新进度消息。
pub fn update_progress(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.set_message(msg.to_string());
    }
}

/// 递增进度计数。
pub fn increment_progress(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.inc(1);
    }
}

/// 以消息完成进度。
pub fn finish_progress(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.finish_with_message(msg.to_string());
    }
}

/// 放弃进度（从屏幕移除）。
pub fn abandon_progress(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.abandon();
    }
}
