#!/usr/bin/env python3
"""Generate a rough API inventory from an upstream M5Unified checkout."""
from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path

CALL_RE = re.compile(r"\bM5(?:\.([A-Za-z_][A-Za-z0-9_]*))?(?:\.([A-Za-z_][A-Za-z0-9_]*))?\s*\(")
METHOD_RE = re.compile(r"\bM5\.([A-Za-z_][A-Za-z0-9_]*)(?:\([^)]*\))?\.([A-Za-z_][A-Za-z0-9_]*)\s*\(")
SUBSYSTEM_METHOD_RE = re.compile(r"\bM5\.([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)\s*\(")
CLASS_RE = re.compile(r"\b(M5Canvas|LGFX_Sprite|BluetoothA2DPSink|AudioGenerator\w+|AudioFileSource\w+|AquesTalk)\b")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--upstream", required=True, type=Path, help="Path to a M5Unified checkout")
    args = parser.parse_args()

    examples = args.upstream / "examples"
    by_file = {}
    all_methods = defaultdict(set)
    all_classes = set()

    for ino in sorted(examples.glob("**/*.ino")):
        text = ino.read_text(errors="ignore")
        rel = ino.relative_to(examples).as_posix()
        methods = set()
        for subsystem, method in SUBSYSTEM_METHOD_RE.findall(text):
            methods.add(f"M5.{subsystem}.{method}")
            all_methods[subsystem].add(method)
        for method, nested in CALL_RE.findall(text):
            if method and not nested:
                methods.add(f"M5.{method}")
                all_methods["M5"].add(method)
        classes = set(CLASS_RE.findall(text))
        all_classes |= classes
        by_file[rel] = (sorted(methods), sorted(classes))

    print("# M5Unified upstream example API gap report\n")
    print(f"Generated from `{args.upstream}`.\n")
    print("## Examples\n")
    for rel, (methods, classes) in by_file.items():
        print(f"### `{rel}`")
        print("- Classes: " + (", ".join(f"`{c}`" for c in classes) if classes else "none"))
        print("- Methods:")
        if methods:
            for method in methods:
                print(f"  - `{method}`")
        else:
            print("  - none detected")
        print()

    print("## Methods by subsystem\n")
    for subsystem in sorted(all_methods):
        print(f"### `{subsystem}`")
        for method in sorted(all_methods[subsystem]):
            print(f"- `{method}`")
        print()

    if all_classes:
        print("## External/direct classes\n")
        for klass in sorted(all_classes):
            print(f"- `{klass}`")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
