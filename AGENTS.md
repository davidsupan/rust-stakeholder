# rust-stakeholder AGENTS

1. Rust remains the canonical source of truth and stays on `main`.
2. Do not widen the Rust baseline in a follower wave unless the change is explicitly requested and documented in stakeholder-core.
3. Commands:
   - `cargo build`
   - `cargo test`
4. Preserve deterministic normalized JSON semantics and the traceability anchors used by follower repos.
5. Any canonical behavior change must be reflected in stakeholder-core docs, fixtures, and downstream traceability.
6. Keep governance and status sync additive; avoid incidental churn.
