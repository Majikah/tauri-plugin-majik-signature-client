#!/usr/bin/env python3
"""Verify that every generated Tauri command artifact matches gen.py."""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GENERATED = (
    Path("src/commands.rs"),
    Path("src/lib.rs"),
    Path("build.rs"),
    Path("permissions/default.toml"),
    Path("contract/commands.json"),
)


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    contract_path = ROOT / "contract/commands.json"
    contract = json.loads(contract_path.read_text(encoding="utf-8"))
    names = [item["name"] for item in contract["commands"]]

    if contract.get("plugin") != "majik-signature-client":
        fail("unexpected plugin name in contract")
    if contract.get("version") != "1.2.0":
        fail("command contract version is not 1.2.0")
    if contract.get("eventChannel") != "majik-signature-client:event":
        fail("unexpected event channel")
    if len(names) != contract.get("commandCount"):
        fail("commandCount does not match commands array")
    if len(names) != len(set(names)):
        fail("duplicate command name in contract")

    with tempfile.TemporaryDirectory(prefix="majik-plugin-gen-") as tmp:
        temp_root = Path(tmp)
        (temp_root / "src").mkdir(parents=True)
        (temp_root / "permissions").mkdir(parents=True)
        shutil.copy2(ROOT / "gen.py", temp_root / "gen.py")
        subprocess.run(
            [sys.executable, "gen.py"],
            cwd=temp_root,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

        for relative in GENERATED:
            actual = ROOT / relative
            regenerated = temp_root / relative
            if not actual.exists() or not regenerated.exists():
                fail(f"missing generated file: {relative}")
            if actual.read_bytes() != regenerated.read_bytes():
                fail(f"generated file is stale: {relative}; run python3 gen.py")

    commands_source = (ROOT / "src/commands.rs").read_text(encoding="utf-8")
    command_functions = re.findall(r"pub async fn ([a-z0-9_]+)\s*\(", commands_source)
    if command_functions != names:
        fail("commands.rs function list/order does not match command contract")

    library_source = (ROOT / "src/lib.rs").read_text(encoding="utf-8")
    registered = re.findall(r"commands::([a-z0-9_]+)", library_source)
    if registered != names:
        fail("lib.rs handler registration does not match command contract")

    build_source = (ROOT / "build.rs").read_text(encoding="utf-8")
    built = re.findall(r'^\s*"([a-z0-9_]+)",\s*$', build_source, re.MULTILINE)
    if built != names:
        fail("build.rs command manifest does not match command contract")

    permission_source = (ROOT / "permissions/default.toml").read_text(encoding="utf-8")
    permitted = re.findall(r'"allow-([a-z0-9-]+)"', permission_source)
    permitted = [item.replace("-", "_") for item in permitted]
    if permitted != names:
        fail("default permission set does not match command contract")

    required = {
        "snapshot",
        "get_snapshot",
        "verify_file_detached_chain",
        "verify_file_revisions_detached",
    }
    missing = sorted(required.difference(names))
    if missing:
        fail(f"required compatibility/current commands missing: {missing}")

    print(f"OK: {len(names)} unique commands registered, generated and permitted.")


if __name__ == "__main__":
    main()
