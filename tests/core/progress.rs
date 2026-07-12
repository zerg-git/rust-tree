//! `core::progress`（进度条配置）的测试。

use rust_tree::core::progress::{create_progress_bar, ProgressConfig};

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
