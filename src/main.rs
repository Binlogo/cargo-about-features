use anyhow::Result;
use cargo_about_features::{analyze_features, generate_toml_output};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    AboutFeatures(Args),
}

#[derive(clap::Args)]
struct Args {
    #[arg(long)]
    manifest_path: Option<PathBuf>,

    /// Target triple (e.g., x86_64-unknown-linux-gnu, wasm32-unknown-unknown)
    #[arg(short, long)]
    target: Option<String>,

    /// Output file path
    #[arg(short, long, default_value = "Cargo.features")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let CargoCli::AboutFeatures(args) = CargoCli::parse();

    // Analyze features
    let features_map = analyze_features(args.manifest_path, args.target.as_deref())?;

    // Generate TOML output with comments
    let output_content = generate_toml_output(&features_map)?;

    // Write output file
    std::fs::write(&args.output, output_content)?;

    Ok(())
}
