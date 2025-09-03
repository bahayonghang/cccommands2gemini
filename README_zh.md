# cccommands2gemini

一个简洁高效的命令行工具，使用 Rust 编写，用于将 [Claude Code](https://www.anthropic.com/product) 的自定义命令格式（`.md`）转换为 Gemini CLI 所使用的格式（`.toml`）。

## 功能特性

-   **批量转换:** 递归地查找并转换目录中的所有 `.md` 命令文件。
-   **智能默认值:** 如果未指定路径，会自动使用标准的命令目录（输入为 `~/.claude/commands/`，输出为 `~/.gemini/commands/`）。
-   **命令行界面:** 提供简单清晰的命令行参数。
-   **跨平台:** 基于 Rust 构建，可在 Windows、macOS 和 Linux 上运行。

## 安装与构建

1.  确保您已安装 [Rust 工具链](https://www.rust-lang.org/tools/install)。
2.  克隆本仓库。
3.  构建优化的发布版本可执行文件：
    ```bash
    cargo build --release
    ```
4.  生成的可执行文件位于 `target/release/cccommands2gemini`。

## 使用方法

您可以通过两种方式运行本工具：

**1. 使用默认目录**

如果您的命令文件位于标准位置，只需直接运行可执行文件：

```bash
./target/release/cccommands2gemini
```

-   **默认输入目录:** `~/.claude/commands/`
-   **默认输出目录:** `~/.gemini/commands/`

**2. 指定目录**

使用 `--input-dir` (`-i`) 和 `--output-dir` (`-o`) 参数来指定自定义路径：

```bash
./target/release/cccommands2gemini --input-dir /path/to/my-claude-cmds --output-dir /path/to/my-gemini-cmds
```

## 转换逻辑

该工具遵循一套简单的转换规则：

-   从输入目录中读取一个 `.md` 文件。
-   文件的**第一行**被视为输出 `.toml` 文件中的 `description`。
-   文件的**余下部分**被视为 `prompt`。
-   `prompt` 中的占位符 `$ARGUMENTS` 会被替换为 `{{args}}`。

例如，一个输入文件 `claude-cmd.md`:

```markdown
生成一个提交信息。

根据下面的 git diff，建议一个符合规范的提交信息：
$ARGUMENTS
```

将会被转换为一个输出文件 `claude-cmd.toml`:

```toml
description = "生成一个提交信息。"
prompt = """
根据下面的 git diff，建议一个符合规范的提交信息：
{{args}}
"""
```
