"""Validate data/research/ and exit 1 on any error (ADR-025). CI runs this on every PR."""

from __future__ import annotations

import sys
from pathlib import Path

import facts

REPO_ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    data_root = Path(sys.argv[1]) if len(sys.argv) > 1 else REPO_ROOT / "data" / "research"
    try:
        data = facts.load(data_root)
    except Exception as error:
        print(f"cannot read {data_root}: {error}", file=sys.stderr)
        return 1
    errors = facts.validate(data, REPO_ROOT)
    for error in errors:
        print(error, file=sys.stderr)
    print(
        f"{len(data.facts)} facts, {len(data.sources)} sources, {len(data.units)} units, "
        f"{len(data.quantities)} quantities: {len(errors)} errors"
    )
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
