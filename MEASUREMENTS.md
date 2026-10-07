# Measurements

Stage 4, recorded by the agent before building. `B2 Require step-by-step justification`.

The starting state, measured rather than recalled, so later changes can be compared with it.

| What | Command | Result | When |
| --- | --- | --- | --- |
| Toolchain | `rustc --version; wasm-bindgen --version` | rustc 1.95.0 (59807616e 2026-04-14); wasm-bindgen 0.2.127 | 2026-10-07 13:31 |
| Core tests | `cargo test --workspace` | 18 passed, 0 failed: 7 unit tests in `tool_core` (`api.rs` 2, `page.rs` 5), 11 boundary tests in `tests/boundary.rs`; shim 0 | 2026-10-07 13:31 |
| Full check | `./scripts/check.sh` | `BUILDER CHECK PASSED`; browser check passed in chromium, firefox, webkit; 42.8 s wall time | 2026-10-07 13:32 |
| Template commit | `git log --oneline` | `87568c3 Start from the Builder template` | 2026-10-07 13:31 |
