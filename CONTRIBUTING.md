# Contributing to rust-stakeholder

## Rules
- Treat Rust as the source-of-truth baseline for downstream ports.
- Use Conventional Commits.
- Do not land silent behavioral changes; update `stakeholder-core` traceability and docs in the same tranche.
- Keep deterministic seeded behavior stable unless the change is explicitly documented as a baseline evolution.

## Local workflow
- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo test`
- `docker build -t rust-stakeholder .`
- `docker run --rm rust-stakeholder --list-values`

## Change discipline
- Generator-family additions must update docs, examples, and fixtures.
- Experimental provider work must stay clearly separated from deterministic parity paths.
- Prefer additive event-schema evolution over breaking changes.
