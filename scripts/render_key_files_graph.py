#!/usr/bin/env python3
"""Render a focused, force-directed graph of the most important Avida files.

Filters to files that matter (high LOC, high connectivity, or high FFI density)
and uses sfdp/fdp for a readable force-directed layout.

Usage:
    python3 scripts/render_key_files_graph.py [--top N] [--min-loc N]
"""

import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

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
}


def sanitize(name: str) -> str:
    return name.replace("/", "__").replace(".", "_").replace("-", "_")


def main():
    top_n = 60
    min_loc = 100

    for arg in sys.argv[1:]:
        if arg.startswith("--top="):
            top_n = int(arg.split("=")[1])
        elif arg.startswith("--min-loc="):
            min_loc = int(arg.split("=")[1])

    graph_path = Path("documentation/graphs/include_graph.json")
    if not graph_path.exists():
        print("Run scripts/extract_include_graph.py first")
        sys.exit(1)

    data = json.loads(graph_path.read_text())
    nodes = {n["id"]: n for n in data["file_level"]["nodes"]}
    edges = [(e["from"], e["to"]) for e in data["file_level"]["edges"]]

    # Compute connectivity
    in_degree = defaultdict(int)
    out_degree = defaultdict(int)
    for src, dst in edges:
        out_degree[src] += 1
        in_degree[dst] += 1

    # Score each file: combination of LOC, connectivity, and FFI density
    scores = {}
    for nid, n in nodes.items():
        loc_score = min(n["loc"] / 1000, 10)
        conn_score = (in_degree.get(nid, 0) + out_degree.get(nid, 0)) / 5
        avd_score = n["avd_calls"] / 10
        scores[nid] = loc_score + conn_score + avd_score

    # Select top N files
    ranked = sorted(scores.items(), key=lambda x: -x[1])
    selected = set()
    for nid, score in ranked[:top_n]:
        if nodes[nid]["loc"] >= min_loc or nodes[nid]["avd_calls"] > 0:
            selected.add(nid)

    # Filter edges to selected nodes
    filtered_edges = [(s, d) for s, d in edges if s in selected and d in selected]

    # Deduplicate edges
    seen_edges = set()
    unique_edges = []
    for s, d in filtered_edges:
        key = (s, d)
        if key not in seen_edges:
            seen_edges.add(key)
            unique_edges.append((s, d))

    print(f"Selected {len(selected)} key files, {len(unique_edges)} edges")

    # Write DOT
    outdir = Path("documentation/graphs")
    dot_path = outdir / "key_files_graph.dot"

    with open(dot_path, "w") as f:
        f.write("digraph avida_key_files {\n")
        f.write("  overlap=prism;\n")
        f.write("  overlap_scaling=2;\n")
        f.write("  sep=\"15\";\n")
        f.write("  splines=true;\n")
        f.write("  node [shape=box, style=filled, fontsize=9, fontname=\"Helvetica\"];\n")
        f.write("  edge [color=\"#88888866\", arrowsize=0.4];\n\n")

        # Write nodes grouped by directory
        dirs = defaultdict(list)
        for nid in selected:
            n = nodes[nid]
            dirs[n["directory"]].append(n)

        for directory, file_list in sorted(dirs.items()):
            color = DIR_COLORS.get(directory, "#CCCCCC")
            f.write(f"  /* {directory} */\n")
            for n in sorted(file_list, key=lambda x: -x["loc"]):
                node_id = sanitize(n["id"])
                label = n["label"]
                loc = n["loc"]
                avd = n["avd_calls"]

                # Size by LOC
                width = max(1.0, min(loc / 500, 4.0))
                height = max(0.4, min(loc / 1500, 1.5))

                # Border thickness by avd_ density
                penwidth = max(1, min(avd / 10 + 1, 6))

                # Build label
                parts = [label, f"{loc:,} LOC"]
                if avd > 0:
                    parts.append(f"{avd} avd_")
                in_d = in_degree.get(n["id"], 0)
                if in_d > 5:
                    parts.append(f"in:{in_d}")
                label_str = "\\n".join(parts)

                # Use the directory color directly
                fillcolor = color

                f.write(f'  {node_id} [label="{label_str}", '
                        f'fillcolor="{fillcolor}", width={width:.1f}, '
                        f'height={height:.1f}, penwidth={penwidth:.1f}];\n')
            f.write("\n")

        # Write edges
        for src, dst in unique_edges:
            src_id = sanitize(src)
            dst_id = sanitize(dst)
            # Highlight cross-directory edges
            src_dir = nodes[src]["directory"]
            dst_dir = nodes[dst]["directory"]
            if src_dir != dst_dir:
                f.write(f'  {src_id} -> {dst_id} [color="#555555"];\n')
            else:
                f.write(f'  {src_id} -> {dst_id};\n')

        f.write("}\n")

    print(f"Wrote {dot_path}")

    # Render with sfdp (force-directed, good for large graphs)
    svg_path = outdir / "key_files_graph.svg"
    try:
        subprocess.run(
            ["sfdp", "-Tsvg", "-Goverlap=prism", "-Gsplines=true",
             str(dot_path), "-o", str(svg_path)],
            check=True, timeout=60
        )
        print(f"Rendered {svg_path}")
    except Exception as e:
        print(f"sfdp failed ({e}), trying fdp...")
        try:
            subprocess.run(
                ["fdp", "-Tsvg", str(dot_path), "-o", str(svg_path)],
                check=True, timeout=60
            )
            print(f"Rendered {svg_path} (via fdp)")
        except Exception as e2:
            print(f"fdp also failed: {e2}")

    # Also render as PNG for quick preview
    png_path = outdir / "key_files_graph.png"
    try:
        subprocess.run(
            ["sfdp", "-Tpng", "-Gdpi=150", "-Goverlap=prism", "-Gsplines=true",
             str(dot_path), "-o", str(png_path)],
            check=True, timeout=60
        )
        print(f"Rendered {png_path}")
    except Exception:
        pass


if __name__ == "__main__":
    main()
