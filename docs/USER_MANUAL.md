# Rust-Tree 用户手册

rust-tree 命令行工具的完整使用指南。

## 目录

- [简介](#简介)
- [安装](#安装)
- [快速上手](#快速上手)
- [命令行选项](#命令行选项)
- [输出格式](#输出格式)
- [示例](#示例)
- [用例](#用例)
- [排错](#排错)

## 简介

rust-tree 是一个用于可视化目录结构并分析文件统计信息的快速命令行工具。它提供多种输出格式，并给出关于文件的全面信息。

### 主要特性

- **多种输出格式**：tree、json 和 table
- **详细的统计信息**：文件数量、大小以及按扩展名的分组
- **灵活的过滤**：深度限制、隐藏文件、排序选项
- **高性能**：使用 Rust 编写，速度极佳

## 安装

### 方式一：使用 cargo

```bash
cargo install rust-tree
```

### 方式二：从源码构建

```bash
git clone https://github.com/zerg-git/rust-tree
cd rust-tree
cargo install --path .
```

### 方式三：预编译二进制

从 [releases 页面](https://github.com/zerg-git/rust-tree/releases) 下载适合你平台的二进制文件，并将其放入 PATH 中。

## 快速上手

### 基本用法

```bash
# 显示当前目录
rust-tree

# 显示指定目录
rust-tree /path/to/directory

# 将深度限制为 2 层
rust-tree -d 2
```

### 常用模式

```bash
# 显示文件大小
rust-tree -s

# 显示统计信息
rust-tree -S

# 以 JSON 输出
rust-tree -f json
```

## 命令行选项

### 用法概要

```bash
rust-tree [OPTIONS] [DIRECTORY]
```

### 参数

| 参数 | 说明 | 默认值 |
|----------|-------------|---------|
| `DIRECTORY` | 目标目录路径 | 当前目录 |

### 选项

| 简写 | 全写 | 说明 | 默认值 |
|-------|------|-------------|---------|
| `-d` | `--depth <N>` | 最大递归深度（0 = 不限制） | 0 |
| `-f` | `--format <FORMAT>` | 输出格式（tree/json/table） | tree |
| `-s` | `--size` | 显示文件大小 | false |
| `-a` | `--all` | 显示隐藏文件 | false |
| `-o` | `--sort <BY>` | 排序字段（name/size/type） | name |
| `-r` | `--reverse` | 反转排序顺序 | false |
| `-S` | `--stats` | 显示统计信息（json/table 中始终包含） | false |
| `-L` | `--follow` | 跟随符号链接 | false |
| | `--top-files <N>` | 统计中显示的最大文件数量 | 10 |
| | `--color <WHEN>` | 颜色模式（always/never/auto） | auto |
| | `--color-scheme <SCHEME>` | 颜色方案（none/basic/extended） | basic |
| `-p` | `--progress` | 显示实时进度条（节点计数 + 当前路径） | false |
| `-e` | `--exclude <PATTERN>` | 排除匹配 glob 模式的条目（可重复） | none |
| | `--include-only <PATTERN>` | 只保留匹配 glob 模式的文件 | none |
| | `--exclude-common <LANGUAGE>` | 应用某种语言的常见排除规则（rust/node/nodejs/javascript/python/common）。未知语言会报错。 | none |
| | `--streaming` | 流式模式：低内存，O(最宽目录宽度)。不能与 `--stats`/`-f json`/`-f table` 同用。 | false |
| `-h` | `--help` | 打印帮助 | - |
| `-V` | `--version` | 打印版本 | - |

### Progress（`-p`、`--progress`）

在扫描时显示一个实时 spinner，报告当前已扫描的节点数量和当前目录路径。在默认（内存）模式和 `--streaming` 模式下均生效。

### Exclude / Include 模式

`-e`/`--exclude` 接受 glob 模式（可重复），并跳过匹配的条目（文件和目录）。`--include-only` 只保留匹配某个模式的文件（目录仍会进入遍历，以便更深层级的匹配仍可达）。`--exclude-common <LANGUAGE>` 应用一组常见排除预设。支持的语言：`rust`、`node`、`nodejs`、`javascript`、`python`、`common`。未知语言会被拒绝并报错（不会被静默忽略）。

### Streaming 模式（`--streaming`）

在遍历过程中输出 tree，同一时刻只保留一个目录的条目在内存中（峰值内存 O(最宽目录的宽度)）。非常适合超大型目录树。因为它不会把整棵 tree 物化到内存中，所以无法计算统计信息——将 `--streaming` 与 `--stats`（或会隐含统计信息的 `-f json`/`-f table`）同用会被显式拒绝。在默认的流式路径下（不带 `--show-size`、按 name 排序），会完全跳过 per-file `stat` 调用以提升速度；`--show-size` 或 `--sort size` 会按需重新启用 stat。

### 输出格式取值

| 取值 | 说明 |
|-------|-------------|
| `tree` | 使用 Unicode 字符的树形输出 |
| `json` | JSON 格式（包含 tree 和统计信息） |
| `table` | 以表格形式展示统计信息 |

### 排序字段取值

| 取值 | 说明 |
|-------|-------------|
| `name` | 按文件/目录名排序（默认） |
| `size` | 按文件大小排序 |
| `type` | 按文件类型/扩展名排序 |

## 输出格式

### tree 格式

默认格式使用 Unicode 制表符展示目录层级结构。

```
project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── test.rs
└── Cargo.toml
```

**带文件大小（`-s`）**：
```
project/
├── src/ (2 files)
│   ├── main.rs (1.2 KB)
│   └── lib.rs (3.4 KB)
└── Cargo.toml (256 B)
```

**带统计信息（`-S`）**：
```
project/
└── ...

12 files, 3 directories, 15.2 KB total
```

### json 格式

用于程序化处理的结构化输出。

```json
{
  "tree": {
    "root": {
      "name": "project",
      "type": "directory",
      "children": [...]
    },
    "max_depth": 3
  },
  "stats": {
    "total_files": 12,
    "total_directories": 3,
    "total_symlinks": 0,
    "total_size": 15563,
    "files_by_extension": {
      ".rs": {
        "count": 8,
        "total_size": 12000,
        "percentage": 77.1
      }
    },
    "largest_files": [...],
    "scan_duration_ms": 5
  }
}
```

### table 格式

以格式化表格展示统计信息。

```
╭───────────────────╬──────────╮
│ Statistics        ║          │
╞═══════════════════╪══════════╡
│ Total Files       ║ 12       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Total Directories ║ 3        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Total Size        ║ 15.2 KB  │
╰───────────────────┴──────────╯

Files by Extension
╭─────────────╬───────╬─────────╬───────────╮
│ Extension   ║ Count ║ Size    ║ Percentage│
╞═════════════╪═══════╪═════════╪═══════════╡
│ .rs         ║ 8     ║ 12 KB   ║ 77.1%     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┌┤
│ .toml       ║ 1     ║ 256 B   ║ 1.6%      │
╰─────────────┴───────┴─────────┴───────────╯
```

## 示例

### 分析项目

```bash
# 显示带文件大小的项目结构
rust-tree -s ~/projects/my-app

# 以表格形式显示统计信息
rust-tree -f table -S ~/projects/my-app

# 查找最大的文件
rust-tree -f json -S ~/projects/my-app | jq '.stats.largest_files'
```

### 快速浏览目录

```bash
# 只显示 2 层深度
rust-tree -d 2

# 包含隐藏文件，按大小排序
rust-tree -a -o size -r

# 紧凑的统计信息
rust-tree -f table -S
```

### 导出用于分析

```bash
# 导出为 JSON
rust-tree -f json -S > project-stats.json

# 将 tree 导出为文本文件
rust-tree > directory-tree.txt

# 获取文件数量
rust-tree -f json | jq '.stats.total_files'
```

### 比较目录

```bash
# 比较两个目录
rust-tree -S dir1
rust-tree -S dir2

# 两者都导出以便比较
rust-tree -f json dir1 > stats1.json
rust-tree -f json dir2 > stats2.json
```

## 用例

### 1. 项目文档

为 README 文件生成目录树：

```bash
rust-tree -d 2 > README-tree.txt
```

### 2. 磁盘占用分析

在目录中查找大文件：

```bash
rust-tree -f table -S -o size -r
```

### 3. 代码审查

获取项目结构的概览：

```bash
rust-tree -d 3 ~/projects/review-project
```

### 4. 备份规划

查看哪些内容会被纳入备份：

```bash
rust-tree -a -S ~/documents
```

### 5. CI/CD 集成

在构建流水线中生成统计信息：

```bash
rust-tree -f json -S > build-stats.json
```

## 排错

### 权限被拒绝

如果遇到权限错误：

```bash
# 使用 sudo（Linux/macOS）
sudo rust-tree /root/directory

# 或排除系统目录
rust-tree ~/projects
```

### 输出过多

限制输出量：

```bash
# 限制深度
rust-tree -d 2

# 只显示特定格式
rust-tree -f table
```

### 隐藏文件不显示

使用 `-a` flag：

```bash
rust-tree -a
```

### 大型目录下性能缓慢

对于超大型目录树，使用流式模式（低内存、默认跳过 per-file `stat`），并可选地查看进度：

```bash
rust-tree --streaming --progress -d 3 /usr
```

如果只想要快速摘要而非整棵 tree：

```bash
rust-tree -d 3 -f table -S
```

### 符号链接导致循环

工具默认会检测 symlink 循环。要跟随 symlinks：

```bash
rust-tree -L
```

### JSON 解析错误

确保你的 JSON 解析器能处理该输出：

```bash
# 使用 jq 美化输出
rust-tree -f json | jq '.'

# 使用 python
rust-tree -f json | python -m json.tool
```

## 技巧与窍门

### 与其他工具组合

```bash
# 按扩展名统计文件数
rust-tree -f json | jq '.stats.files_by_extension | keys'

# 查找总大小
rust-tree -f json | jq '.stats.total_size'

# 获取最大文件路径
rust-tree -f json | jq '.stats.largest_files[0].path'
```

### 创建别名

在 `.bashrc` 或 `.zshrc` 中添加：

```bash
alias tree='rust-tree'
alias trees='rust-tree -s'
alias tree-stats='rust-tree -f table -S'
```

### Shell 集成

```bash
# 从 tree 输出中 cd 进入某个目录
cd $(rust-tree -f json | jq -r '.tree.root.children[0].path')
```

## 支持

如需反馈问题、提问或贡献，请访问：
- GitHub: https://github.com/zerg-git/rust-tree
- 文档: https://docs.rs/rust-tree
