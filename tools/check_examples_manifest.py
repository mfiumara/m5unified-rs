#!/usr/bin/env python3
"""Validate docs/examples/upstream-examples.toml against examples/src/bin."""
from __future__ import annotations

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/examples/upstream-examples.toml"
EXAMPLES = ROOT / "examples/src/bin"


def main() -> int:
    data = tomllib.loads(MANIFEST.read_text())
    entries = data.get("example", [])
    bins = {p.stem for p in EXAMPLES.glob("*.rs")}
    manifest_bins = {entry["rust_bin"] for entry in entries}

    errors: list[str] = []
    for entry in entries:
        rust_bin = entry.get("rust_bin")
        if rust_bin not in bins:
            errors.append(f"manifest entry {rust_bin!r} has no examples/src/bin/{rust_bin}.rs")
        if "upstream" not in entry:
            errors.append(f"manifest entry {rust_bin!r} is missing upstream")
        if entry.get("status") not in {
            "not_started",
            "skeleton",
            "host_compiles",
            "espidf_compiles",
            "hardware_verified",
            "blocked",
        }:
            errors.append(f"manifest entry {rust_bin!r} has invalid status {entry.get('status')!r}")

    missing = sorted(bins - manifest_bins)
    for rust_bin in missing:
        errors.append(f"examples/src/bin/{rust_bin}.rs is not listed in {MANIFEST.relative_to(ROOT)}")

    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print(f"ok: {len(entries)} manifest entries cover {len(bins)} Rust example bins")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
