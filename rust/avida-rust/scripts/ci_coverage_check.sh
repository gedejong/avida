#!/usr/bin/env bash
set -euo pipefail

THRESHOLD="${1:-82}"
SUMMARY_OUTPUT="${2:-target/coverage-summary.txt}"

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

mkdir -p "$(dirname "${SUMMARY_OUTPUT}")"

cargo llvm-cov \
  --workspace \
  --all-targets \
  --fail-under-lines "${THRESHOLD}" \
  --summary-only | tee "${SUMMARY_OUTPUT}"

# Guard against accidental scope regression by requiring representative stable modules.
REQUIRED_MODULES=(
  "provider_helpers.rs"
  "resource_history_helpers.rs"
  "time_series_recorder.rs"
  "resource_count_helpers.rs"
  "spatial_res_count_helpers.rs"
)

for module in "${REQUIRED_MODULES[@]}"; do
  if ! grep -Eq "^${module}[[:space:]]" "${SUMMARY_OUTPUT}"; then
    echo "Coverage scope check failed: missing module row '${module}' in ${SUMMARY_OUTPUT}" >&2
    exit 1
  fi
done
