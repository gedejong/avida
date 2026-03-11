#!/usr/bin/env python3
"""Soft-gate Rust benchmark regressions from Criterion outputs."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Dict, List, Tuple


BASELINES_NS: Dict[str, float] = {
    "resource_scheduling_helpers/num_steps+remainder_pipeline": 58_429.0,
    "provider_id_helpers/classify_standard": 12.870,
    "provider_id_helpers/classify_argumented_with_outputs": 285.03,
    "provider_id_helpers/classify_malformed": 19.252,
    "package_helpers/str_as_bool": 3.3045,
    "package_helpers/str_as_int": 7.4641,
    "package_helpers/str_as_double": 20.267,
    "package_helpers/double_to_string": 168.16,
}


def bench_from_estimate_path(path: Path) -> str:
    parts = path.parts
    try:
        criterion_idx = parts.index("criterion")
    except ValueError:
        return ""
    if len(parts) <= criterion_idx + 4:
        return ""
    group = parts[criterion_idx + 1]
    bench = parts[criterion_idx + 2]
    return f"{group}/{bench}"


def load_current_estimates(target_dir: Path) -> Dict[str, float]:
    results: Dict[str, float] = {}
    for estimate_file in target_dir.glob("criterion/**/new/estimates.json"):
        bench_name = bench_from_estimate_path(estimate_file)
        if not bench_name:
            continue
        with estimate_file.open("r", encoding="utf-8") as f:
            payload = json.load(f)
        median = payload.get("median", {}).get("point_estimate")
        if isinstance(median, (int, float)):
            results[bench_name] = float(median)
    return results


def fmt_ns(nanoseconds: float) -> str:
    return f"{nanoseconds:.3f} ns"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--threshold", type=float, default=0.20)
    parser.add_argument("--target-dir", default="target")
    args = parser.parse_args()

    current = load_current_estimates(Path(args.target_dir))
    if not current:
        print("error: no Criterion estimates found under target/criterion/**/new/estimates.json")
        return 2

    failures: List[Tuple[str, float, float, float]] = []
    warnings: List[str] = []

    for bench, baseline in BASELINES_NS.items():
        current_value = current.get(bench)
        if current_value is None:
            warnings.append(f"missing benchmark result for '{bench}'")
            continue
        regression = (current_value - baseline) / baseline
        pct = regression * 100.0
        print(
            f"{bench}: current={fmt_ns(current_value)} baseline={fmt_ns(baseline)} delta={pct:+.2f}%"
        )
        if regression > args.threshold:
            failures.append((bench, baseline, current_value, pct))
        elif regression > 0:
            warnings.append(
                f"regression within soft threshold for '{bench}': +{pct:.2f}% "
                f"(threshold +{args.threshold * 100:.2f}%)"
            )

    for warning in warnings:
        print(f"warning: {warning}")

    if failures:
        print("error: benchmark regressions exceeded threshold")
        for bench, baseline, current_value, pct in failures:
            print(
                f"  {bench}: current={fmt_ns(current_value)} baseline={fmt_ns(baseline)} delta={pct:+.2f}%"
            )
        return 1

    print("perf gate passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
