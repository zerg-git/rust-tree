# rust-tree

一个用 Rust 编写的快速目录树可视化工具。它能够以多种格式显示目录结构，并提供关于文件分布的全面统计信息。

## 功能特性

- **多种输出格式**
  - 使用 Unicode box-drawing 字符的树形输出
  - 用于程序化处理的 JSON 格式
  - 用于统计摘要的表格格式

- **全面的统计信息**
  - 文件和目录计数
  - 总大小计算
  - 按扩展名分组文件
  - 最大文件列表

- **灵活的选项**
  - 限制扫描深度
  - 按名称、大小或类型排序
  - 显示隐藏文件
  - 显示文件大小
  - 跟随符号链接

## 安装

### 从源码安装

```bash
cargo install --path .
```

### 从源码构建

```bash
git clone <repository-url>
cd rust-tree
cargo build --release
```

生成的二进制文件将位于 `target/release/rust-tree`。

## 使用方法

### 基本用法

```bash
# 显示当前目录树
rust-tree

# 显示指定目录
rust-tree /path/to/directory

# 限制深度为 2 层
rust-tree -d 2 /path/to/directory
```

### 输出格式

```bash
# 树形格式（默认）
rust-tree

# JSON 格式
rust-tree -f json

# 带统计信息的表格格式
rust-tree -f table -S
```

### 显示选项

```bash
# 显示文件大小
rust-tree -s

# 显示隐藏文件
rust-tree -a

# 按文件大小排序（降序）
rust-tree -o size -r

# 按文件类型排序
rust-tree -o type
```

### 示例

```bash
# 显示带大小和统计信息的目录树
rust-tree -s -S

# JSON 输出完整统计信息
rust-tree -f json -S > stats.json

# 仅显示 3 层深度，按大小排序
rust-tree -d 3 -o size -r

# 表格格式显示所有文件（包括隐藏文件）
rust-tree -f table -a -S
```

## 命令行选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `-d, --depth <N>` | 最大递归深度（0 = 无限制） | 0 |
| `-f, --format <FORMAT>` | 输出格式（tree/json/table） | tree |
| `-s, --size` | 显示文件大小 | false |
| `-a, --all` | 显示隐藏文件 | false |
| `-o, --sort <BY>` | 排序方式（name/size/type） | name |
| `-r, --reverse` | 反向排序 | false |
| `-S, --stats` | 显示统计摘要 | false |
| `-L, --follow` | 跟随符号链接 | false |
| `--top-files <N>` | 显示的最大文件数量 | 10 |
| `-h, --help` | 打印帮助信息 | - |
| `-V, --version` | 打印版本信息 | - |

## 输出示例

### 树形格式

```
src/
├── core/
│   ├── models.rs
│   ├── walker.rs
│   └── collector.rs
├── formatters/
│   ├── tree.rs
│   ├── json.rs
│   └── table.rs
├── config.rs
├── lib.rs
└── main.rs
```

### JSON 格式

```json
{
  "tree": {
    "root": {
      "name": "src",
      "type": "directory",
      "children": [...]
    }
  },
  "stats": {
    "total_files": 11,
    "total_directories": 3,
    "total_size": 42944,
    "files_by_extension": {
      ".rs": {
        "count": 11,
        "total_size": 42944,
        "percentage": 100.0
      }
    }
  }
}
```

### 表格格式

```
╭──────────────────╬────────╮
│ Statistics       ║        │
╞══════════════════╪════════╡
│ Total Files      ║ 11     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ Total Directories╩ 3      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ Total Size       ║ 42 KB  │
╰──────────────────┴────────╯
```

## 开发

### 项目结构

```
rust-tree/
├── src/
│   ├── main.rs          # CLI 入口
│   ├── lib.rs           # 库接口
│   ├── config.rs        # 配置
│   ├── core/            # 核心功能
│   │   ├── models.rs    # 数据结构
│   │   ├── walker.rs    # 目录遍历
│   │   └── collector.rs # 统计信息收集
│   └── formatters/      # 输出格式化器
│       ├── tree.rs      # 树形格式
│       ├── json.rs      # JSON 格式
│       └── table.rs     # 表格格式
├── docs/                # 文档
└── tests/               # 测试
```

### 运行测试

```bash
cargo test
```

### 构建

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release
```

## 文档

- [DEVELOPMENT.md](docs/DEVELOPMENT.md) - 技术实现细节
- [USER_MANUAL.md](docs/USER_MANUAL.md) - 完整用户指南
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - 架构设计

## 许可证

MIT OR Apache-2.0

## 贡献

欢迎贡献！请随时提交 Pull Request。
