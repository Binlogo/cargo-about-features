# cargo-about-features

A Cargo subcommand that generates a lock file (`Cargo.features`) listing the actually enabled features for all dependencies in your project.

## Installation

```bash
cargo install cargo-about-features
```

## Usage

Run the subcommand in your Cargo project directory:

```bash
cargo about-features
```

This will generate a `Cargo.features` file in the same directory.

## Output Format

The output file `Cargo.features` is a TOML file where keys are package names (potentially with versions if collisions occur) and values are lists of enabled features.

Example `Cargo.features`:

```toml
serde = ["default", "derive", "std"]
tokio = ["default", "fs", "io-util", "net", "rt", "rt-multi-thread", "sync", "time"]
# ...
```

## License

[MIT](LICENSE)
