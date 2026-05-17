"""Build-time Python compatibility shims for ESP-IDF tooling on macOS."""

from __future__ import annotations

import platform
import re
import subprocess
import sys
from pathlib import Path


if sys.platform == "darwin":
    _original_mac_ver = platform.mac_ver

    def _detect_macos_version() -> str | None:
        try:
            result = subprocess.run(
                ["/usr/bin/sw_vers", "-productVersion"],
                check=False,
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                text=True,
                timeout=2,
            )
            version = result.stdout.strip()
            if re.fullmatch(r"\d+(?:\.\d+)*", version):
                return version
        except Exception:
            pass

        try:
            plist = Path("/System/Library/CoreServices/SystemVersion.plist").read_text(
                encoding="utf-8",
                errors="ignore",
            )
        except OSError:
            return None

        match = re.search(
            r"<key>ProductVersion</key>\s*<string>([^<]+)</string>",
            plist,
        )
        if match and re.fullmatch(r"\d+(?:\.\d+)*", match.group(1)):
            return match.group(1)

        return None

    def _patched_mac_ver(
        release: str = "",
        versioninfo: tuple[str, str, str] = ("", "", ""),
        machine: str = "",
    ) -> tuple[str, tuple[str, str, str], str]:
        current = _original_mac_ver(release, versioninfo, machine)
        if current[0]:
            return current

        detected = _detect_macos_version()
        if not detected:
            return current

        detected_machine = current[2] or machine or platform.machine()
        return (detected, current[1], detected_machine)

    try:
        if not _original_mac_ver()[0]:
            platform.mac_ver = _patched_mac_ver
    except Exception:
        pass
