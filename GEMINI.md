# GEMINI Project Context

## Project Overview

This project, `cccommands2gemini`, is a command-line utility designed to convert custom command definitions from the Claude Code format to the Gemini CLI and Qwen Code formats, as described in the `README.md`.

The original technical proposal and example files that would contain specific architectural choices and data formats are currently missing from the directory. Based on the project's name and common practices for building such command-line tools, it is presumed to be a Rust-based application.

The core functionality will involve:
1.  Reading command definition files from a source directory.
2.  Parsing the content to extract relevant data (like descriptions and prompts).
3.  Writing that data into a new file with the target format in an output directory.

## Building and Running

The Rust project has not been initialized yet (no `Cargo.toml` is present). The following are general steps that would be required to set up and run the project.

**TODO:** These steps are based on a standard Rust workflow and need to be confirmed once the project is initialized.

**1. Project Setup (run once):**

```bash
# Initialize a new Rust project in the current directory
cargo init

# TODO: Add necessary dependencies once they are determined.
# For a tool like this, they would likely include:
# - `clap` (for command-line argument parsing)
# - `walkdir` (for directory traversal)
# - `serde` and `toml` (for data serialization into the TOML format)
# - `anyhow` (for error handling)
#
# Example command:
# cargo add clap --features derive && cargo add walkdir && cargo add serde --features derive && cargo add toml && cargo add anyhow
```

**2. Building the Tool:**

```bash
# Build the project for a release version
cargo build --release
```

**3. Running the Tool:**

The executable would be located at `target/release/cccommands2gemini`.

```bash
# TODO: The exact command-line arguments need to be defined.
# Example command:
./target/release/cccommands2gemini --input-dir /path/to/claude/commands --output-dir /path/to/gemini/commands
```

## Development Conventions

**TODO:** This section needs to be defined.

This section should describe the expected format of the input and output files, as well as any coding style or error handling conventions. The original example files (`example_claude_command.md`, `example_gemini_command.toml`) would be needed to define this.
