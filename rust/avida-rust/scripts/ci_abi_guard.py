#!/usr/bin/env python3
"""Guard exported avd_* ABI symbols against accidental removals."""

from __future__ import annotations

import argparse
import pathlib
import sys


def read_symbols(path: pathlib.Path) -> set[str]:
    if not path.exists():
        raise FileNotFoundError(path)
    return {line.strip() for line in path.read_text().splitlines() if line.strip()}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", required=True, help="Baseline symbols file")
    parser.add_argument("--current", required=True, help="Current symbols file")
    args = parser.parse_args()

    baseline_path = pathlib.Path(args.baseline)
    current_path = pathlib.Path(args.current)
    if not baseline_path.is_absolute():
        baseline_path = pathlib.Path.cwd() / baseline_path
    if not current_path.is_absolute():
        current_path = pathlib.Path.cwd() / current_path

    baseline = read_symbols(baseline_path)
    current = read_symbols(current_path)

    removed = sorted(baseline - current)
    added = sorted(current - baseline)

    if added:
        print("ABI note: new avd_* symbols detected (allowed with baseline update):")
        for sym in added:
            print(f"  + {sym}")

    if removed:
        print("ABI guard failure: removed/renamed avd_* symbols detected:", file=sys.stderr)
        for sym in removed:
            print(f"  - {sym}", file=sys.stderr)
        return 1

    print("ABI guard passed: no removed avd_* symbols.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
