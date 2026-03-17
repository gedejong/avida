# Avida — Claude Code Context

This file is automatically loaded by Claude Code in every session for this project.

---

## C++ to Rust Migration — Slice Workflow

Use this workflow for any migration slice. Pair with the program-docs rules below.

### 1. Plan-first execution
- If asked to plan, produce a concrete slice plan with clear scope and validation gates.
- Do not edit attached plan files; implement exactly from the plan unless the user requests changes.
- Keep migration incremental: preserve C ABI boundaries and avoid ownership redesign in one slice.

### 2. Todo discipline
- Reuse existing todos from the plan; do not create duplicates.
- Set the first todo to `in_progress` before making edits.
- Keep exactly one todo `in_progress`; mark each `completed` as work finishes.

### 3. Implementation constraints
- Preserve behavior parity by default (no silent policy changes).
- Keep exported FFI signatures stable unless the slice explicitly changes API surface.
- Prefer helper extraction and deterministic pure-function seams before stateful rewrites.

### 4. Mandatory validation gates

Rust gates (run inside `rust/avida-rust`):
```
cargo test --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
./scripts/ci_coverage_check.sh 75
```

Project gates (run from repo root):
```
./build_avida -DAVD_UNIT_TESTS:BOOL=ON -DAPTO_UNIT_TESTS:BOOL=OFF
./cbuild/work/unit-tests
./run_tests --mode=slave
```

### 5. Documentation refresh
After gates pass, update:
- `documentation/Tech-Debt-Backlog.md` — completed item + next candidate
- `rust-migration-waves.md` — slice status and scope notes

If the slice changes long-range prioritization, complexity assumptions, or workstream ordering, also update:
- `rust-migration-remaining-cpp-static-analysis.md`
- `rust-migration-full-plan.md`
- `rust-migration-workstream-catalog.md`
- `rust-migration-execution-playbook.md`

### 6. Commit hygiene
- Commit only when the user requests.
- Stage intended files explicitly; avoid unrelated submodule pointer changes.
- Use concise commit messages focused on *why* the slice was done.

---

## Rust Migration Program Docs

When working at program scale, always read and maintain these documents:

| File | Purpose |
|------|---------|
| `rust-migration-remaining-cpp-static-analysis.md` | Static-analysis baseline |
| `rust-migration-full-plan.md` | Full roadmap |
| `rust-migration-workstream-catalog.md` | Workstream registry |
| `rust-migration-execution-playbook.md` | Mandatory slice gates + playbook |
| `rust-migration-waves.md` | Per-wave status and scope notes |
| `documentation/Tech-Debt-Backlog.md` | Active backlog |

### During planning
- Read the static-analysis baseline and current roadmap before proposing a new slice.
- Align each new slice to the highest-priority remaining workstream/hotspot.
- State where the slice fits in the full plan and what becomes the next candidate.

### During implementation
- Keep changes incremental and ABI-safe unless explicitly asked otherwise.
- After gates pass, update the roadmap/backlog and relevant program docs in the same change set.
- Reflect any reprioritization caused by new findings (risk, complexity, coupling, or test gaps).

### Documentation quality bar
- Keep docs factual and metric-backed; avoid vague progress notes.
- Preserve consistency across all migration docs (no conflicting next-candidate entries).
- If static-analysis assumptions change materially, refresh the baseline document before closing the slice.

---

## Current Migration State (as of 2026-03-17)

**71 unique `avd_*` FFI exports, 179 C++ call-sites across 7 actively-routed files.**

- **Waves 1–4**: Completed (archived in `documentation/archive/rust-migration-waves-completed.md`).
- **Wave 5**: COMPLETE — `cResourceCount`/`cSpatialResCount` fully extracted (42 call-sites, 35+ FFI exports). Resource runtime is orchestration-only.
- **Wave 6**: CPU pilot done (`cHardwareCPU` dispatch family/opcode — 2 calls). Massive remaining surface (11K LOC).
- **Wave 7**: Active — `cTaskLib` scoring/registration near-complete (39 calls). `cPopulation` deme routing done (7 calls). **`cEnvironment.cc` and `cStats.cc` are untouched fresh targets (0 calls each).**
- **Wave 8**: Near saturation — `PopulationActions` (45 calls), `cAnalyze` (26 calls), `PrintActions` (18 calls).
- **Waves 6–8 execute in parallel** by seam-readiness, not sequentially.

**Next candidate**: `cEnvironment.cc` reaction-process-type dispatch classifiers (string→enum patterns, 0 avd_ calls, very seam-ready, lowest risk).
