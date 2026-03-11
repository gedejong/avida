#!/usr/bin/env bash
set -euo pipefail

THRESHOLD="${1:-75}"
IGNORE_REGEX="${2:-.*\/(bit_array|double_sum|histogram|ordered_weighted_index|running_average|running_stats|weighted_index)\.rs$}"

SYSROOT="$(rustc --print sysroot)"
HOST_TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"
TOOLS_DIR="${SYSROOT}/lib/rustlib/${HOST_TRIPLE}/bin"

if [[ ! -x "${TOOLS_DIR}/llvm-cov" || ! -x "${TOOLS_DIR}/llvm-profdata" ]]; then
  ACTIVE_TOOLCHAIN="$(rustup show active-toolchain | cut -d ' ' -f1)"
  TOOLS_DIR="${HOME}/.rustup/toolchains/${ACTIVE_TOOLCHAIN}/lib/rustlib/${HOST_TRIPLE}/bin"
fi

if [[ -x "${TOOLS_DIR}/llvm-cov" && -x "${TOOLS_DIR}/llvm-profdata" ]]; then
  export LLVM_COV="${TOOLS_DIR}/llvm-cov"
  export LLVM_PROFDATA="${TOOLS_DIR}/llvm-profdata"
fi

cargo llvm-cov \
  --workspace \
  --all-targets \
  --ignore-filename-regex "${IGNORE_REGEX}" \
  --fail-under-lines "${THRESHOLD}" \
  --summary-only
