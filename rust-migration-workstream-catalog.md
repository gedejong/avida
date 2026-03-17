# Migration Workstream Catalog (Remaining C++)

This catalog breaks the full migration into concrete workstreams and slice queues.

Related documents:

- `rust-migration-remaining-cpp-static-analysis.md`
- `rust-migration-full-plan.md`

## Workstream Prioritization

Priority order is based on:

1. Remaining code and branch mass
2. Coupling impact
3. Deterministic seam availability
4. Existing test/fixture leverage

## Workstream A: Resource Runtime Completion (`source/main`) — COMPLETE

Objective: complete `cResourceCount`/`cSpatialResCount` policy extraction.

Status: All deterministic helper seams extracted (42 C++ call-sites, 35+ FFI exports). Resource runtime C++ is orchestration-only. Parity coverage locked. Archived.

## Workstream B: CPU Execution Core (`source/cpu`)

Objective: migrate high-complexity execution logic safely.

Slice queue:

1. instruction family dispatch classifiers
2. stack/register deterministic helpers
3. control-flow and mutation selection policies
4. hardware-variant shared-kernel extraction

Primary files:

- `source/cpu/cHardwareCPU.cc`
- `source/cpu/cHardwareExperimental.cc`
- `source/cpu/cHardwareBCR.cc`
- `source/cpu/cHardwareGP8.cc`

## Workstream C: Population + Environment Core (`source/main`) — ACTIVE PRIORITY

Objective: migrate high-mass runtime state transition logic. This workstream now holds the highest-value fresh targets.

Slice queue:

1. **`cEnvironment.cc` reaction/process type dispatch classifiers** — NEXT CANDIDATE, 0 avd_ calls, string→enum patterns (process type, entry type, bonus type), very seam-ready
2. **`cStats.cc` task-count filtering and resource-gradient classification** — 0 avd_ calls, pure aggregation/filtering patterns, high value
3. `cPopulation` opinion/group assignment and forager classification helpers — 7 avd_ calls, large remaining decision surface
4. `cTaskLib` remaining name-dispatch and span/fibonacci scoring — 39 avd_ calls, diminishing returns
5. `cEnvironment` deeper reaction/requisite configuration parsing
6. `cStats` spatial resource data formatting and germline classification

Primary files:

- `source/main/cEnvironment.cc` (2,095 LOC, **0 avd_ calls** — fresh)
- `source/main/cStats.cc` (5,272 LOC, **0 avd_ calls** — fresh)
- `source/main/cPopulation.cc` (9,475 LOC, 7 avd_ calls)
- `source/main/cTaskLib.cc` (4,255 LOC, 39 avd_ calls)
- `source/main/cPhenotype.cc`

## Workstream D: Analyze and Action Pipeline (`source/analyze`, `source/actions`) — NEAR SATURATION

Objective: move heavy decision/aggregation logic while keeping I/O glue stable.

Current status: Policy/validation extraction is near-complete for deterministic seams.
- `PopulationActions.cc`: 45 avd_ calls across 16 unique helpers — validation/dispatch/guard patterns largely extracted
- `cAnalyze.cc`: 26 avd_ calls across 8 unique helpers — token/output short-circuit patterns extracted
- `PrintActions.cc`: 18 avd_ calls across 2 unique helpers — instruction output routing extracted

Remaining slice queue (maintenance-mode):

1. Residual `cAnalyze` file-type header generation and batch genotype iteration helpers (if seam-ready patterns emerge)
2. Residual `PopulationActions` genome-loading error classification (requires error context — lower priority)
3. fixture expansion for output parity across platforms

Primary files:

- `source/analyze/cAnalyze.cc` (11,459 LOC, 26 avd_ calls)
- `source/actions/PopulationActions.cc` (6,448 LOC, 45 avd_ calls)
- `source/actions/PrintActions.cc` (5,868 LOC, 18 avd_ calls)

## Workstream E: Script/Viewer/Systematics Tail

Objective: complete remaining computation-heavy tails after core runtime stabilization.

Slice queue:

1. script AST/semantic deterministic walkers
2. viewer data projection/aggregation helpers
3. systematics deterministic grouping/metrics helpers

## Cross-Cutting Workstream F: ABI + Quality Infrastructure

Objective: keep migration safe at scale.

Ongoing tasks:

- Keep additive ABI-only growth policy
- Maintain symbol baseline and CI FFI analysis
- Expand microbenchmarks for newly migrated hot paths
- Track coverage floor and per-module migration parity status
- Maintain a profile-driven hotspot list (call-frequency ranked) to steer benchmark-first planning.

## Suggested Milestones

- **M1**: Resource runtime completion (Workstream A)
- **M2**: CPU core first tranche (Workstream B slices 1-2)
- **M3**: Main runtime core first tranche (Workstream C slices 1-2)
- **M4**: Analyze/action stabilization (Workstream D slices 1-2)
- **M5**: Remaining tails + retirement cleanup

## Updated Priority Order (Top 5 executable candidates, 2026-03-17)

1. **Workstream C.1: `cEnvironment.cc` reaction-process-type dispatch classifiers** — fresh target, 0 avd_ calls, highest seam-readiness score, lowest risk
2. **Workstream C.2: `cStats.cc` task-count filtering and resource-gradient classification** — fresh target, 0 avd_ calls, pure aggregation patterns
3. **Workstream B.1: `cHardwareCPU.cc` instruction precondition and thread-evolution helpers** — 2 avd_ calls in 11K LOC, massive value surface
4. **Workstream C.3: `cPopulation.cc` opinion/group assignment and forager classification** — 7 avd_ calls, large remaining decision surface
5. **Workstream C.4: `cTaskLib.cc` remaining name-dispatch chain and scoring patterns** — diminishing returns but clear patterns remain
