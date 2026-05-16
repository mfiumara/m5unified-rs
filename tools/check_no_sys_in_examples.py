#!/usr/bin/env python3
"""Ensure user-facing examples use the safe m5unified crate, not m5unified-sys."""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / "examples/src/bin"


def main() -> int:
    offenders = []
    for path in sorted(EXAMPLES.glob("*.rs")):
        text = path.read_text()
        if "m5unified_sys" in text or "m5unified-sys" in text:
            offenders.append(path.relative_to(ROOT))

    if offenders:
        for offender in offenders:
            print(f"error: {offender} references m5unified-sys directly", file=sys.stderr)
        return 1

    print(f"ok: checked {len(list(EXAMPLES.glob('*.rs')))} examples")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
