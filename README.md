# Learning Tools - 编程训练营本地评测工具

这是一个用于在本地对多种编程课程练习进行评测的工具，支持 Rust、C++ 等多种语言的课程，无需依赖 GitHub Actions。

## 功能特点

- 🚀 **多课程支持**: 支持 `learning-lm-rs`、`learning-cxx`、`rustlings` 等多种课程类型
- 📦 **Git 子模块管理**: 自动管理课程仓库作为 Git 子模块
- 🔍 **智能评测**: 根据课程类型自动选择合适的评测策略
- 📊 **详细统计**: 提供完整的评测结果和统计信息
- 💾 **结果保存**: 将评测结果保存为 JSON 文件，方便后续分析
- 🎨 **友好界面**: 彩色输出和进度条显示，提升用户体验
- ⚡ **高性能**: 基于 Rust 构建，确保高性能和内存安全

## 支持的课程类型

| 课程类型 | 语言 | 评测方式 | 描述 |
|---------|------|----------|------|
| `learning-lm-rs` | Rust | `cargo test` | 大语言模型相关的 Rust 练习 |
| `learning-cxx` | C++ | `xmake run summary` | C++ 编程练习 |
| `rustlings` | Rust | `rustc` 编译测试 | Rust 语言学习练习 |
| 其他 | Rust | `rustc` 编译测试 | 默认使用 Rustlings 评测方式 |

## 安装步骤

### 前置要求

1. **Rust 环境**（必需）
```bash
# Ubuntu/Debian
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source ~/.cargo/env

sudo apt install rustup
```

2. **Git**（必需）
```bash
# Ubuntu/Debian
sudo apt install git

# macOS
brew install git
```

3. **XMake**（仅 learning-cxx 课程需要）
```bash
# 安装 XMake
curl -fsSL https://xmake.io/shget.text | bash
```

### 安装项目

1. 克隆本仓库
```bash
git clone https://github.com/yourusername/learning-tools.git
cd learning-tools
```

2. 编译项目
```bash
cargo build --release
```

## 使用方法

### 1. 配置课程
请先将练习仓库 fork 到您自己的 GitHub 账户，然后使用您自己的仓库链接进行配置，例如您fork后的rustlings仓库链接为https://github.com/user/rustlings.git 使用 `learn` 命令来添加新的课程仓库：


```bash
# 配置 learning-lm-rs 课程
cargo xtask learn learning-lm-rs --submodule https://github.com/user/learning-lm-rs.git

# 配置 learning-cxx 课程
cargo xtask learn learning-cxx --submodule https://github.com/user/learning-cxx.git

# 配置 rustlings 课程
cargo xtask learn rustlings --submodule https://github.com/user/rustlings.git
```

### 2. 评测练习

#### 评测所有课程
```bash
# 评测 exercises 目录下的所有课程
cargo xtask eval

# 显示详细输出
cargo xtask eval --verbose
```

#### 评测指定课程
```bash
# 评测指定课程
cargo xtask eval --course learning-lm-rs

# 评测指定路径的课程
cargo xtask eval --path ./my-exercises --verbose
```



## 评测结果

评测完成后，工具会在项目根目录生成 `eval_result.json` 文件，包含以下信息：

```json
{
  "exercises": [
    {
      "name": "exercise01",
      "result": true
    }
  ],
  "statistics": {
    "total_exercations": 42,
    "total_succeeds": 40,
    "total_failures": 2,
    "total_time": 15
  }
}
```

### 字段说明

- `exercises`: 每个练习的详细结果
  - `name`: 练习名称
  - `result`: 是否通过（true/false）
- `statistics`: 统计信息
  - `total_exercations`: 总练习数
  - `total_succeeds`: 通过数量
  - `total_failures`: 失败数量
  - `total_time`: 总耗时（秒）

## 项目结构

```
learning-tools/
├── Cargo.toml              # 工作空间配置
├── README.md               # 项目说明
├── LICENSE                 # 许可证
├── .gitmodules            # Git 子模块配置
├── exercises/             # 课程练习目录（Git 子模块）
│   ├── learning-lm-rs/    # Rust 大语言模型课程
│   ├── learning-cxx/      # C++ 课程
│   └── rustlings/         # Rustlings 课程
├── xtask/                 # 主要工具实现
│   ├── src/
│   │   ├── main.rs        # 命令行入口
│   │   ├── eval.rs        # 评测逻辑
│   │   ├── learn.rs       # 课程配置
│   │   └── setup.rs       # 环境配置
│   └── Cargo.toml
├── environment/           # 环境配置模块
├── course/               # 课程管理模块
└── eval_result.json      # 评测结果文件
```



## 常见问题

### Q: 如何添加新的课程？
A: 使用 `cargo xtask learn --course <课程名> --submodule <仓库地址>` 命令。

### Q: 评测失败怎么办？
A: 使用 `--verbose` 参数查看详细输出，检查课程目录和依赖是否正确安装。

### Q: 支持哪些课程类型？
A: 目前支持 learning-lm-rs、learning-cxx、rustlings 等，可以通过扩展代码支持更多类型。

## 贡献指南

我们欢迎各种形式的贡献！

1. **报告问题**: 在 [Issues](https://github.com/yourusername/learning-tools/issues) 中报告 bug 或提出功能请求
2. **提交代码**: Fork 项目，创建分支，提交 Pull Request
3. **完善文档**: 改进 README、代码注释或添加示例
4. **添加课程支持**: 为新的编程课程添加评测支持

### 开发环境设置

```bash
# 克隆项目
git clone https://github.com/yourusername/learning-tools.git
cd learning-tools

# 安装依赖并编译
cargo build

# 运行测试
cargo test

# 运行工具
cargo xtask --help
```

## 许可证

本项目采用 [MIT 许可证](LICENSE)。