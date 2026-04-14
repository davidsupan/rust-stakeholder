# Rust Experimental Surface

- Live-provider concepts are modeled separately from the deterministic baseline.
- The guarded runtime path is wired behind `--experimental-provider` and remains opt-in rather than part of the deterministic default loop.
- Local validation evidence now covers unit tests, `--list-values`, `local-demo`, and orphan experimental flag fail-fast behavior.
- Experimental provider profiles currently cover `local-demo`, `openai-compatible`, `anthropic`, `openai-consumer`, and `claude-consumer`.
- Prompt assets, prompt versions, personalization profiles, cache metadata, and provenance records are resolved separately from the deterministic scheduler.
- Consumer-session imports require encrypted local persistence through `STAKEHOLDER_EXPERIMENTAL_STORE_KEY`; plain-text secret/session storage remains forbidden.
- API-backed providers require explicit credentials and remain outside deterministic CI.
- Experimental additions must not change seeded parity fixtures by default.
