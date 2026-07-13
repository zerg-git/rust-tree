# Rust-Tree 开发文档

本文档为参与 rust-tree 项目开发的人员提供技术细节。

## 目录

- [开发环境](#开发环境)
- [项目架构](#项目架构)
- [核心模块](#核心模块)
- [数据结构](#数据结构)
- [算法](#算法)
- [测试](#测试)
- [构建](#构建)
- [发布流程](#发布流程)

## 开发环境

### 前置要求

- Rust 1.70 或更高版本
- Cargo（随 Rust 一同安装）

### 环境准备

```bash
# Clone the repository
git clone https://github.com/zerg-git/rust-tree
cd rust-tree

# Build the project
cargo build

# Run tests
cargo test

# Install for development
cargo install --path .
```

### 推荐工具

- **rustfmt**：代码格式化（`cargo fmt`）
- **clippy**：代码检查（`cargo clippy`）
- **IDE**：VS Code 搭配 rust-analyzer 扩展

## 项目架构

### 模块组织

```
rust-tree/
├── src/
│   ├── main.rs              # Binary entry point
│   ├── lib.rs               # Library interface
│   ├── config.rs            # CLI configuration (with validate())
│   ├── config/color.rs      # Color modes & schemes
│   ├── core/
│   │   ├── mod.rs           # Module exports
│   │   ├── models.rs        # Core data structures
│   │   ├── walker.rs        # In-memory tree builder (walk_directory)
│   │   ├── streaming.rs     # Unified traversal core (walk_core)
│   │   ├── collector.rs     # Statistics collection
│   │   ├── filter.rs        # Glob-based include/exclude filtering
│   │   └── progress.rs      # Progress bar (indicatif)
│   └── formatters/
│       ├── mod.rs           # Module exports
│       ├── tree.rs          # Tree output
│       ├── json.rs          # JSON output
│       ├── table.rs         # Table output
│       └── streaming_tree.rs # Streaming tree output
```

### 依赖关系图

```
main.rs
  └─> lib.rs
       ├─> config.rs (→ config/color.rs)
       ├─> core/
       │    ├─> models.rs
       │    ├─> walker.rs ──> streaming.rs (walk_core)
       │    ├─> streaming.rs
       │    ├─> collector.rs
       │    ├─> filter.rs
       │    └─> progress.rs
       └─> formatters/
            ├─> tree.rs
            ├─> json.rs
            ├─> table.rs
            └─> streaming_tree.rs ──> streaming.rs (walk_core)
```

## 核心模块

### models.rs

定义所有核心数据结构：

- **FsNode**：表示文件/目录/符号链接节点
- **FsTree**：整个目录树的容器
- **TreeStats**：汇总后的统计数据
- **FileTypeInfo**：按扩展名聚合的文件信息
- **FileEntry**：用于排序后列表的文件条目
- **TreeError**：操作的错误类型

### walker.rs

处理目录遍历并将结果物化为 `FsTree`：

```rust
pub fn walk_directory(
    path: &Path,
    config: &WalkConfig,
    progress: Option<&indicatif::ProgressBar>,
) -> Result<FsTree, TreeError>
```

关键特性：
- 委托给统一的 `core::streaming::walk_core`（无独立的遍历实现）
- 从 `walk_core` 的流中以栈帧方式物化 `FsTree`
- 可选的 `progress` 句柄，按节点推进（显示计数 + 当前目录）
- 支持深度限制、隐藏文件、跟随符号链接，以及按名称/大小/类型排序

### collector.rs

从树中收集统计信息：

```rust
pub fn collect_stats(tree: &FsTree, start_time: Instant, largest_limit: usize) -> TreeStats
```

关键特性：
- 统计文件、目录、符号链接的数量
- 计算总大小
- 按扩展名分组文件（如 `.gitignore` 这类 dotfile 归入 "(no extension)"）
- 通过 `select_nth_unstable_by` 查找最大文件（O(n) 选择 + 前缀排序）
- 度量扫描耗时

### formatters/

#### tree.rs

使用 Unicode 字符生成树状输出：
```
├── file.txt
└── directory/
    └── file.rs
```

#### json.rs

将目录树和统计信息序列化为 JSON：
```json
{
  "tree": {...},
  "stats": {...}
}
```

#### table.rs

使用 `comfy-table` 生成格式化表格：
```
╭──────────┬──────╮
│ Extension │ Count│
╞══════════╪══════╡
│ .rs       │ 11   │
╰──────────┴──────╯
```

## 数据结构

### FsNode

```rust
pub struct FsNode {
    pub name: String,
    pub path: Option<PathBuf>,
    pub node_type: FsNodeType,
    pub size: u64,
    pub depth: usize,
    pub children: Option<Vec<FsNode>>,
}
```

### TreeStats

```rust
pub struct TreeStats {
    pub total_files: usize,
    pub total_directories: usize,
    pub total_symlinks: usize,
    pub total_size: u64,
    pub files_by_extension: HashMap<String, FileTypeInfo>,
    pub largest_files: Vec<FileEntry>,
    pub scan_duration: Duration,
}
```

## 算法

### 目录遍历

walker 采用递归方式：

1. 从根目录开始
2. 对每个条目：
   - 检查是否为隐藏文件（若不显示则跳过）
   - 为条目创建 FsNode
   - 若为目录且在深度限制内：递归
3. 根据配置对子节点排序
4. 返回完整的目录树

### 统计收集

1. 遍历整棵树
2. 按类型（文件、目录、符号链接）计数
3. 累加文件大小
4. 使用 HashMap 按扩展名分组
5. 通过 `select_nth_unstable_by` 选出前 N 个文件，仅对前缀部分排序

### 排序

条目按以下优先级排序：
1. 目录优先于文件
2. 主排序字段（名称、大小或类型）
3. 若设置了反向标志则反转

## 测试

### 运行测试

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_format_tree
```

### 测试结构

每个模块在 `#[cfg(test)]` 模块中包含单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

### 集成测试

将集成测试放在 `tests/` 目录下：

```rust
// tests/integration_test.rs
use rust_tree::*;

#[test]
fn test_full_workflow() {
    // Test complete workflow
}
```

## 构建

### Debug 构建

```bash
cargo build
```

### Release 构建

```bash
cargo build --release
```

### 交叉编译

```bash
# Build for Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Build for macOS
cargo build --release --target x86_64-apple-darwin

# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc
```

## 发布流程

1. 更新 `Cargo.toml` 中的版本号
2. 更新 CHANGELOG
3. 提交改动
4. 创建 git tag
5. 构建 release 二进制
6. 发布到 crates.io（可选）

```bash
# Update version
# Edit Cargo.toml

# Commit
git add -A
git commit -m "Release vX.X.X"

# Tag
git tag vX.X.X

# Publish
cargo publish
```

## 代码风格

遵循 Rust 惯例：
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查
- 用 rustdoc 为公开 API 编写文档
- 为所有公开函数编写测试

## 性能考量

- 使用 `walkdir` 进行高效的目录遍历（从 readdir 缓存获取 file_type）
- 按目录排序（仅对正在处理的目录进行缓冲/排序）
- 使用 `HashMap` 实现 O(1) 的扩展名分组
- `--streaming` 模式：峰值内存为 O(width of the widest directory)；边遍历边输出
- **按需 stat**：当 `need_size == false` 且 `sort_by != Size` 时，文件完全跳过
  `metadata()`（默认的 streaming 路径）。实测全盘基线
  （7.5M entries）：约 86s 实际耗时、约 76MB 峰值 RSS、约 49.5k entries/s。
- **Top-N 选择**：`find_largest_files` 使用 `select_nth_unstable_by` 而非完整排序
- `--progress` 按节点推进，实时显示计数 + 当前目录路径（内存模式与 streaming 模式均支持）
- 并行遍历是未来的方向（会破坏排序输出的顺序）
