//! rust-tree - 一个快速的目录树可视化工具。
//!
//! 这是命令行界面的主入口。

use clap::Parser;
use rust_tree::Config;
use std::process;

fn main() {
    // 解析命令行参数
    let config = Config::parse();

    // 运行工具
    if let Err(e) = rust_tree::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
