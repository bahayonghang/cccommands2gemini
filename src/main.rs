use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A tool to convert Claude Code custom commands (.md) to Gemini CLI format (.toml)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets the input directory to search for Claude commands.
    /// Defaults to ~/.claude/commands/
    #[arg(short, long)]
    input_dir: Option<PathBuf>,

    /// Sets the output directory to save Gemini commands.
    /// Defaults to ~/.gemini/commands/
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}

/// Represents the structure of a Gemini command in TOML format.
#[derive(Serialize)]
struct GeminiCommand {
    description: String,
    prompt: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine input directory
    let input_dir = cli.input_dir.or_else(|| {
        home::home_dir().map(|mut path| {
            path.push(".claude");
            path.push("commands");
            path
        })
    }).ok_or_else(|| anyhow::anyhow!("Could not determine input directory. Please specify with --input-dir or ensure ~/.claude/commands exists."))?;

    // Determine output directory
    let output_dir = cli.output_dir.or_else(|| {
        home::home_dir().map(|mut path| {
            path.push(".gemini");
            path.push("commands");
            path
        })
    }).ok_or_else(|| anyhow::anyhow!("Could not determine output directory. Please specify with --output-dir or ensure ~/.gemini/commands exists."))?;
    
    println!("Input directory: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

    // Ensure output directory exists
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory at {}", output_dir.display()))?;

    // Process files
    for entry in WalkDir::new(&input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let source_path = entry.path();
        match convert_file(source_path, &input_dir, &output_dir) {
            Ok(dest_path) => println!("Successfully converted {} to {}", source_path.display(), dest_path.display()),
            Err(e) => eprintln!("Failed to convert {}: {}", source_path.display(), e),
        }
    }

    Ok(())
}

/// Converts a single Claude command file to a Gemini command file.
fn convert_file(source_path: &Path, input_dir: &Path, output_dir: &Path) -> Result<PathBuf> {
    // 1. Read file content
    let content = fs::read_to_string(source_path)
        .with_context(|| format!("Failed to read file: {}", source_path.display()))?;

    // 2. Parse content
    let (description, prompt_body) = match content.split_once('\n') {
        Some((desc, prompt)) => (desc, prompt),
        None => (content.as_str(), ""),
    };
    let description = description.trim().to_string();
    let prompt_body = prompt_body.trim_start().to_string();

    // 3. Replace arguments
    let final_prompt = prompt_body.replace("$ARGUMENTS", "{{args}}");

    // 4. Create GeminiCommand struct
    let gemini_command = GeminiCommand {
        description,
        prompt: final_prompt,
    };

    // 5. Serialize to TOML
    let toml_content = toml::to_string(&gemini_command)
        .with_context(|| "Failed to serialize command to TOML format")?;

    // 6. Determine and write to output file
    let relative_path = source_path.strip_prefix(input_dir)?;
    let mut dest_path = output_dir.to_path_buf();
    dest_path.push(relative_path);
    dest_path.set_extension("toml");

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(&dest_path, toml_content)
        .with_context(|| format!("Failed to write to file: {}", dest_path.display()))?;

    Ok(dest_path)
}
