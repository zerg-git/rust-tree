//! rust-tree - A fast directory tree visualization tool.
//!
//! This is the main entry point for the command-line interface.

use clap::Parser;
use rust_tree::Config;
use std::process;

fn main() {
    // Parse command-line arguments
    let config = Config::parse();

    // Run the tool
    if let Err(e) = rust_tree::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
