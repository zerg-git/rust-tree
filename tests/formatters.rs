//! `formatters`（tree、json、table、streaming_tree 输出）的测试。
//!
//! `tests/formatters.rs` 是 `formatters` 集成测试目标的 crate root，因此每个
//! 子模块都用 `#[path]` 锚定到 `tests/formatters/` 下对应的镜像位置。

#[path = "formatters/json.rs"]
mod json;
#[path = "formatters/streaming_tree.rs"]
mod streaming_tree;
#[path = "formatters/table.rs"]
mod table;
#[path = "formatters/tree.rs"]
mod tree;
