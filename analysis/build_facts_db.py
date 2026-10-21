"""Build out/facts.sqlite from data/research/ (ADR-025).

The database is generated and git-ignored; never edit it. Rebuild after every change to the
YAML. Values are stored as written and in the base unit of their dimension (metre, square
metre, people per hectare, count, ratio, degree, year).
"""

from __future__ import annotations

import sqlite3
import sys
from pathlib import Path

import yaml

import facts

REPO_ROOT = Path(__file__).resolve().parent.parent

SCHEMA = """
CREATE TABLE sources (id TEXT PRIMARY KEY, citation TEXT, kind TEXT, url TEXT, language TEXT,
                      note TEXT);
CREATE TABLE units (id TEXT PRIMARY KEY, name TEXT, dimension TEXT, factor REAL, source TEXT,
                    note TEXT);
CREATE TABLE quantities (id TEXT PRIMARY KEY, dimension TEXT, description TEXT);
CREATE TABLE facts (id TEXT PRIMARY KEY, topic TEXT, quantity TEXT, value REAL, min REAL,
                    max REAL, unit TEXT, base_min REAL, base_max REAL, place TEXT,
                    culture TEXT, region TEXT, year_from INTEGER, year_to INTEGER, era TEXT,
                    source TEXT, locator TEXT, status TEXT, superseded_by TEXT, report TEXT,
                    note TEXT);
"""


def build(data_root: Path, out_path: Path) -> int:
    """Validate, then write the database; return the number of facts written."""
    data = facts.load(data_root)
    errors = facts.validate(data, REPO_ROOT)
    if errors:
        raise ValueError("data/research/ has errors; run validate_facts.py")
    out_path.parent.mkdir(parents=True, exist_ok=True)
    if out_path.exists():
        out_path.unlink()
    db = sqlite3.connect(out_path)
    try:
        db.executescript(SCHEMA)
        db.executemany(
            "INSERT INTO sources VALUES (:id, :citation, :kind, :url, :language, :note)",
            [{k: s.get(k) for k in ("id", "citation", "kind", "url", "language", "note")}
             for s in data.sources],
        )
        db.executemany(
            "INSERT INTO units VALUES (:id, :name, :dimension, :factor, :source, :note)",
            [{k: u.get(k) for k in ("id", "name", "dimension", "factor", "source", "note")}
             for u in data.units],
        )
        db.executemany(
            "INSERT INTO quantities VALUES (:id, :dimension, :description)",
            [{k: q.get(k) for k in ("id", "dimension", "description")}
             for q in data.quantities],
        )
        rows = []
        for fact in data.facts:
            base_min, base_max = facts.to_base(fact, data.units)
            row = {key: fact.get(key) for key in facts.FACT_KEYS}
            row.update(
                topic=data.fact_files[fact["id"]].removesuffix(".yaml"),
                base_min=base_min,
                base_max=base_max,
            )
            rows.append(row)
        db.executemany(
            "INSERT INTO facts VALUES (:id, :topic, :quantity, :value, :min, :max, :unit, "
            ":base_min, :base_max, :place, :culture, :region, :year_from, :year_to, :era, "
            ":source, :locator, :status, :superseded_by, :report, :note)",
            rows,
        )
        db.commit()
    finally:
        db.close()
    return len(data.facts)


def main() -> int:
    data_root = REPO_ROOT / "data" / "research"
    out_path = REPO_ROOT / "out" / "facts.sqlite"
    try:
        count = build(data_root, out_path)
    except (ValueError, yaml.YAMLError, OSError) as error:
        print(error, file=sys.stderr)
        return 1
    print(f"wrote {count} facts to {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
