//! Output formatters for different display formats.

pub mod tree;
pub mod json;
pub mod table;

pub use tree::format_tree;
pub use json::format_json;
pub use table::format_table;
