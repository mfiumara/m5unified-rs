#!/usr/bin/env python3
"""Validate docs/examples/upstream-examples.toml against examples/src/bin."""
from __future__ import annotations

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/examples/upstream-examples.toml"
EXAMPLES = ROOT / "examples/src/bin"
OPTIONAL_INTEGRATIONS = {
    "aquestalk",
    "bluetooth-a2dp",
    "mp3-decoder",
    "webradio-mp3",
    "wifi",
}
ALLOWED_STATUSES = {
    "not_started",
    "skeleton",
    "host_compiles",
    "espidf_compiles",
    "hardware_verified",
    "blocked",
}


def main() -> int:
    data = tomllib.loads(MANIFEST.read_text())
    entries = data.get("example", [])
    bins = {p.stem for p in EXAMPLES.glob("*.rs")}
    manifest_bins = {entry["rust_bin"] for entry in entries}

    errors: list[str] = []
    for entry in entries:
        rust_bin = entry.get("rust_bin")
        status = entry.get("status")
        source = EXAMPLES / f"{rust_bin}.rs" if rust_bin else None
        source_text = source.read_text() if source and source.exists() else ""

        if rust_bin not in bins:
            errors.append(f"manifest entry {rust_bin!r} has no examples/src/bin/{rust_bin}.rs")
        if "upstream" not in entry:
            errors.append(f"manifest entry {rust_bin!r} is missing upstream")
        if status not in ALLOWED_STATUSES:
            errors.append(f"manifest entry {rust_bin!r} has invalid status {status!r}")

        notes = entry.get("notes", "")
        requires = set(entry.get("requires", []))
        uses_unavailable_path = "unavailable_integration" in source_text

        if status == "blocked":
            if "blocked" not in notes.lower():
                errors.append(f"blocked manifest entry {rust_bin!r} must explain the blocker in notes")
            if not uses_unavailable_path:
                errors.append(
                    f"blocked manifest entry {rust_bin!r} must use unavailable_integration()"
                )

        if uses_unavailable_path and status != "blocked":
            errors.append(
                f"manifest entry {rust_bin!r} uses unavailable_integration() but is not blocked"
            )

        optional_requires = requires & OPTIONAL_INTEGRATIONS
        if optional_requires and status not in {"blocked", "espidf_compiles", "hardware_verified"}:
            missing = ", ".join(sorted(optional_requires))
            errors.append(
                f"manifest entry {rust_bin!r} requires optional integration(s) {missing} "
                f"but has status {status!r}"
            )

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
