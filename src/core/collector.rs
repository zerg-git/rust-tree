//! Statistics collection from file system trees.

use std::collections::HashMap;
use std::time::Instant;
use crate::core::models::{FsTree, FsNode, TreeStats, FileTypeInfo, FileEntry};

/// Default number of largest files to track.
const DEFAULT_MAX_LARGEST: usize = 10;

/// Collect statistics from a file system tree.
///
/// # Arguments
///
/// * `tree` - The file system tree to analyze
/// * `start_time` - When the scan started (for duration calculation)
///
/// # Returns
///
/// A `TreeStats` object containing all collected statistics.
pub fn collect_stats(tree: &FsTree, start_time: Instant) -> TreeStats {
    let mut stats = TreeStats::new();

    // Collect all files and directories
    let mut all_files: Vec<&FsNode> = Vec::new();
    count_nodes(&tree.root, &mut stats, &mut all_files);

    // Group by extension
    stats.files_by_extension = analyze_by_extension(&all_files, stats.total_size);

    // Find largest files
    stats.largest_files = find_largest_files(&all_files, DEFAULT_MAX_LARGEST);

    // Calculate scan duration
    stats.scan_duration = start_time.elapsed();

    stats
}

/// Recursively count nodes in the tree.
fn count_nodes<'a>(node: &'a FsNode, stats: &mut TreeStats, all_files: &mut Vec<&'a FsNode>) {
    match node.node_type {
        crate::core::models::FsNodeType::Directory => {
            stats.total_directories += 1;
        }
        crate::core::models::FsNodeType::File => {
            stats.total_files += 1;
            stats.total_size += node.size;
            all_files.push(node);
        }
        crate::core::models::FsNodeType::Symlink => {
            stats.total_symlinks += 1;
        }
    }

    if let Some(children) = &node.children {
        for child in children {
            count_nodes(child, stats, all_files);
        }
    }
}

/// Analyze files by extension.
///
/// Returns a HashMap mapping extensions to file type information.
fn analyze_by_extension(files: &[&FsNode], total_size: u64) -> HashMap<String, FileTypeInfo> {
    let mut by_ext: HashMap<String, (usize, u64)> = HashMap::new();

    for file in files {
        let ext = file.extension().unwrap_or_else(|| "(no extension)".to_string());

        let entry = by_ext.entry(ext).or_insert((0, 0));
        entry.0 += 1;  // count
        entry.1 += file.size;  // total size
    }

    // Convert to FileTypeInfo with percentages
    by_ext.into_iter()
        .map(|(ext, (count, size))| {
            let percentage = if total_size > 0 {
                (size as f64 / total_size as f64) * 100.0
            } else {
                0.0
            };

            let info = FileTypeInfo {
                extension: ext.clone(),
                count,
                total_size: size,
                percentage,
            };

            (ext, info)
        })
        .collect()
}

/// Find the N largest files.
///
/// # Arguments
///
/// * `files` - Slice of file nodes to analyze
/// * `limit` - Maximum number of files to return
///
/// # Returns
///
/// A vector of `FileEntry` objects, sorted by size (largest first).
fn find_largest_files(files: &[&FsNode], limit: usize) -> Vec<FileEntry> {
    if files.is_empty() {
        return Vec::new();
    }

    // Collect all entries
    let mut entries: Vec<FileEntry> = files
        .iter()
        .map(|file| FileEntry::new(
            file.name.clone(),
            file.path.clone().unwrap_or_default(),
            file.size,
        ))
        .collect();

    // Sort by size (descending)
    entries.sort_by(|a, b| b.size.cmp(&a.size));

    // Take top N
    entries.truncate(limit);
    entries
}

/// Get a flat list of all file nodes in the tree.
///
/// # Arguments
///
/// * `tree` - The file system tree
///
/// # Returns
///
/// A vector containing all file nodes.
pub fn get_all_files(tree: &FsTree) -> Vec<FsNode> {
    let mut files = Vec::new();
    collect_files_recursive(&tree.root, &mut files);
    files
}

/// Recursively collect file nodes.
fn collect_files_recursive(node: &FsNode, files: &mut Vec<FsNode>) {
    if node.is_file() {
        files.push(node.clone());
    }

    if let Some(children) = &node.children {
        for child in children {
            collect_files_recursive(child, files);
        }
    }
}

/// Get a flat list of all directory nodes in the tree.
///
/// # Arguments
///
/// * `tree` - The file system tree
///
/// # Returns
///
/// A vector containing all directory nodes.
pub fn get_all_directories(tree: &FsTree) -> Vec<FsNode> {
    let mut dirs = Vec::new();
    collect_dirs_recursive(&tree.root, &mut dirs);
    dirs
}

/// Recursively collect directory nodes.
fn collect_dirs_recursive(node: &FsNode, dirs: &mut Vec<FsNode>) {
    if node.is_directory() {
        dirs.push(node.clone());
    }

    if let Some(children) = &node.children {
        for child in children {
            collect_dirs_recursive(child, dirs);
        }
    }
}

/// Calculate the total number of nodes in the tree.
pub fn total_node_count(tree: &FsTree) -> usize {
    count_nodes_recursive(&tree.root)
}

/// Recursively count all nodes.
fn count_nodes_recursive(node: &FsNode) -> usize {
    let mut count = 1;

    if let Some(children) = &node.children {
        for child in children {
            count += count_nodes_recursive(child);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::FsNodeType;

    #[test]
    fn test_find_largest_files() {
        let files = vec![
            FsNode::new("a.txt".into(), "/a.txt".into(), FsNodeType::File, 100, 0),
            FsNode::new("b.txt".into(), "/b.txt".into(), FsNodeType::File, 500, 0),
            FsNode::new("c.txt".into(), "/c.txt".into(), FsNodeType::File, 200, 0),
            FsNode::new("d.txt".into(), "/d.txt".into(), FsNodeType::File, 1000, 0),
            FsNode::new("e.txt".into(), "/e.txt".into(), FsNodeType::File, 50, 0),
        ];

        let refs: Vec<&FsNode> = files.iter().collect();
        let largest = find_largest_files(&refs, 3);

        assert_eq!(largest.len(), 3);
        assert_eq!(largest[0].size, 1000);
        assert_eq!(largest[1].size, 500);
        assert_eq!(largest[2].size, 200);
    }

    #[test]
    fn test_analyze_by_extension() {
        let files = vec![
            FsNode::new("file.rs".into(), "/file.rs".into(), FsNodeType::File, 100, 0),
            FsNode::new("doc.md".into(), "/doc.md".into(), FsNodeType::File, 50, 0),
            FsNode::new("main.rs".into(), "/main.rs".into(), FsNodeType::File, 200, 0),
        ];

        let refs: Vec<&FsNode> = files.iter().collect();
        let by_ext = analyze_by_extension(&refs, 350);

        assert_eq!(by_ext.len(), 2);
        assert_eq!(by_ext.get(".rs").unwrap().count, 2);
        assert_eq!(by_ext.get(".md").unwrap().count, 1);
    }
}
