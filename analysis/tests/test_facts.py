"""Unit tests for analysis/facts.py: one test per validation message (ADR-025)."""

from __future__ import annotations

import sqlite3
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import build_facts_db  # noqa: E402
import facts  # noqa: E402

REPO_ROOT = Path(__file__).resolve().parent.parent.parent

SOURCES = """
sources:
  - { id: s1, citation: "A. Author, A Book, 2000", kind: book }
"""
UNITS = """
units:
  - { id: m, dimension: length, factor: 1 }
  - { id: ha, dimension: area, factor: 10000, source: s1 }
  - { id: people_per_ha, dimension: population_density, factor: 1 }
"""
QUANTITIES = """
quantities:
  - { id: street_width, dimension: length }
  - { id: population_density, dimension: population_density }
"""
GOOD_FACT = """
  - id: streets.example.main
    quantity: street_width
    min: 6
    max: 8
    unit: m
    place: Example
    source: s1
    status: verified
    report: CLAUDE.md
"""


def make_root(directory: Path, fact_block: str, units: str = UNITS) -> Path:
    root = directory / "research"
    (root / "facts").mkdir(parents=True)
    (root / "sources.yaml").write_text(textwrap.dedent(SOURCES), encoding="utf-8")
    (root / "units.yaml").write_text(textwrap.dedent(units), encoding="utf-8")
    (root / "quantities.yaml").write_text(textwrap.dedent(QUANTITIES), encoding="utf-8")
    (root / "facts" / "streets.yaml").write_text("facts:" + fact_block, encoding="utf-8")
    return root


class ValidateTests(unittest.TestCase):
    def errors_for(self, fact_block: str, units: str = UNITS) -> list[str]:
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), fact_block, units)
            return facts.validate(facts.load(root), REPO_ROOT)

    def assert_error(self, fact_block: str, fragment: str, units: str = UNITS) -> None:
        errors = self.errors_for(fact_block, units)
        self.assertTrue(any(fragment in error for error in errors), errors)

    def test_good_fact_has_no_errors(self) -> None:
        self.assertEqual(self.errors_for(GOOD_FACT), [])

    def test_real_data_directory_is_valid(self) -> None:
        data = facts.load(REPO_ROOT / "data" / "research")
        self.assertEqual(facts.validate(data, REPO_ROOT), [])

    def test_duplicate_id(self) -> None:
        self.assert_error(GOOD_FACT + GOOD_FACT, "duplicate id")

    def test_duplicate_key_is_rejected_on_load(self) -> None:
        block = GOOD_FACT + "    status: unverified\n"
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), block)
            with self.assertRaises(Exception):
                facts.load(root)

    def test_unknown_field(self) -> None:
        self.assert_error(GOOD_FACT + "    colour: red\n", "unknown field 'colour'")

    def test_missing_required_field(self) -> None:
        self.assert_error(GOOD_FACT.replace("    place: Example\n", ""), "missing place")

    def test_value_and_range_together(self) -> None:
        self.assert_error(GOOD_FACT + "    value: 7\n", "either value or min and max")

    def test_min_greater_than_max(self) -> None:
        self.assert_error(GOOD_FACT.replace("min: 6", "min: 9"), "min is greater than max")

    def test_unknown_source(self) -> None:
        self.assert_error(GOOD_FACT.replace("source: s1", "source: s9"), "unknown source")

    def test_unknown_unit(self) -> None:
        self.assert_error(GOOD_FACT.replace("unit: m\n", "unit: ell\n"), "unknown unit")

    def test_unit_of_wrong_dimension(self) -> None:
        self.assert_error(GOOD_FACT.replace("unit: m\n", "unit: ha\n"), "unit 'ha' is area")

    def test_bad_status(self) -> None:
        self.assert_error(GOOD_FACT.replace("verified", "probable"), "status must be one of")

    def test_superseded_needs_replacement(self) -> None:
        self.assert_error(GOOD_FACT.replace("verified", "superseded"), "superseded needs")

    def test_superseded_by_without_status(self) -> None:
        self.assert_error(GOOD_FACT + "    superseded_by: x\n", "only allowed with status")

    def test_year_order(self) -> None:
        block = GOOD_FACT + "    year_from: 1300\n    year_to: 1200\n"
        self.assert_error(block, "year_from is after year_to")

    def test_missing_report_file(self) -> None:
        self.assert_error(GOOD_FACT.replace("CLAUDE.md", "nope.md"), "does not exist")

    def test_converted_unit_needs_source(self) -> None:
        units = UNITS.replace(", source: s1", "")
        self.assert_error(GOOD_FACT, "needs a known source", units)


class BuildTests(unittest.TestCase):
    def test_build_converts_to_base_units(self) -> None:
        block = GOOD_FACT.replace("min: 6\n    max: 8\n", "value: 2\n").replace(
            "street_width", "population_density"
        ).replace("unit: m\n", "unit: people_per_ha\n")
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), block)
            out = Path(directory) / "facts.sqlite"
            self.assertEqual(build_facts_db.build(root, out), 1)
            db = sqlite3.connect(out)
            try:
                row = db.execute("SELECT topic, base_min, base_max FROM facts").fetchone()
            finally:
                db.close()
            self.assertEqual(row, ("streets", 2.0, 2.0))


if __name__ == "__main__":
    unittest.main()
