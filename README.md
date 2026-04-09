# rust-stakeholder

Satirical CLI output generator, now serving as the 2026+ source baseline for the multi-language `stakeholder` rewrite program.

## Status
- Rust is the source-of-truth implementation.
- The runtime is now scheduler-driven and deterministic under seed.
- The baseline includes modern generator families for agents, AI operations, blockchain, healthcare, EV charging, protocol ecosystems, multilingual security flavor packs, and quantum-themed infrastructure work.
- Experimental live-provider concepts exist in the design, but deterministic parity behavior remains the default path.

## Command contract
- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo build`
- `cargo test`
- `docker build -t rust-stakeholder .`
- `docker run --rm rust-stakeholder --list-values`

## Example usage
```bash
cargo run -- --dev-type security --jargon high --complexity extreme --alerts --seed 42
cargo run -- --dev-type blockchain --framework "rollup-mcp-gateway" --output-format json --seed 7
cargo run -- --project "hospital-ocpp-quantum-control" --jargon extreme --team --trace --seed 11
```

## What changed in the 2026+ baseline
- classic families were modernized instead of left as legacy parody strings
- activity planning now uses typed family selection instead of a fixed loop
- keyword routing brings in healthcare, charging, protocol, and quantum families where relevant
- security runs can layer multilingual language packs and persona overlays
- JSON output is deterministic and snapshot-friendly when a seed is supplied

## Docs
- [Tooling](docs/tooling.md)
- [Docker](docs/docker.md)
- [Edge cases](docs/edge-cases.md)
- [Language specialties](docs/language-specialties.md)
- [Example outputs](docs/example-outputs.md)
- [Experimental](docs/experimental.md)
- [Traceability](docs/traceability/README.md)

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md). Use Conventional Commits and keep Rust behavior traceable because downstream repos depend on this baseline.
