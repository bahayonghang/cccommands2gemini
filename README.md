# cccommands2gemini

A simple and efficient command-line tool, written in Rust, to convert [Claude Code](https://www.anthropic.com/product)'s custom command format (`.md`) to the format used by the Gemini CLI (`.toml`).

## Features

-   **Batch Conversion:** Recursively finds and converts all `.md` command files in a directory.
-   **Smart Defaults:** Automatically uses standard command directories (`~/.claude/commands/` for input, `~/.gemini/commands/` for output) if not specified.
-   **CLI Interface:** Simple and clear command-line arguments.
-   **Cross-Platform:** Built with Rust, runs on Windows, macOS, and Linux.

## Installation & Building

1.  Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.
2.  Clone this repository.
3.  Build the optimized release executable:
    ```bash
    cargo build --release
    ```
4.  The executable will be available at `target/release/cccommands2gemini`.

## Usage

You can run the tool in two ways:

**1. Using Default Directories**

If your commands are in the standard locations, simply run the executable:

```bash
./target/release/cccommands2gemini
```

-   **Default Input Directory:** `~/.claude/commands/`
-   **Default Output Directory:** `~/.gemini/commands/`

**2. Specifying Directories**

Use the `--input-dir` (`-i`) and `--output-dir` (`-o`) flags to specify custom paths:

```bash
./target/release/cccommands2gemini --input-dir /path/to/my-claude-cmds --output-dir /path/to/my-gemini-cmds
```

## Conversion Logic

The tool follows a simple set of rules for conversion:

-   It reads a `.md` file from the input directory.
-   The **first line** of the file is treated as the `description` in the output `.toml` file.
-   The **rest of the file** is treated as the `prompt`.
-   The placeholder `$ARGUMENTS` in the prompt is replaced with `{{args}}`.

For example, an input file `claude-cmd.md`:

```markdown
Generate a commit message.

Suggest a conventional commit message based on the following git diff:
$ARGUMENTS
```

Will be converted to an output file `claude-cmd.toml`:

```toml
description = "Generate a commit message."
prompt = """
Suggest a conventional commit message based on the following git diff:
{{args}}
"""
```
