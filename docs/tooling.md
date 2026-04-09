# Rust Tooling

## Commands
- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo build`
- `cargo test`
- `docker build -t rust-stakeholder .`
- `docker run --rm rust-stakeholder --list-values`

## Extended local checks
- `cargo nextest run`
- `cargo audit`
- `cargo deny check`
- `cargo udeps`

## Notes
- The Docker path is the reproducible Linux baseline.
- Native CI should still cover macOS and Windows semantics.
