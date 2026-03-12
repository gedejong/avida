#!/usr/bin/env python3
"""Generate Rust FFI analysis artifacts.

Outputs:
- avd_symbols.txt: exported avd_* symbols from libavida_rust.a
- extern_c_index.txt: source-level extern "C" avd_* index
- llvm_callgraph.dot: LLVM IR call graph rooted at avd_* exports (best-effort)
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import subprocess
import sys
from typing import Iterable


def run(cmd: list[str], cwd: pathlib.Path) -> str:
    result = subprocess.run(
        cmd,
        cwd=str(cwd),
        text=True,
        capture_output=True,
        check=True,
    )
    return result.stdout


def nm_symbols(lib_path: pathlib.Path, cwd: pathlib.Path) -> list[str]:
    nm_bin = shutil.which("llvm-nm") or shutil.which("nm")
    if nm_bin is None:
        raise RuntimeError("neither llvm-nm nor nm is available")

    nm_proc = subprocess.run(
        [nm_bin, "-g", str(lib_path)],
        cwd=str(cwd),
        text=True,
        capture_output=True,
        check=False,
    )
    out = nm_proc.stdout
    symbols: set[str] = set()
    for line in out.splitlines():
        parts = line.split()
        if not parts:
            continue
        symbol = parts[-1]
        if symbol.startswith("_avd_"):
            symbol = symbol[1:]
        if symbol.startswith("avd_"):
            symbols.add(symbol)
    return sorted(symbols)


def extern_index(src_dir: pathlib.Path) -> list[str]:
    entries: list[str] = []
    pat = re.compile(r'pub\s+extern\s+"C"\s+fn\s+(avd_[A-Za-z0-9_]+)\s*\(')
    for path in sorted(src_dir.glob("*.rs")):
        rel = path.name
        for line_no, line in enumerate(path.read_text().splitlines(), start=1):
            m = pat.search(line)
            if m:
                entries.append(f"{m.group(1)}\t{rel}:{line_no}")
    return entries


def newest_ll_file(target_release_deps: pathlib.Path) -> pathlib.Path | None:
    ll_files = list(target_release_deps.glob("*.ll"))
    if not ll_files:
        return None
    return max(ll_files, key=lambda p: p.stat().st_mtime)


def parse_llvm_callgraph(ll_text: str, roots: Iterable[str]) -> list[tuple[str, str]]:
    roots_set = set(roots)
    edges: list[tuple[str, str]] = []
    def_pat = re.compile(r'^define\b.*@("?)([^"(@]+)\1\(')
    call_pat = re.compile(r'call\b.*@("?)([^"(@]+)\1\(')

    current_fn: str | None = None
    for line in ll_text.splitlines():
        d = def_pat.match(line.strip())
        if d:
            current_fn = d.group(2)
            continue
        if current_fn is None:
            continue
        c = call_pat.search(line)
        if c:
            callee = c.group(2)
            if current_fn in roots_set:
                edges.append((current_fn, callee))
        if line.strip() == "}":
            current_fn = None
    return edges


def write_dot(path: pathlib.Path, roots: list[str], edges: list[tuple[str, str]], note: str | None) -> None:
    lines = ["digraph ffi_callgraph {", "  rankdir=LR;"]
    for root in roots:
        lines.append(f'  "{root}";')
    for src, dst in edges:
        lines.append(f'  "{src}" -> "{dst}";')
    if note:
        lines.append(f'  "analysis_note" [label="{note}"];')
    lines.append("}")
    path.write_text("\n".join(lines) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True, help="Output directory for artifacts")
    args = parser.parse_args()

    crate_dir = pathlib.Path(__file__).resolve().parents[1]
    output_dir = pathlib.Path(args.output)
    if not output_dir.is_absolute():
        output_dir = crate_dir / output_dir
    output_dir.mkdir(parents=True, exist_ok=True)

    # Ensure staticlib is built for symbol extraction.
    run(["cargo", "+stable", "build", "--release", "--lib"], crate_dir)

    lib_path = crate_dir / "target" / "release" / "libavida_rust.a"
    if not lib_path.exists():
        raise RuntimeError(f"missing archive: {lib_path}")

    symbols = nm_symbols(lib_path, crate_dir)
    (output_dir / "avd_symbols.txt").write_text("\n".join(symbols) + ("\n" if symbols else ""))

    extern_entries = extern_index(crate_dir / "src")
    (output_dir / "extern_c_index.txt").write_text(
        "\n".join(extern_entries) + ("\n" if extern_entries else "")
    )

    # Best-effort LLVM call graph.
    note = None
    edges: list[tuple[str, str]] = []
    try:
        run(["cargo", "+stable", "rustc", "--release", "--lib", "--", "--emit=llvm-ir"], crate_dir)
        ll_path = newest_ll_file(crate_dir / "target" / "release" / "deps")
        if ll_path is None:
            note = "No LLVM IR file found after rustc --emit=llvm-ir."
        else:
            edges = parse_llvm_callgraph(ll_path.read_text(), symbols)
    except Exception as exc:  # best-effort only
        note = f"LLVM call graph generation skipped: {exc}"

    write_dot(output_dir / "llvm_callgraph.dot", symbols, edges, note)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        raise
