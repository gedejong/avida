# Rust Migration Waves

This roadmap keeps the codebase in mixed C/C++ + Rust mode while incrementally migrating modules behind stable C ABI boundaries, and records forward-looking cleanup in `documentation/Tech-Debt-Backlog.md`.

## Wave 1: Low-coupling utilities

- `cRunningStats` (Rust-only; legacy fallback removed)
- `cRunningAverage` (Rust-only; legacy fallback removed)
- `cDoubleSum` (Rust-only; legacy fallback removed)

Exit criteria:
- Build and test gates pass (`./build_avida -DAVD_UNIT_TESTS:BOOL=ON`, `./run_tests --mode=slave`, `./cbuild/work/unit-tests`).
- Rust crate quality gates pass (`cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` in `rust/avida-rust`).
- FFI invariants are documented and followed: opaque handles, explicit create/free, null-safe no-op reads/writes, and no cross-language frees.

## Wave 2: Deterministic weighted selectors

- `cWeightedIndex` (Rust-only; legacy fallback removed)
- `cOrderedWeightedIndex` (Rust-only; legacy fallback removed)

Focus:
- Preserve exact floating-point behavior for weighted lookups.
- Keep C++ wrappers for existing Apto container interactions.

## Wave 3: Data utilities with side effects isolated

- `cHistogram` (Rust-only; legacy fallback removed)
- `cBitArray` (completed strict slice, Rust-only; legacy fallback removed)

Focus:
- Replace direct file side effects with explicit API hooks where possible.
- Add edge-case tests for bit operations and histogram boundaries.

## Wave 4: Medium-coupling analysis/data modules

- Selected modules in `source/analyze`
- Selected modules in `source/data`
- `Data::Package` (Rust-backed deterministic `ArrayPackage` conversion/format path via C ABI)
- `Data::TimeSeriesRecorder` (Rust-backed deterministic parse/serialize + append path via C ABI)
- `Data::Provider` deterministic helper dispatch/parse path (Rust-backed C ABI helpers; C++ API unchanged)
- `Data::Manager` deterministic argumented-ID split path (Rust-backed C ABI helpers for parse/classify reuse)

Focus:
- Define narrow ABI seams around pure computation and deterministic transforms.
- Avoid crossing ownership boundaries with complex object graphs.
- Keep `rust/avida-rust/src/lib.rs` as a thin hub with per-domain module ownership.
- Evaluate mature external crates first for each slice; keep custom code only when parity/ABI constraints require it.
- Reuse shared Rust helper parsers across `source/data` modules to eliminate duplicated C++ bracket parsing logic.

## Wave 5: Runtime and execution core

- Selected modules in `source/main`
- Selected modules in `source/cpu`

Focus:
- Migrate only after FFI and release-process maturity from waves 1-4.
- Introduce migration slices that can be toggled independently in CI.
