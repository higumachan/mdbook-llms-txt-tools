use anyhow::Result;
use clap::{Parser, Subcommand};
use mdbook::renderer::RenderContext;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mdbook-debug-context-export", about = "A mdbook backend for exporting RenderContext as JSON")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Supports {
        #[arg(required = true)]
        renderer: String,
    },
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    if let Some(Commands::Supports { renderer }) = cli.command {
        if renderer == "debug-context-export" {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin)?;

    // Create output directory
    let output_path = PathBuf::from(&ctx.destination).join("context.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Output as JSON
    let json = serde_json::to_string_pretty(&ctx)?;
    fs::write(&output_path, json)?;

    println!(
        "Successfully exported RenderContext to: {}",
        output_path.display()
    );
    Ok(())
}
