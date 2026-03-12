# Rust FFI ABI Baseline

`avd_symbols_baseline.txt` records the expected exported `avd_*` symbol set for
the Rust static library.

CI behavior:
- Additive symbols are allowed and reported as a note.
- Removed or renamed symbols fail CI (`ci_abi_guard.py`).

When intentionally adding new stable symbols:
1. Regenerate FFI analysis:
   - `python3 scripts/ci_ffi_analysis.py --output target/ffi-analysis`
2. Update baseline:
   - `cp target/ffi-analysis/avd_symbols.txt abi/avd_symbols_baseline.txt`
3. Run Rust gates and commit both code + baseline changes together.
