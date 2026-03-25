#!/usr/bin/env python3
"""Extract include dependency graph from Avida C++ source and output as DOT + JSON.

Usage:
    python3 scripts/extract_include_graph.py [--format dot|json|both] [--level file|dir]

Outputs:
    documentation/graphs/include_graph.dot   — Graphviz DOT format
    documentation/graphs/include_graph.json  — JSON adjacency list
    documentation/graphs/include_graph.svg   — SVG rendering (if graphviz installed)
"""

import os
import re
import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

AVIDA_CORE = Path("avida-core")
SOURCE_DIR = AVIDA_CORE / "source"
INCLUDE_DIR = AVIDA_CORE / "include"

# Directories to scan
SCAN_DIRS = [SOURCE_DIR, INCLUDE_DIR]

# Map short names to full paths for resolution
HEADER_INDEX: dict[str, str] = {}

# Include pattern
INCLUDE_RE = re.compile(r'#include\s+"([^"]+)"')

# avd_ call counting
AVD_RE = re.compile(r'\bavd_\w+')


def index_headers():
    """Build a mapping from header basename to relative path."""
    for scan_dir in SCAN_DIRS:
        if not scan_dir.exists():
            continue
        for root, dirs, files in os.walk(scan_dir):
            for f in files:
                if f.endswith(('.h', '.hh', '.hpp')):
                    full = Path(root) / f
                    rel = str(full.relative_to(AVIDA_CORE))
                    # Index by basename and by various partial paths
                    HEADER_INDEX[f] = rel
                    # Also index by path fragments commonly used in includes
                    parts = full.parts
                    for i in range(len(parts)):
                        partial = os.path.join(*parts[i:])
                        HEADER_INDEX[partial] = rel


def resolve_include(inc_path: str, from_file: Path) -> str | None:
    """Resolve an include path to a canonical relative path."""
    # Try exact match
    if inc_path in HEADER_INDEX:
        return HEADER_INDEX[inc_path]
    # Try basename
    basename = os.path.basename(inc_path)
    if basename in HEADER_INDEX:
        return HEADER_INDEX[basename]
    # Try relative to including file's directory
    candidate = from_file.parent / inc_path
    if candidate.exists():
        try:
            return str(candidate.relative_to(AVIDA_CORE))
        except ValueError:
            return str(candidate)
    return None


def get_directory_label(path: str) -> str:
    """Extract the subsystem directory from a path."""
    parts = path.split("/")
    if "source" in parts:
        idx = parts.index("source")
        if idx + 1 < len(parts):
            return parts[idx + 1]
    if "include" in parts:
        return "include"
    return "other"


def count_avd_calls(filepath: Path) -> int:
    """Count avd_* function calls in a file."""
    try:
        text = filepath.read_text(errors="replace")
        return len(AVD_RE.findall(text))
    except Exception:
        return 0


def count_loc(filepath: Path) -> int:
    """Count lines of code."""
    try:
        return sum(1 for _ in filepath.open(errors="replace"))
    except Exception:
        return 0


def extract_graph():
    """Extract the full include dependency graph."""
    index_headers()

    nodes = {}  # path -> node info
    edges = []  # (from, to)

    for scan_dir in SCAN_DIRS:
        if not scan_dir.exists():
            continue
        for root, dirs, files in os.walk(scan_dir):
            # Skip build artifacts
            if "cbuild" in root or ".git" in root:
                continue
            for f in files:
                if not f.endswith(('.cc', '.cpp', '.c', '.h', '.hh', '.hpp')):
                    continue

                filepath = Path(root) / f
                try:
                    rel_path = str(filepath.relative_to(AVIDA_CORE))
                except ValueError:
                    continue

                loc = count_loc(filepath)
                avd = count_avd_calls(filepath) if f.endswith(('.cc', '.cpp', '.c')) else 0
                directory = get_directory_label(rel_path)

                nodes[rel_path] = {
                    "id": rel_path,
                    "label": f,
                    "directory": directory,
                    "loc": loc,
                    "avd_calls": avd,
                    "is_header": f.endswith(('.h', '.hh', '.hpp')),
                }

                try:
                    text = filepath.read_text(errors="replace")
                except Exception:
                    continue

                for match in INCLUDE_RE.finditer(text):
                    inc_path = match.group(1)
                    resolved = resolve_include(inc_path, filepath)
                    if resolved:
                        edges.append((rel_path, resolved))

    return nodes, edges


def build_dir_graph(nodes, edges):
    """Aggregate to directory-level graph."""
    dir_nodes = defaultdict(lambda: {"loc": 0, "files": 0, "avd_calls": 0})
    dir_edges = defaultdict(int)  # (from_dir, to_dir) -> weight

    for path, info in nodes.items():
        d = info["directory"]
        dir_nodes[d]["loc"] += info["loc"]
        dir_nodes[d]["files"] += 1
        dir_nodes[d]["avd_calls"] += info["avd_calls"]

    for src, dst in edges:
        src_dir = nodes.get(src, {}).get("directory", "?")
        dst_dir = nodes.get(dst, {}).get("directory", "?")
        if src_dir != dst_dir:
            dir_edges[(src_dir, dst_dir)] += 1

    return dict(dir_nodes), dict(dir_edges)


# Color scheme by directory
DIR_COLORS = {
    "main": "#FF6B6B",
    "cpu": "#4ECDC4",
    "actions": "#45B7D1",
    "analyze": "#96CEB4",
    "tools": "#FFEAA7",
    "viewer": "#DDA0DD",
    "data": "#98D8C8",
    "script": "#F7DC6F",
    "systematics": "#BB8FCE",
    "core": "#AED6F1",
    "targets": "#D5DBDB",
    "include": "#FAD7A0",
    "other": "#CCCCCC",
}


def write_dot_file_level(nodes, edges, output_path):
    """Write file-level DOT graph."""
    with open(output_path, "w") as f:
        f.write("digraph avida_includes {\n")
        f.write("  rankdir=LR;\n")
        f.write("  node [shape=box, style=filled, fontsize=8];\n")
        f.write("  edge [color=\"#999999\", arrowsize=0.5];\n")
        f.write("  overlap=false;\n")
        f.write("  splines=true;\n\n")

        # Group by directory
        dirs = defaultdict(list)
        for path, info in nodes.items():
            dirs[info["directory"]].append((path, info))

        for directory, file_list in sorted(dirs.items()):
            color = DIR_COLORS.get(directory, "#CCCCCC")
            cluster_id = sanitize_dot_id(directory)
            f.write(f'  subgraph cluster_{cluster_id} {{\n')
            f.write(f'    label="{directory}";\n')
            f.write(f'    style=filled;\n')
            f.write(f'    color="{color}40";\n')
            f.write(f'    fillcolor="{color}20";\n')
            for path, info in file_list:
                node_id = path.replace("/", "__").replace(".", "_").replace("-", "_")
                label = info["label"]
                if info["avd_calls"] > 0:
                    label += f"\\n({info['avd_calls']} avd_)"
                penwidth = max(1, min(info["loc"] / 1000, 5))
                f.write(f'    {node_id} [label="{label}", '
                        f'fillcolor="{color}", penwidth={penwidth:.1f}];\n')
            f.write("  }\n\n")

        # Edges (only cross-directory to reduce clutter, or all if small)
        seen = set()
        for src, dst in edges:
            if src not in nodes or dst not in nodes:
                continue
            src_id = src.replace("/", "__").replace(".", "_").replace("-", "_")
            dst_id = dst.replace("/", "__").replace(".", "_").replace("-", "_")
            key = (src_id, dst_id)
            if key in seen:
                continue
            seen.add(key)
            # Only show cross-directory edges for readability
            if nodes[src]["directory"] != nodes[dst]["directory"]:
                f.write(f"  {src_id} -> {dst_id};\n")

        f.write("}\n")


def sanitize_dot_id(name: str) -> str:
    """Make a string safe for use as a DOT node identifier."""
    return name.replace("-", "_").replace(".", "_").replace("/", "__")


def write_dot_dir_level(dir_nodes, dir_edges, output_path):
    """Write directory-level DOT graph."""
    with open(output_path, "w") as f:
        f.write("digraph avida_modules {\n")
        f.write("  rankdir=TB;\n")
        f.write("  node [shape=box, style=filled, fontsize=12, fontname=\"Helvetica\"];\n")
        f.write("  edge [fontsize=9];\n\n")

        for d, info in sorted(dir_nodes.items()):
            color = DIR_COLORS.get(d, "#CCCCCC")
            node_id = sanitize_dot_id(d)
            label = f"{d}\\n{info['loc']:,} LOC\\n{info['files']} files"
            if info["avd_calls"] > 0:
                label += f"\\n{info['avd_calls']} avd_ calls"
            penwidth = max(1, min(info["loc"] / 5000, 6))
            f.write(f'  {node_id} [label="{label}", fillcolor="{color}", '
                    f'penwidth={penwidth:.1f}];\n')

        f.write("\n")
        for (src, dst), weight in sorted(dir_edges.items(), key=lambda x: -x[1]):
            if weight >= 3:  # Only show significant edges
                src_id = sanitize_dot_id(src)
                dst_id = sanitize_dot_id(dst)
                penwidth = max(0.5, min(weight / 10, 5))
                f.write(f'  {src_id} -> {dst_id} [label="{weight}", '
                        f'penwidth={penwidth:.1f}];\n')

        f.write("}\n")


def write_json(nodes, edges, dir_nodes, dir_edges, output_path):
    """Write JSON graph data."""
    data = {
        "file_level": {
            "nodes": list(nodes.values()),
            "edges": [{"from": s, "to": t} for s, t in edges],
        },
        "directory_level": {
            "nodes": [{"id": d, **info} for d, info in dir_nodes.items()],
            "edges": [{"from": s, "to": t, "weight": w}
                      for (s, t), w in dir_edges.items()],
        },
    }
    with open(output_path, "w") as f:
        json.dump(data, f, indent=2)


def main():
    level = "both"
    fmt = "both"
    for arg in sys.argv[1:]:
        if arg.startswith("--level="):
            level = arg.split("=")[1]
        elif arg.startswith("--format="):
            fmt = arg.split("=")[1]

    print("Indexing headers...")
    nodes, edges = extract_graph()
    print(f"Found {len(nodes)} files, {len(edges)} include edges")

    dir_nodes, dir_edges = build_dir_graph(nodes, edges)
    print(f"Found {len(dir_nodes)} directories, {len(dir_edges)} cross-directory edges")

    outdir = Path("documentation/graphs")
    outdir.mkdir(parents=True, exist_ok=True)

    if fmt in ("dot", "both"):
        # Directory-level graph (overview)
        dot_dir = outdir / "module_graph.dot"
        write_dot_dir_level(dir_nodes, dir_edges, dot_dir)
        print(f"Wrote {dot_dir}")

        # Render SVG
        svg_dir = outdir / "module_graph.svg"
        try:
            subprocess.run(["dot", "-Tsvg", str(dot_dir), "-o", str(svg_dir)], check=True)
            print(f"Rendered {svg_dir}")
        except (subprocess.CalledProcessError, FileNotFoundError):
            print("Warning: graphviz 'dot' not available, skipping SVG render")

        if level in ("file", "both"):
            # File-level graph (detailed)
            dot_file = outdir / "include_graph.dot"
            write_dot_file_level(nodes, edges, dot_file)
            print(f"Wrote {dot_file}")

            svg_file = outdir / "include_graph.svg"
            try:
                subprocess.run(
                    ["dot", "-Tsvg", str(dot_file), "-o", str(svg_file)],
                    check=True, timeout=120
                )
                print(f"Rendered {svg_file}")
            except subprocess.TimeoutExpired:
                print("Warning: file-level graph too large for dot, trying sfdp...")
                try:
                    subprocess.run(
                        ["sfdp", "-Tsvg", str(dot_file), "-o", str(svg_file)],
                        check=True, timeout=120
                    )
                    print(f"Rendered {svg_file} (via sfdp)")
                except Exception:
                    print("Warning: could not render file-level SVG")
            except Exception:
                print("Warning: could not render file-level SVG")

    if fmt in ("json", "both"):
        json_path = outdir / "include_graph.json"
        write_json(nodes, edges, dir_nodes, dir_edges, json_path)
        print(f"Wrote {json_path}")

    # Print summary
    print("\n=== Module Dependency Summary ===")
    for d, info in sorted(dir_nodes.items(), key=lambda x: -x[1]["loc"]):
        deps_out = sum(1 for (s, _) in dir_edges if s == d)
        deps_in = sum(1 for (_, t) in dir_edges if t == d)
        print(f"  {d:15s}  {info['loc']:>6,} LOC  {info['files']:>3} files  "
              f"in:{deps_in:>2}  out:{deps_out:>2}  avd:{info['avd_calls']:>4}")


if __name__ == "__main__":
    main()
