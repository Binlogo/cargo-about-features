//! # cargo-about-features Examples
//!
//! This module contains examples demonstrating how to use the `cargo-about-features` library.
//!
//! ## Basic Usage
//!
//! ```ignore
//! use cargo_about_features::{analyze_features, generate_toml_output};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Analyze features in the current directory
//!     let features_map = analyze_features(None, None)?;
//!
//!     // Generate TOML output
//!     let output = generate_toml_output(&features_map)?;
//!
//!     // Print or write to file
//!     println!("{}", output);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Target-Specific Analysis
//!
//! Analyze features for a specific target platform (e.g., wasm32):
//!
//! ```ignore
//! use cargo_about_features::{analyze_features, generate_toml_output};
//!
//! fn analyze_wasm_features() -> Result<(), Box<dyn std::error::Error>> {
//!     let features_map = analyze_features(
//!         None,                          // Use current directory
//!         Some("wasm32-unknown-unknown") // Filter for WASM target
//!     )?;
//!
//!     let output = generate_toml_output(&features_map)?;
//!     std::fs::write("wasm-features.toml", output)?;
//!
//!     Ok(())
//! }
//! ```

use anyhow::Result;
use cargo_about_features::{analyze_features, generate_toml_output};

/// Example: Basic feature analysis
///
/// Run with: `cargo run --example basic`
fn basic_example() -> Result<()> {
    println!("=== Basic Feature Analysis ===\n");

    // Analyze features in current project
    let features_map = analyze_features(None, None)?;

    // Generate readable output
    let output = generate_toml_output(&features_map)?;

    // Print summary
    println!("Total packages analyzed: {}", features_map.len());

    // Count enabled features
    let total_enabled: usize = features_map.values().map(|fs| fs.enabled.len()).sum();

    println!("Total enabled features: {}", total_enabled);

    // Print first 5 packages as example
    println!("\nFirst 5 packages:");
    for (i, (name, fs)) in features_map.iter().take(5).enumerate() {
        println!(
            "  {}. {}: {} enabled features",
            i + 1,
            name,
            fs.enabled.len()
        );
    }

    Ok(())
}

/// Example: Target-specific analysis
///
/// Run with: `cargo run --example target`
fn target_example() -> Result<()> {
    println!("=== Target-Specific Feature Analysis ===\n");

    // Determine current target
    let target = std::env::consts::OS;
    let target_triple = match target {
        "macos" => "aarch64-apple-darwin",
        "linux" => "x86_64-unknown-linux-gnu",
        "windows" => "x86_64-pc-windows-msvc",
        _ => {
            println!("Unknown platform, skipping target-specific example");
            return Ok(());
        }
    };

    println!("Current target: {}\n", target_triple);

    // Analyze with target filter
    let features_map = analyze_features(None, Some(target_triple))?;

    println!(
        "Packages visible to {}: {}",
        target_triple,
        features_map.len()
    );

    Ok(())
}

/// Example: CLI integration
///
/// This shows how the library integrates with cargo subcommand pattern.
fn cli_integration_example() {
    // This is the pattern used in src/main.rs
    //
    // #[derive(clap::Parser)]
    // #[command(name = "cargo", bin_name = "cargo")]
    // enum CargoCli {
    //     AboutFeatures(Args),
    // }
    //
    // #[derive(clap::Args)]
    // struct Args {
    //     #[arg(short, long)]
    //     target: Option<String>,
    //
    //     #[arg(short, long, default_value = "Cargo.features")]
    //     output: PathBuf,
    // }
    //
    // fn main() -> Result<()> {
    //     let CargoCli::AboutFeatures(args) = CargoCli::parse();
    //
    //     let features_map = analyze_features(None, args.target.as_deref())?;
    //     let output = generate_toml_output(&features_map)?;
    //     std::fs::write(&args.output, output)?;
    //
    //     Ok(())
    // }

    println!("See src/main.rs for CLI implementation");
}

fn main() -> Result<()> {
    // Run basic example
    basic_example()?;

    println!("\n---\n");

    // Run target example
    target_example()?;

    println!("\n---\n");

    // Show CLI pattern
    cli_integration_example();

    Ok(())
}
