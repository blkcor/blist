<p align="center">
  <img src="assets/logo.svg" alt="BList Logo" width="200" height="200"/>
</p>

# BList - Better ListLI

[![Crates.io](https://img.shields.io/crates/v/blist.svg)](https://crates.io/crates/blist)
[![Downloads](https://img.shields.io/crates/d/blist.svg)](https://crates.io/crates/blist)
[![License](https://img.shields.io/crates/l/blist.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg?logo=rust)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/actions/workflow/status/blkcor/bls/rust.yml?branch=master)](https://github.com/blkcor/bls/actions)

一个功能强大的现代化文件列表工具，使用 Rust 编写，提供比传统 `ls` 命令更丰富的功能。

## 🚀 功能特色

### 📋 多种输出格式

- **表格格式** (`--format table`) - 默认的现代化表格显示
- **长格式** (`--format long`) - 类似 `ls -l` 的详细信息显示
- **树状格式** (`--format tree`) - 层次化的树状结构显示
- **JSON 格式** (`--format json`) - 机器可读的 JSON 输出

### 🔍 强大的过滤功能

- **扩展名过滤** (`--extensions rs,txt,md`) - 按文件扩展名过滤
- **大小过滤** (`--min-size 1024 --max-size 1048576`) - 按文件大小范围过滤
- **类型过滤** (`--dirs-only` / `--files-only`) - 只显示目录或文件
- **Glob 模式** (`--glob "*.rs"`) - 使用通配符模式过滤
- **隐藏文件** (`--all`) - 显示隐藏文件和目录

### 📊 智能排序

- **按名称排序** (`--sort name`) - 自然排序，正确处理数字
- **按大小排序** (`--sort size`) - 按文件大小排序
- **按时间排序** (`--sort modified/created`) - 按修改或创建时间排序
- **按类型排序** (`--sort type`) - 按文件类型排序
- **排序顺序** (`--order asc/desc`) - 升序或降序

### 🌲 递归遍历

- **递归列表** (`--recursive`) - 递归遍历子目录
- **深度控制** (`--max-depth 3`) - 限制递归深度
- **目录优先** - 递归模式下目录优先排序

### 📏 人性化显示

- **可读大小** (`--human-readable`) - 显示 KB、MB、GB 等单位
- **详细信息** (`--long`) - 显示权限、所有者、时间戳等
- **统计摘要** (`--summary`) - 显示文件统计信息
- **颜色支持** (默认启用，`--no-color` 禁用)

## 📦 安装

### 从 crates.io 安装

```bash
cargo install blist
```

### 从源码编译

```bash
git clone https://github.com/blkcor/blist
cd blist
cargo build --release
```

编译后的可执行文件位于 `target/release/blist`

### cargo install

```bash
cargo install blist
```

## 🛠️ 使用方法

### 基本用法

```bash
# 列出当前目录
blist

# 列出指定目录
blist --path /home/user/documents

# 显示隐藏文件
blist --all
```

### 输出格式

```bash
# 表格格式（默认）
blist --format table

# 长格式显示
blist --format long

# 树状格式
blist --format tree --recursive

# JSON 输出
blist --format json
```

### 过滤功能

```bash
# 只显示 Rust 文件
blist --extensions rs

# 显示多种扩展名
blist --extensions rs,txt,md

# 按大小过滤（大于 1KB 的文件）
blist --min-size 1024

# 只显示目录
blist --dirs-only

# 使用 glob 模式
blist --glob "test*"
```

### 排序功能

```bash
# 按大小排序（降序）
blist --sort size --order desc

# 按修改时间排序
blist --sort modified

# 按文件类型排序
blist --sort type
```

### 递归遍历

```bash
# 递归列出所有文件
blist --recursive

# 限制递归深度
blist --recursive --max-depth 2

# 递归显示树状结构
blist --recursive --format tree
```

### 高级功能

```bash
# 人性化大小显示 + 统计摘要
blist --human-readable --summary

# 组合多个选项
blist --recursive --extensions rs --sort size --order desc --human-readable

# 长格式显示大文件
blist --format long --min-size 100000 --sort size --order desc
```

## 📊 输出示例

### 表格格式

```
╭────────────┬──────┬──────────┬─────────────────────┬─────────────╮
│ Name       │ Type │ Size     │ Modified            │ Permissions │
├────────────┼──────┼──────────┼─────────────────────┼─────────────┤
│ Cargo.lock │ File │ 18.6 kiB │ 2025-07-07 02:54:50 │ -rw-r--r--  │
│ Cargo.toml │ File │ 951 B    │ 2025-07-07 02:54:47 │ -rw-r--r--  │
│ src        │ Dir  │ 384 B    │ 2025-07-07 02:58:11 │ drwxr-xr-x  │
╰────────────┴──────┴──────────┴─────────────────────┴─────────────╯
```

### 长格式

```
 Permissions   Owner        Group   Size    Modified              Name
 -rw-r--r--    chenzilong   staff   18996   2025-07-07 02:54:50   Cargo.lock
 -rw-r--r--    chenzilong   staff   951     2025-07-07 02:54:47   Cargo.toml
 drwxr-xr-x    chenzilong   staff   384     2025-07-07 02:58:11   src
```

### 统计摘要

```
Summary:
  Files: 8
  Directories: 1
  Total Size: 52.1 kiB

File Statistics:
  Files: 8
  Directories: 1
  Symlinks: 0
  Hidden: 0
  Total Size: 52.1 kiB
```

## 🎨 颜色主题

BList 支持智能颜色主题，根据文件类型自动着色：

- **目录** - 蓝色加粗
- **可执行文件** - 绿色加粗
- **源代码文件** - 根据语言不同颜色
  - Rust (`.rs`) - 亮红色
  - Python (`.py`) - 亮黄色
  - JavaScript/TypeScript - 黄色
- **配置文件** - 黄色
- **文档文件** - 白色
- **图片文件** - 洋红色
- **压缩文件** - 红色加粗

## 🔧 命令行选项

| 选项                  | 短选项 | 描述                                       |
| --------------------- | ------ | ------------------------------------------ |
| `--path <PATH>`       | `-p`   | 指定要列出的目录路径                       |
| `--format <FORMAT>`   | `-f`   | 输出格式 (table/long/tree/json)            |
| `--all`               | `-a`   | 显示隐藏文件和目录                         |
| `--recursive`         | `-r`   | 递归列出目录                               |
| `--max-depth <DEPTH>` | `-d`   | 最大递归深度                               |
| `--sort <FIELD>`      | `-s`   | 排序字段 (name/size/modified/created/type) |
| `--order <ORDER>`     | `-o`   | 排序顺序 (asc/desc)                        |
| `--extensions <EXTS>` | `-e`   | 按扩展名过滤                               |
| `--min-size <SIZE>`   |        | 最小文件大小（字节）                       |
| `--max-size <SIZE>`   |        | 最大文件大小（字节）                       |
| `--human-readable`    | `-H`   | 人性化大小显示                             |
| `--long`              | `-l`   | 详细信息显示                               |
| `--no-color`          |        | 禁用颜色输出                               |
| `--dirs-only`         |        | 只显示目录                                 |
| `--files-only`        |        | 只显示文件                                 |
| `--glob <PATTERN>`    | `-g`   | Glob 模式过滤                              |
| `--summary`           |        | 显示统计摘要                               |

## 🏗️ 项目结构

```
src/
├── main.rs          # 主程序入口
├── cli.rs           # 命令行参数解析
├── file_entry.rs    # 文件条目数据结构
├── file_ops.rs      # 文件系统操作
├── output.rs        # 输出格式化
├── sorting.rs       # 排序功能
├── filtering.rs     # 过滤功能
├── colors.rs        # 颜色主题
└── size_utils.rs    # 大小格式化工具
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

本项目采用 MIT 许可证。

## 🔄 版本历史

### v0.1.0

- ✅ 基本文件列表功能
- ✅ 多种输出格式支持
- ✅ 强大的过滤和排序功能
- ✅ 递归目录遍历
- ✅ 人性化显示选项
- ✅ 颜色主题支持
- ✅ 详细的文件信息显示

---

**BList** - 让文件列表变得更简单、更强大！ 🚀
