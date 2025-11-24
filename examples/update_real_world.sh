#!/bin/bash
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REAL_WORLD_DIR="$SCRIPT_DIR/real-world"
CARGO_ABOUT_FEATURES="$SCRIPT_DIR/../target/debug/cargo-about-features"

# Ensure cargo-about-features is built
pushd "$SCRIPT_DIR/.."
cargo build
popd

mkdir -p "$REAL_WORLD_DIR"

# Function to process a repository
process_repo() {
    local repo_url=$1
    local name=$(basename "$repo_url" .git)
    local repo_dir="$REAL_WORLD_DIR/$name"
    local output_dir="$SCRIPT_DIR/$name"
    mkdir -p "$output_dir"

    echo "Processing $name..."

    if [ -d "$repo_dir" ]; then
        echo "Updating $name..."
        pushd "$repo_dir"
        git pull
        popd
    else
        echo "Cloning $name..."
        git clone "$repo_url" "$repo_dir"
    fi

    echo "Generating Cargo.features for $name..."
    "$CARGO_ABOUT_FEATURES" about-features --manifest-path "$repo_dir/Cargo.toml" --output "$output_dir/Cargo.features"
    echo "Done."
}

# List of repositories to process
process_repo "https://github.com/zefchain/serde-reflection"
