use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    AboutFeatures(Args),
}

#[derive(clap::Args)]
struct Args {
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,

    /// Output file path
    #[arg(short, long, default_value = "Cargo.features")]
    output: std::path::PathBuf,
}

fn main() -> Result<()> {
    let CargoCli::AboutFeatures(args) = CargoCli::parse();

    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(path) = &args.manifest_path {
        cmd.manifest_path(path);
    }

    let metadata = cmd.exec()?;

    let mut features_map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            let package = metadata
                .packages
                .iter()
                .find(|p| p.id == node.id)
                .expect("Package not found in metadata");

            let mut features = node.features.clone();
            features.sort();

            // Simple collision handling: append version if collision
            let key = if features_map.contains_key(&package.name) {
                format!("{}@{}", package.name, package.version)
            } else {
                package.name.clone()
            };

            features_map.insert(key, features);
        }
    }

    let toml_string = toml::to_string_pretty(&features_map)?;
    std::fs::write(&args.output, toml_string)?;

    Ok(())
}
