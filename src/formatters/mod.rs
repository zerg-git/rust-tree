//! 不同显示格式的输出格式化器。

pub mod json;
pub mod streaming_tree;
pub mod table;
pub mod tree;

pub use json::format_json;
pub use table::format_table;
pub use tree::format_tree;
