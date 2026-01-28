#!/usr/bin/env python3
"""Generate canonical BankFlow fixtures with configurable row counts.

This script reuses the builder helpers in generate-fixtures.py but exposes
smaller, reproducible outputs for day-to-day verification (instead of the
20/50 MB stress sets).
"""

from __future__ import annotations

import argparse
import importlib.util
import pathlib
import subprocess
from typing import Any, Callable


ROOT = pathlib.Path(__file__).resolve().parents[1]
GENERATOR_PATH = pathlib.Path(__file__).with_name("generate-fixtures.py")


def load_generator_module() -> Any:
    spec = importlib.util.spec_from_file_location("fixtures_generator", GENERATOR_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"Unable to load generator script at {GENERATOR_PATH}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)  # type: ignore[misc]
    return module


def build_fixture(
    builder: Callable[[int], bytes],
    rows: int,
    destination: pathlib.Path,
    write_fn: Callable[[str, bytes], None],
) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    data = builder(rows)
    write_fn(str(destination), data)


def verify_with_core(file_a: pathlib.Path, file_b: pathlib.Path) -> None:
    manifest = ROOT / "crates" / "bankflow-core" / "Cargo.toml"
    cmd = [
        "cargo",
        "run",
        "--manifest-path",
        str(manifest),
        "--bin",
        "verify_fixtures",
        "--",
        "--file-a",
        str(file_a),
        "--file-b",
        str(file_b),
    ]
    subprocess.run(cmd, check=True)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Generate canonical fixture XLSX files"
    )
    parser.add_argument(
        "--rows-a", type=int, default=1000, help="Rows to generate for transactions"
    )
    parser.add_argument(
        "--rows-b", type=int, default=1000, help="Rows to generate for IP records"
    )
    parser.add_argument(
        "--out-a",
        type=pathlib.Path,
        default=ROOT / "tests" / "fixtures" / "test_transactions_latest.xlsx",
        help="Destination for File A (transactions)",
    )
    parser.add_argument(
        "--out-b",
        type=pathlib.Path,
        default=ROOT / "tests" / "fixtures" / "test_ip_records_latest.xlsx",
        help="Destination for File B (IP records)",
    )
    parser.add_argument(
        "--skip-verify",
        action="store_true",
        help="Skip running cargo verify_fixtures after generation",
    )
    args = parser.parse_args()

    module = load_generator_module()
    build_fixture(module.build_file_a, args.rows_a, args.out_a, module.write_file)
    build_fixture(module.build_file_b, args.rows_b, args.out_b, module.write_file)

    print(f"Generated {args.out_a} (rows={args.rows_a})")
    print(f"Generated {args.out_b} (rows={args.rows_b})")

    if not args.skip_verify:
        verify_with_core(args.out_a, args.out_b)


if __name__ == "__main__":
    main()
