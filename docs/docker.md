# Rust Docker

## Build and test
- `docker build -t rust-stakeholder .`
- `docker run --rm rust-stakeholder --list-values`

## Rationale
- The image compiles and tests the Rust baseline before packaging the runtime binary.
- Docker is the reproducible Linux gate; host and CI matrices still cover native OS behavior.
