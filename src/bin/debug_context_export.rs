use anyhow::Result;
use mdbook::renderer::RenderContext;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();

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
