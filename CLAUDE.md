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

## Current Migration State (as of 2026-03-17, post debt-tranche)

**268 unique `avd_*` FFI exports, ~260 C++ call-sites across 18 actively-routed files. 193 Rust tests.**

| File | avd_ calls | Status |
|---|---:|---|
| `PopulationActions.cc` | 45 | Near saturation |
| `cResourceCount.cc` | 41 | Complete |
| `cTaskLib.cc` | 39 | Near saturation |
| `cAnalyze.cc` | 26 | Near saturation |
| `cPopulation.cc` | 21 | Active — depth extraction ongoing |
| `PrintActions.cc` | 18 | Near saturation |
| `cHardwareCPU.cc` | 16 | Active — depth extraction ongoing |
| `cStats.cc` | 9 | Covered |
| `cHardwareExperimental.cc` | 9 | Covered |
| `cHardwareBCR.cc` | 7 | Covered |
| `cHardwareGP8.cc` | 7 | Covered |
| `cGradientCount.cc` | 7 | Covered |
| `cEnvironment.cc` | 6 | Covered |
| `cOrgSensor.cc` | 4 | Covered |
| `cResource.cc` | 3 | Covered |
| `cHardwareTransSMT.cc` | 1 | Pilot |

- **Waves 1–5**: Complete.
- **Waves 6–8**: Executing in parallel by seam-readiness.
- **ABI baseline**: Refreshed to 268 symbols (2026-03-17).
- **Remaining untouched files**: `cPopulationInterface.cc` (2.4K), `cOrganism.cc` (1.7K), `cDeme.cc` (1.7K), `cLandscape.cc` (1K).

**Next candidates**: Deeper `cHardwareCPU.cc` / `cPopulation.cc` extraction, or `cPopulationInterface.cc` as a fresh target.
