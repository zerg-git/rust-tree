# Rust-Tree 项目架构

本文档介绍 rust-tree 项目的架构设计和核心概念，帮助开发者快速理解项目结构。

## 目录

- [快速概览](#快速概览)
- [项目结构](#项目结构)
- [核心模块](#核心模块)
- [数据流](#数据流)
- [关键数据结构](#关键数据结构)
- [扩展指南](#扩展指南)

## 快速概览

### rust-tree 是什么？

rust-tree 是一个**用 Rust 编写的快速目录树可视化工具**，可以：

- 以多种格式（树形、JSON、表格）显示目录结构
- 提供文件统计信息（按扩展名分组、最大文件列表等）
- 支持灵活的排序、过滤和显示选项

### 架构原则

```
┌────────────────────────────────────────────────────────┐
│                    设计原则                            │
├────────────────────────────────────────────────────────┤
│  1. 模块分离     - 每个模块职责单一、清晰              │
│  2. 库优先       - 可作为库使用，CLI 只是上层应用       │
│  3. 可扩展性     - 易于添加新格式、新统计、新选项      │
│  4. 可测试性     - 每个模块可独立测试                  │
└────────────────────────────────────────────────────────┘
```

## 项目结构

```
rust-tree/
├── src/
│   ├── main.rs              # CLI 入口，解析参数并调用库
│   ├── lib.rs               # 库接口，协调各模块工作
│   │
│   ├── config.rs            # 配置模块（CLI 参数解析）
│   │
│   ├── core/                # 核心功能模块
│   │   ├── mod.rs           # 模块导出
│   │   ├── models.rs        # 数据结构定义
│   │   ├── walker.rs        # 目录遍历逻辑
│   │   └── collector.rs     # 统计信息收集
│   │
│   └── formatters/          # 输出格式化器
│       ├── mod.rs           # 模块导出
│       ├── tree.rs          # 树形格式
│       ├── json.rs          # JSON 格式
│       └── table.rs         # 表格格式
│
├── docs/                    # 文档目录
│   ├── ARCHITECTURE.md      # 详细架构文档（英文）
│   ├── ARCHITECTURE.zh-CN.md# 架构文档（中文，本文件）
│   ├── USER_MANUAL.md       # 用户手册
│   └── DEVELOPMENT.md       # 开发指南
│
└── tests/                   # 集成测试
```

## 核心模块

### 1. Config 模块 ([config.rs](../src/config.rs))

**职责**：解析和存储配置

```rust
// 命令行参数结构
pub struct Config {
    pub path: PathBuf,           // 目标路径
    pub max_depth: usize,        // 最大深度
    pub format: OutputFormat,    // 输出格式
    pub show_size: bool,         // 显示大小
    pub show_hidden: bool,       // 显示隐藏文件
    pub sort_by: SortBy,         // 排序方式
    pub reverse: bool,           // 反向排序
    // ...
}

// 输出格式枚举
pub enum OutputFormat {
    Tree,   // 树形（默认）
    Json,   // JSON
    Table,  // 表格
}
```

**关键方法**：
- `to_walk_config()` - 转换为内部使用的 `WalkConfig`
- `should_show_stats()` - 判断是否显示统计信息

### 2. Core 模块

#### 2.1 Models ([models.rs](../src/core/models.rs))

**职责**：定义核心数据结构

```rust
// 文件系统节点
pub struct FsNode {
    pub name: String,              // 名称
    pub path: Option<PathBuf>,     // 完整路径
    pub node_type: FsNodeType,     // 类型（文件/目录/符号链接）
    pub size: u64,                 // 大小（字节）
    pub depth: usize,              // 深度
    pub children: Option<Vec<FsNode>>, // 子节点
}

// 文件系统树
pub struct FsTree {
    pub root: FsNode,      // 根节点
    pub max_depth: usize,  // 最大深度
}

// 统计信息
pub struct TreeStats {
    pub total_files: usize,                        // 文件总数
    pub total_directories: usize,                  // 目录总数
    pub total_size: u64,                           // 总大小
    pub files_by_extension: HashMap<String, ...>,  // 按扩展名分组
    pub largest_files: Vec<FileEntry>,             // 最大文件列表
    pub scan_duration: Duration,                   // 扫描耗时
}

// 错误类型
pub enum TreeError {
    Io(std::io::Error),
    PathNotFound(PathBuf),
    NotADirectory(PathBuf),
    PermissionDenied(PathBuf),
    Json(String),
    Other(String),
}
```

#### 2.2 Walker ([walker.rs](../src/core/walker.rs))

**职责**：遍历目录结构

```rust
// 核心函数
pub fn walk_directory(path: &Path, config: &WalkConfig) -> Result<FsTree, TreeError>

// 内部工作流程
walk_directory()
    │
    ├─ 验证路径存在性和类型
    │
    ├─ walk_recursive(path, 0, config)
    │   │
    │   ├─ 读取文件元数据
    │   ├─ 创建 FsNode
    │   ├─ 如果是目录:
    │   │   └─ collect_children()
    │   │       ├─ WalkDir 遍历
    │   │       ├─ 过滤隐藏文件
    │   │       ├─ 递归处理子项
    │   │       └─ sort_entries() 排序
    │   └─ 返回 FsNode
    │
    └─ 返回 FsTree
```

**排序选项**：
- `Name` - 按名称排序（目录优先）
- `Size` - 按大小排序（目录优先）
- `Type` - 按扩展名排序（目录优先）

#### 2.3 Collector ([collector.rs](../src/core/collector.rs))

**职责**：收集统计信息

```rust
// 核心函数
pub fn collect_stats(tree: &FsTree, start_time: Instant) -> TreeStats

// 工作流程
collect_stats()
    │
    ├─ count_nodes() - 递归统计节点
    │   ├─ 统计文件数、目录数、符号链接数
    │   ├─ 累计文件大小
    │   └─ 收集所有文件引用
    │
    ├─ analyze_by_extension() - 按扩展名分组
    │   └─ 返回 HashMap<String, FileTypeInfo>
    │
    ├─ find_largest_files() - 查找最大文件
    │   └─ 返回 Vec<FileEntry>（按大小降序）
    │
    └─ 计算扫描耗时
```

### 3. Formatters 模块

**职责**：生成各种格式的输出

| 格式化器 | 文件 | 输出格式 |
|---------|------|---------|
| Tree | [tree.rs](../src/formatters/tree.rs) | Unicode 树形结构 |
| JSON | [json.rs](../src/formatters/json.rs) | JSON 对象 |
| Table | [table.rs](../src/formatters/table.rs) | 表格统计 |

```rust
// 公共接口
pub fn format_tree(node: &FsNode, show_size: bool) -> String
pub fn format_json(tree: &FsTree, stats: &TreeStats, pretty: bool) -> Result<String, TreeError>
pub fn format_table(stats: &TreeStats) -> String
```

**设计特点**：
- 每个格式化器完全独立
- 无共享状态
- 返回字符串便于测试
- 使用外部库（comfy-table、serde_json）处理格式化

## 数据流

### 主流程

```
用户输入
    │
    ▼
┌─────────────┐
│   Config    │ 解析命令行参数
└─────┬───────┘
      │
      ▼
┌─────────────┐
│   Walker    │ 遍历文件系统
└─────┬───────┘
      │
      ▼ FsTree
┌─────────────┐
│  Collector  │ 收集统计信息
└─────┬───────┘
      │
      ▼ TreeStats
┌─────────────┐
│ Formatters  │ 生成输出
└─────┬───────┘
      │
      ▼ String
   显示输出
```

### 库入口 ([lib.rs](../src/lib.rs))

```rust
pub fn run(config: Config) -> Result<(), TreeError> {
    // 1. 遍历目录
    let tree = walk_directory(&config.path, &config.to_walk_config())?;

    // 2. 收集统计
    let stats = collect_stats(&tree, start_time);

    // 3. 格式化输出
    let output = match config.format {
        OutputFormat::Tree => format_tree(...),
        OutputFormat::Json => format_json(...),
        OutputFormat::Table => format_table(...),
    };

    // 4. 打印结果
    print!("{}", output);
    Ok(())
}
```

## 关键数据结构

### 类型关系图

```
┌─────────────────────────────────────────────────────────┐
│                      FsTree                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   FsNode                        │   │
│  │  ├── name: String                               │   │
│  │  ├── node_type: FsNodeType (File/Dir/Symlink)   │   │
│  │  ├── size: u64                                  │   │
│  │  └── children: Option<Vec<FsNode>>  ◄─────────┐ │   │
│  └───────────────────────────────────────────────│─┘   │
└────────────────────────────────────────────────────┼─────┘
                                                   │
                       递归结构 ─────────────────────┘
```

### 统计数据流

```
FsTree (树结构)
    │
    ▼ 递归遍历
TreeStats (统计)
    ├── total_files: usize
    ├── total_directories: usize
    ├── total_size: u64
    ├── files_by_extension: HashMap<String, FileTypeInfo>
    │   └── ".rs" -> { count: 10, total_size: 1024, ... }
    └── largest_files: Vec<FileEntry>
        └─ [ { name: "main.rs", size: 2048 }, ... ]
```

## 扩展指南

### 添加新的输出格式

```rust
// 1. 在 formatters/ 目录创建新文件
// src/formatters/xml.rs

pub fn format_xml(tree: &FsTree, stats: &TreeStats) -> Result<String, TreeError> {
    // 实现格式化逻辑
    Ok(String::from("<xml>...</xml>"))
}

// 2. 在 formatters/mod.rs 中导出
pub mod xml;
pub use xml::format_xml;

// 3. 在 config.rs 中添加枚举值
pub enum OutputFormat {
    Tree,
    Json,
    Table,
    Xml,  // 新增
}

// 4. 在 lib.rs 中处理新格式
OutputFormat::Xml => format_xml(&tree, &stats)?,
```

### 添加新的统计字段

```rust
// 在 models.rs 的 TreeStats 中添加字段
pub struct TreeStats {
    // ... 现有字段
    pub oldest_files: Vec<FileEntry>,  // 新增
}

// 在 collector.rs 中收集数据
pub fn collect_stats(tree: &FsTree, start_time: Instant) -> TreeStats {
    let mut stats = TreeStats::new();
    // ...
    stats.oldest_files = find_oldest_files(&all_files, 10);  // 新增
    stats
}
```

### 添加新的排序选项

```rust
// 在 config.rs 中添加枚举值
pub enum SortBy {
    Name,
    Size,
    Type,
    Modified,  // 新增：按修改时间
}

// 在 walker.rs 中实现排序逻辑
SortField::Modified => {
    entries.sort_by(|a, b| {
        // 获取修改时间并比较
        get_modified_time(a).cmp(&get_modified_time(b))
    });
}
```

## 性能特点

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| 目录遍历 | O(n) | n = 文件系统节点数 |
| 排序 | O(n log n) | 每个目录独立排序 |
| 扩展名分组 | O(n) | HashMap 查找 O(1) |
| 最大文件 | O(n log k) | k = top 数量 |

## 依赖关系

```
主要依赖：
├── clap (4.5)      - 命令行参数解析
├── walkdir (2.5)   - 高效目录遍历
├── serde (1.0)     - 序列化支持
├── serde_json      - JSON 格式化
├── comfy-table     - 表格格式化
├── humansize       - 人类可读的文件大小
├── thiserror       - 错误处理
└── anyhow          - 错误处理
```

## 相关文档

- [ARCHITECTURE.md](ARCHITECTURE.md) - 详细架构文档（英文）
- [USER_MANUAL.md](USER_MANUAL.md) - 用户手册
- [DEVELOPMENT.md](DEVELOPMENT.md) - 开发指南
- [README.zh-CN.md](../README.zh-CN.md) - 项目说明
