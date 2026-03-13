# Rust FFI error and out-param policy

This note documents internal conventions for `avd_*` functions. It does not change any ABI surface.

1. Out-parameter writes go through `common::set_out`.
2. A null out pointer is an immediate failure path for functions that return success/failure.
3. On failure, output buffers remain untouched whenever possible.
4. Pointer/slice guards must run before conversion/parsing work.
5. Prefer `with_*` accessors from `common.rs` instead of local raw pointer dereference.
6. Keep C-visible return contracts stable:
   - status-returning APIs use `1` success / `0` failure.
   - numeric conversion helpers preserve legacy Apto coercion behavior.
