FROM rust:1-bookworm AS build
WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo test && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /workspace/target/release/rust-stakeholder /usr/local/bin/rust-stakeholder
ENTRYPOINT ["rust-stakeholder"]
CMD ["--list-values"]
