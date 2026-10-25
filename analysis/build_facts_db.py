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

REPOSITORY_ROOT = Path(__file__).resolve().parent.parent

SCHEMA = """
CREATE TABLE sources (identifier TEXT PRIMARY KEY, citation TEXT, kind TEXT, url TEXT,
                      language TEXT, note TEXT);
CREATE TABLE units (identifier TEXT PRIMARY KEY, name TEXT, dimension TEXT, factor REAL,
                    source TEXT, note TEXT);
CREATE TABLE quantities (identifier TEXT PRIMARY KEY, dimension TEXT, description TEXT);
CREATE TABLE facts (identifier TEXT PRIMARY KEY, topic TEXT, quantity TEXT, value REAL,
                    minimum REAL, maximum REAL, unit TEXT, base_minimum REAL,
                    base_maximum REAL, place TEXT, culture TEXT, region TEXT,
                    year_from INTEGER, year_to INTEGER, era TEXT, source TEXT, locator TEXT,
                    status TEXT, superseded_by TEXT, report TEXT, note TEXT,
                    approximate INTEGER, year_basis TEXT, year_approximate INTEGER,
                    source_via TEXT);
CREATE TABLE disputes (fact TEXT, other TEXT, PRIMARY KEY (fact, other));
"""


def build(data_root: Path, output_path: Path) -> int:
    """Validate, then write the database; return the number of facts written."""
    data = facts.load(data_root)
    errors = facts.validate(data, REPOSITORY_ROOT)
    if errors:
        raise ValueError("data/research/ has errors; run validate_facts.py")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    if output_path.exists():
        output_path.unlink()
    database = sqlite3.connect(output_path)
    try:
        database.executescript(SCHEMA)
        database.executemany(
            "INSERT INTO sources VALUES (:identifier, :citation, :kind, :url, :language, :note)",
            [
                {
                    key: source.get(key)
                    for key in ("identifier", "citation", "kind", "url", "language", "note")
                }
                for source in data.sources
            ],
        )
        database.executemany(
            "INSERT INTO units VALUES (:identifier, :name, :dimension, :factor, :source, :note)",
            [
                {
                    key: unit.get(key)
                    for key in ("identifier", "name", "dimension", "factor", "source", "note")
                }
                for unit in data.units
            ],
        )
        database.executemany(
            "INSERT INTO quantities VALUES (:identifier, :dimension, :description)",
            [{key: quantity.get(key) for key in ("identifier", "dimension", "description")}
             for quantity in data.quantities],
        )
        rows = []
        for fact in data.facts:
            base_minimum, base_maximum = facts.to_base(fact, data.units)
            row = {key: fact.get(key) for key in facts.FACT_KEYS - {"disputes"}}
            row.update(
                topic=data.fact_files[fact["identifier"]].removesuffix(".yaml"),
                base_minimum=base_minimum,
                base_maximum=base_maximum,
                approximate=bool(fact.get("approximate", False)),
                year_approximate=bool(fact.get("year_approximate", False)),
            )
            rows.append(row)
        database.executemany(
            "INSERT INTO facts VALUES (:identifier, :topic, :quantity, :value, :minimum, :maximum, "
            ":unit, :base_minimum, :base_maximum, :place, :culture, :region, :year_from, :year_to, "
            ":era, :source, :locator, :status, :superseded_by, :report, :note, :approximate, "
            ":year_basis, :year_approximate, :source_via)",
            rows,
        )
        database.executemany(
            "INSERT INTO disputes VALUES (?, ?)",
            [
                (fact["identifier"], other)
                for fact in data.facts
                for other in fact.get("disputes", [])
            ],
        )
        database.commit()
    finally:
        database.close()
    return len(data.facts)


def main() -> int:
    data_root = REPOSITORY_ROOT / "data" / "research"
    output_path = REPOSITORY_ROOT / "out" / "facts.sqlite"
    try:
        count = build(data_root, output_path)
    except (ValueError, yaml.YAMLError, OSError) as error:
        print(error, file=sys.stderr)
        return 1
    print(f"wrote {count} facts to {output_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
