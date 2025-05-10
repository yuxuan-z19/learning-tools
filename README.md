# Rustlings 本地评测工具

这是一个用于在本地对 Rustlings 练习进行评测的工具，无需依赖 GitHub Actions。

## 功能特点

- 支持评测单个 Rustlings 练习文件
- 支持批量评测整个目录下的所有 Rustlings 练习
- 提供详细的评测结果和统计信息
- 将评测结果保存为 JSON 文件，方便后续分析
- 友好的命令行界面和进度显示

## 安装步骤

1. 确保已安装 Rust 和 Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 克隆本仓库

```bash
git clone https://github.com/yourusername/learning-tools.git
```

3. 编译项目

```bash
cargo build --release
```

## 使用方法

### 评测所有练习

```bash
# 评测当前目录下的所有 Rustlings 练习
cargo run -- grade

# 评测指定目录下的所有 Rustlings 练习
cargo run -- grade -p /path/to/rustlings/exercises

# 显示详细输出
cargo run -- grade -p /path/to/rustlings/exercises -v
```

### 评测单个练习

```bash
# 评测单个 Rustlings 练习文件
cargo run -- grade-single -f /path/to/rustlings/exercises/variables1.rs

# 显示详细输出
cargo run -- grade-single -f /path/to/rustlings/exercises/variables1.rs -v
```

## 评测结果

评测完成后，工具会在当前目录生成 `rustlings_result.json` 文件，包含以下信息：

- 每个练习的评测结果（通过/失败）
- 总练习数量
- 通过的练习数量
- 失败的练习数量
- 总评测耗时

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT