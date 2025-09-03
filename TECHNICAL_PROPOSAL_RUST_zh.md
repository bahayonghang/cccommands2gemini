# 技术方案：cccommands2gemini (Rust 实现)

## 1. 项目概述

**项目名称:** `cccommands2gemini`

**核心目标:** 开发一个高效、可靠的命令行工具，用于将 Claude Code 的自定义命令格式（存储在 `.md` 文件中）批量转换为 Gemini CLI 所支持的命令格式（存储在 `.toml` 文件中）。

**主要功能:**
- 递归扫描指定的输入目录，查找所有 Claude 命令文件 (`.md`)。
- 解析每个 `.md` 文件，提取命令描述和提示内容。
- 对提取的内容进行必要的格式转换（例如，参数变量的替换）。
- 将转换后的内容序列化为 TOML 格式。
- 在指定的输出目录中生成对应的 `.toml` 文件，文件名保持一致（仅扩展名不同）。

## 2. 技术选型

**编程语言:** **Rust**

- **理由:**
    - **高性能:** Rust 在文件 I/O 和字符串处理方面提供卓越的性能，非常适合此类文件批处理任务。
    - **高可靠性:** 其所有权和类型系统能在编译期消除大量的常见错误，确保工具的稳定性和健壮性。
    - **强大的生态:** Rust 的包管理器 Cargo 和社区库 (crates.io) 为开发高质量的 CLI 工具提供了强大的支持。

**核心依赖库 (Crates):**

- **`clap`** (v4, with `derive` feature)
    - **用途:** 用于解析命令行参数。我们将定义一个结构体来声明 `--input-dir` 和 `--output-dir` 等参数，`clap` 会自动生成解析逻辑和帮助信息。
- **`walkdir`**
    - **用途:** 用于高效地递归遍历输入目录，查找所有需要处理的 `.md` 文件。
- **`serde`** (with `derive` feature)
    - **用途:** Rust 数据结构和特定格式之间进行序列化/反序列化的标准框架。我们将定义一个 `GeminiCommand` 结构体，并使用 `serde` 的 `Serialize` trait 将其转换为 TOML 格式。
- **`toml`**
    - **用途:** 与 `serde` 配合，提供将 Rust 结构体序列化为 TOML 字符串的功能。
- **`anyhow`**
    - **用途:** 提供统一、简洁的错误处理机制。所有可能失败的函数都将返回 `anyhow::Result<T>`，方便错误传递和上下文附加，提升用户体验。
- **`home`**
    - **用途:** 用于获取用户的主目录路径（例如 `~`），以便在用户未提供输入/输出目录时，定位到默认的 `~/.claude/commands` 和 `~/.gemini/commands`。

## 3. 架构设计

本工具将作为单个可执行的命令行程序。

**数据结构:**

为了匹配目标 TOML 文件的格式，我们将在 Rust 中定义一个核心的 `struct`：

```rust
// src/main.rs
use serde::Serialize;

#[derive(Serialize)]
struct GeminiCommand {
    description: String,
    prompt: String,
}
```

**核心工作流程:**

1.  **CLI 参数解析:**
    - 程序启动时，`clap` 将解析用户传入的 `--input-dir` 和 `--output-dir` 参数。
    - **默认目录:** 如果用户未提供这些参数，程序将使用默认路径：
        - **输入:** `~/.claude/commands`
        - **输出:** `~/.gemini/commands`
    - 如果参数缺失或无效（例如，默认目录不存在），程序将向用户显示清晰的错误信息。

2.  **文件处理:**
    - 使用 `walkdir` 库遍历 `input-dir`。
    - 针对遍历到的每个条目，检查它是否是文件并且扩展名为 `.md`。

3.  **解析与转换 (单个文件):**
    - 对于每个 `.md` 文件，执行以下操作：
        a. 读取文件全部内容到一个字符串中。
        b. **解析规则:**
            - 将文件内容的第一个非空行作为 `description`。
            - 将文件的余下部分作为 `prompt`。
        c. **内容替换:**
            - 在 `prompt` 字符串中，将 Claude 格式的参数变量 `$ARGUMENTS` 替换为 Gemini 格式的 `{{args}}`。
        d. 创建一个 `GeminiCommand` 结构体实例并填充解析出的数据。

4.  **序列化与写入:**
    - 使用 `toml::to_string(&gemini_command)` 将 `GeminiCommand` 实例序列化成一个 TOML 格式的字符串。
    - 构建输出文件路径。例如，如果输入文件是 `/path/to/input/my-command.md`，输出路径将是 `/path/to/output/my-command.toml`。
    - 将生成的 TOML 字符串写入到新的输出文件中。

5.  **错误处理:**
    - 整个流程中的所有 I/O 操作（读/写文件）和解析操作都将被包裹在 `anyhow::Result` 中。
    - 如果在处理任何一个文件时发生错误（如文件不可读、无写入权限等），程序将打印出清晰的错误信息，并可以选择继续处理下一个文件或直接终止。

## 4. 实现步骤

1.  **项目初始化:**
    - `cargo init`
    - `cargo add clap --features derive`
    - `cargo add walkdir`
    - `cargo add serde --features derive`
    - `cargo add toml`
    - `cargo add anyhow`
    - `cargo add home`

2.  **CLI 骨架搭建:**
    - 在 `src/main.rs` 中，使用 `clap` 定义命令行参数的结构体。

3.  **主逻辑实现:**
    - 实现 `main` 函数，调用核心处理逻辑，并优雅地处理返回的任何错误。

4.  **文件转换逻辑实现:**
    - 创建一个独立的函数，该函数接受输入文件路径和输出文件路径，并完成单个文件的读取、解析、转换和写入操作。

5.  **测试 (可选但推荐):**
    - 在 `tests` 目录下为核心转换逻辑编写单元测试，以确保解析和字符串替换的正确性。
