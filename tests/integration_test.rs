use anyhow::Result;
use cargo_about_features::{analyze_features, generate_toml_output};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_simple_project_analysis() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("test_project");
    fs::create_dir(&project_dir)?;

    // Create test Cargo.toml
    let cargo_toml = r#"
[package]
name = "test_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
"#;

    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    // Create src directory and lib.rs
    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir)?;
    fs::write(src_dir.join("lib.rs"), "// test")?;

    // Analyze features
    let features_map = analyze_features(Some(project_dir.join("Cargo.toml")), None)?;
    let output = generate_toml_output(&features_map)?;

    // Use insta for snapshot testing
    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!("simple_project_analysis", output);
    });

    Ok(())
}

#[test]
fn test_workspace_project_analysis() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let workspace_dir = temp_dir.path().join("workspace_test");
    fs::create_dir(&workspace_dir)?;

    // 创建工作区 Cargo.toml
    let workspace_cargo_toml = r#"
[workspace]
members = ["crate-a", "crate-b"]
resolver = "2"
"#;
    fs::write(workspace_dir.join("Cargo.toml"), workspace_cargo_toml)?;

    // Create crate-a
    let crate_a_dir = workspace_dir.join("crate-a");
    fs::create_dir(&crate_a_dir)?;
    let crate_a_cargo_toml = r#"
[package]
name = "crate-a"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive", "std"] }
"#;
    fs::write(crate_a_dir.join("Cargo.toml"), crate_a_cargo_toml)?;

    let crate_a_src = crate_a_dir.join("src");
    fs::create_dir(&crate_a_src)?;
    fs::write(
        crate_a_src.join("lib.rs"),
        r#"
#[cfg(feature = "test-feature")]
fn test_function() {
    // This function uses a test feature
}
"#,
    )?;

    // Create crate-b
    let crate_b_dir = workspace_dir.join("crate-b");
    fs::create_dir(&crate_b_dir)?;
    let crate_b_cargo_toml = r#"
[package]
name = "crate-b"
version = "0.1.0"
edition = "2021"

[dependencies]
crate-a = { path = "../crate-a" }
toml = { version = "0.8", features = ["parse"] }
"#;
    fs::write(crate_b_dir.join("Cargo.toml"), crate_b_cargo_toml)?;

    let crate_b_src = crate_b_dir.join("src");
    fs::create_dir(&crate_b_src)?;
    fs::write(crate_b_src.join("lib.rs"), "// crate-b")?;

    // Analyze features
    let features_map = analyze_features(Some(workspace_dir.join("Cargo.toml")), None)?;
    let output = generate_toml_output(&features_map)?;

    // Use insta for snapshot testing
    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!("workspace_project_analysis", output);
    });

    Ok(())
}

#[test]
fn test_feature_collision_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("collision_test");
    fs::create_dir(&project_dir)?;

    // Create test project that introduces different versions of the same package
    let cargo_toml = r#"
[package]
name = "collision_test"
version = "0.1.0"
edition = "2021"

[dependencies]
# These dependencies will indirectly introduce different versions of the same package
regex = "1.0"
regex-automata = "0.3"
"#;

    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir)?;
    fs::write(src_dir.join("lib.rs"), "// test")?;

    // Analyze features
    let features_map = analyze_features(Some(project_dir.join("Cargo.toml")), None)?;
    let output = generate_toml_output(&features_map)?;

    // Verify output contains version information
    assert!(
        output.contains('@'),
        "Output should contain version numbers for collision handling"
    );

    // Use insta for snapshot testing
    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!("feature_collision_handling", output);
    });

    Ok(())
}
