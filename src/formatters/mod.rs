//! 不同显示格式的输出格式化器。

pub mod tree;
pub mod json;
pub mod table;
pub mod streaming_tree;

pub use tree::format_tree;
pub use json::format_json;
pub use table::format_table;
