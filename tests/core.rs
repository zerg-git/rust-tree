//! `core`（遍历、统计、过滤、进度、流式）的测试。
//!
//! `tests/core.rs` 是 `core` 集成测试目标的 crate root，因此每个子模块都用
//! `#[path]` 锚定到 `tests/core/` 下对应的镜像位置（crate root 的 `mod` 声明
//! 相对 `tests/` 解析，而非进入 `core/` 子目录）。

#[path = "core/collector.rs"]
mod collector;
#[path = "core/filter.rs"]
mod filter;
#[path = "core/progress.rs"]
mod progress;
#[path = "core/streaming.rs"]
mod streaming;
#[path = "core/walker.rs"]
mod walker;
