# Rust Migration Waves

This roadmap keeps the codebase in mixed C/C++ + Rust mode while incrementally migrating modules behind stable C ABI boundaries, and records forward-looking cleanup in `documentation/Tech-Debt-Backlog.md`.

## Wave 1: Low-coupling utilities

- Completed Wave 1 module entries archived in `documentation/archive/rust-migration-waves-completed.md`.

Exit criteria:
- Build and test gates pass (`./build_avida -DAVD_UNIT_TESTS:BOOL=ON`, `./run_tests --mode=slave`, `./cbuild/work/unit-tests`).
- Rust crate quality gates pass (`cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` in `rust/avida-rust`).
- Rust coverage gate passes (`./scripts/ci_coverage_check.sh 75` in `rust/avida-rust`; CI `coverage-rust` lane), with initial scope focused on actively migrated FFI-centric modules.
- FFI invariants are documented and followed: opaque handles, explicit create/free, null-safe no-op reads/writes, and no cross-language frees.

## Wave 2: Deterministic weighted selectors

- Completed Wave 2 module entries archived in `documentation/archive/rust-migration-waves-completed.md`.

Focus:
- Preserve exact floating-point behavior for weighted lookups.
- Keep C++ wrappers for existing Apto container interactions.

## Wave 3: Data utilities with side effects isolated

- Completed Wave 3 module entries archived in `documentation/archive/rust-migration-waves-completed.md`.

Focus:
- Replace direct file side effects with explicit API hooks where possible.
- Add edge-case tests for bit operations and histogram boundaries.

## Wave 4: Medium-coupling analysis/data modules

- Selected modules in `source/analyze`
- Selected modules in `source/data`
- `Data::Package` (Rust-backed deterministic `ArrayPackage` conversion/format, `Wrap<Apto::String>` parse helpers, and primitive `Wrap<bool/int/double>::StringValue` formatting via C ABI)
- `Data::TimeSeriesRecorder` (Rust-authoritative state for parse/serialize + append + typed reads via C ABI; C++ mirror state removed)
- `Data::Provider` deterministic helper dispatch/parse path (Rust-backed C ABI helpers; classify/split dispatch centralized through `avd_provider_classify_id` with expanded edge-shape matrix coverage and consistency fixture lock)
- `Data::Manager` deterministic argumented-ID classify/split path (Rust-backed C ABI helpers reused across active/describe/attach/get/register flows, including provider registration/activation mapping and edge-shape parity fixtures)

Focus:
- Define narrow ABI seams around pure computation and deterministic transforms.
- Avoid crossing ownership boundaries with complex object graphs.
- Keep `rust/avida-rust/src/lib.rs` as a thin hub with per-domain module ownership.
- Evaluate mature external crates first for each slice; keep custom code only when parity/ABI constraints require it.
- Reuse shared Rust helper parsers across `source/data` modules to eliminate duplicated C++ bracket parsing logic.
- Consolidate duplicated FFI CString/output-pointer handling through shared `common.rs` helpers (`provider_helpers`, `time_series_recorder`, `package`) to reduce repeated unsafe patterns.

## Wave 5: Runtime and execution core

- Selected modules in `source/main`
- Selected modules in `source/cpu`
- `cResourceCount` deterministic helper paths (Rust-backed FFI lookup, inflow/decay precalc helper math, and update-step scheduling helper math used by `GetResourceCountID`/`GetResourceByName`/`SetInflow`/`SetDecay`/`Update`/`DoUpdates` step derivation)
- `cSpatialResCount::FlowAll` per-neighbor transfer accumulation (completed): Rust-backed additive pair-delta helper now computes neighbor transfer deltas used by `FlowAll` while C++ retains grid traversal, neighbor lookup, and state mutation ordering.
- `cSpatialResCount` aggregate-update helpers (completed): Rust-backed additive `StateAll` fold and `SumAll` reduction helpers now compute deterministic per-cell fold/reduction math while C++ retains container ownership and call sequencing.
- `cSpatialResCount` bulk-rate/reset helpers (completed): Rust-backed additive helpers now compute deterministic per-cell `RateAll` delta progression and `ResetResourceCounts` amount reset math while C++ retains traversal order, ownership, and call sequencing.
- `cSpatialResCount` SetCellList cell-init helpers (completed): Rust-backed additive helper now computes deterministic per-cell initialization fold used by `SetCellList` while C++ retains cell-list traversal, bounds policy, and state ownership/order.
- Completed Wave 5 slice history archived in `documentation/archive/rust-migration-waves-completed.md`.
- Starter seam definition remains in `documentation/Wave5-cResourceCount-Starter-Seam.md` for follow-on expansion

Focus:
- Migrate only after FFI and release-process maturity from waves 1-4.
- Introduce migration slices that can be toggled independently in CI.
- Next candidate: extract deterministic `cResourceCount` precalc table-fill recurrence helpers (`SetInflow`/`SetDecay` table population loops) behind additive Rust helpers while preserving C++ ownership and sequencing.
