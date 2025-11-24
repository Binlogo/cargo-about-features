use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

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

#[derive(serde::Serialize)]
struct FeatureSet {
    enabled: Vec<String>,
    unused: Vec<String>,
    dangling: Vec<String>,
}

fn main() -> Result<()> {
    let CargoCli::AboutFeatures(args) = CargoCli::parse();

    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(path) = &args.manifest_path {
        cmd.manifest_path(path);
    }

    let metadata = cmd.exec()?;

    let mut features_map: BTreeMap<String, FeatureSet> = BTreeMap::new();

    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            let package = metadata
                .packages
                .iter()
                .find(|p| p.id == node.id)
                .expect("Package not found in metadata");

            // Only check for dangling features if the package is a workspace member
            let is_workspace_member = metadata.workspace_members.contains(&package.id);

            let mut enabled_features = node.features.clone();
            enabled_features.sort();

            let mut unused_features: Vec<String> = package
                .features
                .keys()
                .filter(|f| !enabled_features.contains(f))
                .cloned()
                .collect();
            unused_features.sort();

            let mut dangling_features = Vec::new();
            if is_workspace_member {
                let package_root = package.manifest_path.parent().unwrap().as_std_path();
                let used_in_code = scan_feature_usages(package_root)?;

                for (feature, deps) in &package.features {
                    // A feature is dangling if:
                    // 1. It has no dependencies (does not enable other features)
                    // 2. AND it is not used in the code
                    if deps.is_empty() && !used_in_code.contains(feature) {
                        dangling_features.push(feature.clone());
                    }
                }
                dangling_features.sort();
            }

            // Simple collision handling: append version if collision
            let key = if features_map.contains_key(&package.name) {
                format!("{}@{}", package.name, package.version)
            } else {
                package.name.clone()
            };

            features_map.insert(
                key,
                FeatureSet {
                    enabled: enabled_features,
                    unused: unused_features,
                    dangling: dangling_features,
                },
            );
        }
    }

    let toml_string = toml::to_string_pretty(&features_map)?;
    std::fs::write(&args.output, toml_string)?;

    Ok(())
}

fn scan_feature_usages(root: &Path) -> Result<HashSet<String>> {
    let mut used_features = HashSet::new();
    let re = Regex::new(r#"feature\s*=\s*"([^"]+)""#)?;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "rs")
                && !e.path().components().any(|c| c.as_os_str() == "target")
        })
    {
        let content = std::fs::read_to_string(entry.path())?;
        for cap in re.captures_iter(&content) {
            if let Some(m) = cap.get(1) {
                used_features.insert(m.as_str().to_string());
            }
        }
    }

    Ok(used_features)
}
