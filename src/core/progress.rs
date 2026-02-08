//! Progress reporting for directory traversal.

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Progress reporter configuration.
#[derive(Debug, Clone)]
pub struct ProgressConfig {
    /// Show progress bar
    pub enabled: bool,
    /// Progress style template
    pub template: String,
    /// Clear progress bar when complete
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

/// Create a new progress bar.
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

/// Update progress message.
pub fn update_progress(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.set_message(msg.to_string());
    }
}

/// Increment progress counter.
pub fn increment_progress(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.inc(1);
    }
}

/// Finish progress with message.
pub fn finish_progress(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.finish_with_message(msg.to_string());
    }
}

/// Abandon progress (remove from screen).
pub fn abandon_progress(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.abandon();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_config_default() {
        let config = ProgressConfig::default();
        assert!(!config.enabled);
        assert!(config.clear_on_finish);
    }

    #[test]
    fn test_create_progress_bar_disabled() {
        let config = ProgressConfig::default();
        let pb = create_progress_bar(&config);
        assert!(pb.is_none());
    }

    #[test]
    fn test_create_progress_bar_enabled() {
        let config = ProgressConfig {
            enabled: true,
            ..Default::default()
        };
        let pb = create_progress_bar(&config);
        assert!(pb.is_some());
    }
}
