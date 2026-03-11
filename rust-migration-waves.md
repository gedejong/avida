# Rust Migration Waves

This roadmap keeps the codebase in mixed C/C++ + Rust mode while incrementally migrating modules behind stable C ABI boundaries, and records forward-looking cleanup in `documentation/Tech-Debt-Backlog.md`.

## Wave 1: Low-coupling utilities

- `cRunningStats` (Rust-only; legacy fallback removed)
- `cRunningAverage` (Rust-only; legacy fallback removed)
- `cDoubleSum` (Rust-only; legacy fallback removed)

Exit criteria:
- Build and test gates pass (`./build_avida -DAVD_UNIT_TESTS:BOOL=ON`, `./run_tests --mode=slave`, `./cbuild/work/unit-tests`).
- Rust crate quality gates pass (`cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` in `rust/avida-rust`).
- Rust coverage gate passes (`./scripts/ci_coverage_check.sh 75` in `rust/avida-rust`; CI `coverage-rust` lane), with initial scope focused on actively migrated FFI-centric modules.
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
- `cResourceCount` scheduling boundary policy lock (completed): explicit truncation/saturation/invalid-step semantics in Rust helper math with boundary parity fixtures (negative, exact-boundary, NaN/Inf, large-ratio saturation) in Rust+C++ unit tests
- `cResourceCount` spatial scheduling helper derivation (completed): Rust-backed deterministic spatial update-count delta derivation for `DoUpdates` (`m_spatial_update - m_last_updated`) with explicit saturation parity fixtures while keeping `DoSpatialUpdates`/`FlowAll`/`StateAll` in C++
- `cResourceCount` setup precalc table derivation (completed): Rust-backed additive helper for deterministic `Setup` recurrence table fill (`decay_precalc`/`inflow_precalc`) while preserving C++ table ownership and update-loop usage
- `cResourceCount` non-spatial step-application kernel (completed): Rust-backed deterministic helper for chunked + remainder recurrence application used by `DoNonSpatialUpdates`, preserving C++ ownership while centralizing update math parity
- `cSpatialResCount` source/sink scalar helper extraction (completed): Rust-backed additive helpers now compute source per-cell distribution and clamped sink/cell-outflow deltas used by `Source`/`Sink`/`CellOutflow`, while C++ retains grid traversal, index resolution, and state mutation ownership
- `cResourceHistory` deterministic entry helpers (completed): Rust-backed exact/non-exact update selection and bounds-safe value lookup used by `getEntryForUpdate`, `GetResourceCountForUpdate`, and `GetResourceLevelsForUpdate` while keeping file loading and state ownership in C++
- `cEventList` deterministic parse helpers (completed): Rust-backed trigger classification and timing tuple parsing (`begin`/`all`/`once`/`end` and numeric forms) routed through additive C ABI while preserving C++ event creation/state mutation ownership
- `Data::TimeSeriesRecorder` typed parse-policy parity hardening (completed): Rust typed getter coercion now matches legacy `Apto::StrAs` semantics (`bool` exact true aliases only; `int`/`double` C-style coercion including partial/hex/exponent forms) with shared Rust+C++ matrix fixtures and unchanged ABI surface
- `Data::Package` primitive formatting parity hardening (completed): Rust-backed `Wrap<bool/int/double>::StringValue` paths now have expanded boundary/threshold parity matrices (signed zero, denormals, exponent cutovers, integer limits, NaN/Inf) locked against legacy `Apto::AsStr` behavior
- `cBitArray` selective `bitvec` evaluation (completed, no-adopt): Added focused Criterion benchmark matrix for `shift`/`increment`/`count` workloads plus expanded Rust parity matrices across binary/unary ops and edge widths; benchmark deltas were mixed/marginal, so production internals remain on the current custom bit-field path with stronger decision evidence.
- Backtrace-enabled CI validation (completed): Added a dedicated CI smoke leg that explicitly sets `AVIDA_ENABLE_BACKTRACE=1` for configure/reconfigure + build coverage, and hardened vendored `backward-cpp` alias-target creation to avoid duplicate-target failures when backtrace mode is enabled.
- `cSpatialResCount` deterministic helper extraction (completed): Rust-backed additive helpers now normalize inflow/outflow spans and compute per-neighbor flow scalar math used by `CheckRanges` and `FlowMatter`, with C++ retaining traversal/state mutation ownership and parity guards in Rust+C++ unit tests.
- `Data::TimeSeriesRecorder` FFI guardrail cleanup (completed): Rust FFI entry points now consistently use shared handle access/output patterns from `common.rs` for typed/string getters, preserving ABI/coercion behavior while tightening null-handle/out-param stability checks.
- Provider/history deterministic readability hardening (completed): Refactored `provider_helpers` parsing internals to a structured parse result and clarified `resource_history_helpers` nearest-index iteration semantics, preserving C ABI/return policies with expanded duplicate-index parity coverage.
- Rust coverage scope expansion gate (completed): Coverage gating now writes a reusable summary artifact, enforces representative module presence checks to prevent accidental scope drift, and raises the CI line threshold from 80% to 82% with passing evidence.
- Build/configure robustness hardening (completed): default low-noise configure path with optional backtrace opt-in, CI reconfigure smoke check, and stable Linux static link-order preservation for `aptostatic` resolution
- Consistency fixture hardening (completed): cross-platform determinism stabilizations for `sex` and `shaded_green_beard_instructions` via fixture-local knob pinning, narrower intent-focused output assertions, and richer CI diagnostics artifacts
- Starter seam definition remains in `documentation/Wave5-cResourceCount-Starter-Seam.md` for follow-on expansion

Focus:
- Migrate only after FFI and release-process maturity from waves 1-4.
- Introduce migration slices that can be toggled independently in CI.
- Next candidate: extract deterministic `cSpatialResCount` rectangle-iteration index helpers (including wrapped element addressing) behind additive Rust helpers while preserving C++ traversal/state ownership.
