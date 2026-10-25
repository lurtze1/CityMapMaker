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
import yaml  # noqa: E402

REPOSITORY_ROOT = Path(__file__).resolve().parent.parent.parent

SOURCES = """
sources:
  - { identifier: s1, citation: "A. Author, A Book, 2000", kind: book }
"""
UNITS = """
units:
  - { identifier: m, dimension: length, factor: 1 }
  - { identifier: ha, dimension: area, factor: 10000, source: s1 }
  - { identifier: people_per_ha, dimension: population_density, factor: 1 }
"""
QUANTITIES = """
quantities:
  - { identifier: street_width, dimension: length }
  - { identifier: population_density, dimension: population_density }
"""
GOOD_FACT = """
  - identifier: streets.example.main
    quantity: street_width
    minimum: 6
    maximum: 8
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
            return facts.validate(facts.load(root), REPOSITORY_ROOT)

    def assert_error(self, fact_block: str, fragment: str, units: str = UNITS) -> None:
        errors = self.errors_for(fact_block, units)
        self.assertTrue(any(fragment in error for error in errors), errors)

    def test_good_fact_has_no_errors(self) -> None:
        self.assertEqual(self.errors_for(GOOD_FACT), [])

    def test_real_data_directory_is_valid(self) -> None:
        data = facts.load(REPOSITORY_ROOT / "data" / "research")
        self.assertEqual(facts.validate(data, REPOSITORY_ROOT), [])

    def test_duplicate_identifier(self) -> None:
        self.assert_error(GOOD_FACT + GOOD_FACT, "duplicate identifier")

    def test_duplicate_key_is_rejected_on_load(self) -> None:
        block = GOOD_FACT + "    status: unverified\n"
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), block)
            with self.assertRaisesRegex(yaml.constructor.ConstructorError, "duplicate key 'status'"):
                facts.load(root)

    def test_unknown_field(self) -> None:
        self.assert_error(GOOD_FACT + "    colour: red\n", "unknown field 'colour'")

    def test_missing_required_field(self) -> None:
        self.assert_error(GOOD_FACT.replace("    place: Example\n", ""), "missing place")

    def test_value_and_range_together(self) -> None:
        self.assert_error(GOOD_FACT + "    value: 7\n", "either value or minimum and maximum")

    def test_minimum_greater_than_maximum(self) -> None:
        self.assert_error(GOOD_FACT.replace("minimum: 6", "minimum: 9"), "minimum is greater than maximum")

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

    def test_non_finite_value(self) -> None:
        self.assert_error(GOOD_FACT.replace("maximum: 8", "maximum: .inf"), "must both be numbers")

    def test_oversized_integer(self) -> None:
        self.assert_error(GOOD_FACT.replace("maximum: 8", "maximum: " + "9" * 400), "must both be numbers")

    def test_integer_beyond_sqlite_range(self) -> None:
        self.assert_error(GOOD_FACT.replace("maximum: 8", "maximum: 9223372036854775808"), "must both be numbers")

    def test_year_beyond_sqlite_range(self) -> None:
        self.assert_error(GOOD_FACT + "    year_from: 9223372036854775808\n", "whole year")

    def test_text_field_must_be_text(self) -> None:
        self.assert_error(GOOD_FACT + "    note: 5\n", "note must be text")

    def test_wrong_document_shape_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), GOOD_FACT)
            (root / "units.yaml").write_text("units: {m: 1}\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "must be a list of mappings"):
                facts.load(root)

    def test_reference_that_is_not_text(self) -> None:
        errors = self.errors_for(GOOD_FACT.replace("source: s1", "source: [s1]"))
        self.assertTrue(any("unknown source" in error for error in errors), errors)

    def test_year_basis_required_with_years(self) -> None:
        self.assert_error(GOOD_FACT + "    year_from: 1300\n    year_to: 1300\n", "year_basis is required")

    def test_year_basis_value(self) -> None:
        block = GOOD_FACT + "    year_from: 1300\n    year_to: 1300\n    year_basis: guessed\n"
        self.assert_error(block, "year_basis must be one of")

    def test_valid_until_takes_year_to_only(self) -> None:
        block = GOOD_FACT + "    year_from: 1800\n    year_to: 1837\n    year_basis: valid_until\n"
        self.assert_error(block, "valid_until takes year_to only")

    def test_valid_until_with_year_to_passes(self) -> None:
        self.assertEqual(self.errors_for(GOOD_FACT + "    year_to: 1837\n    year_basis: valid_until\n"), [])

    def test_year_basis_without_years(self) -> None:
        self.assert_error(GOOD_FACT + "    year_basis: census\n", "need year_from or year_to")

    def test_flag_must_be_boolean(self) -> None:
        self.assert_error(GOOD_FACT + "    approximate: yes please\n", "approximate must be true or false")

    def test_unknown_source_via(self) -> None:
        self.assert_error(GOOD_FACT + "    source_via: s9\n", "unknown source_via")

    def test_disputed_needs_disputes(self) -> None:
        self.assert_error(GOOD_FACT.replace("verified", "disputed"), "status disputed needs disputes")

    def test_disputes_must_be_mutual(self) -> None:
        other = GOOD_FACT.replace("streets.example.main", "streets.example.other")
        block = GOOD_FACT + "    disputes: [streets.example.other]\n" + other
        self.assert_error(block, "does not list it back")

    def test_null_source_via_is_omitted(self) -> None:
        self.assertEqual(self.errors_for(GOOD_FACT + "    source_via: null\n"), [])

    def test_disputes_listed_twice(self) -> None:
        other = GOOD_FACT.replace("streets.example.main", "streets.example.other")
        block = (
            GOOD_FACT + "    disputes: [streets.example.other, streets.example.other]\n"
            + other + "    disputes: [streets.example.main]\n"
        )
        self.assert_error(block, "lists the same fact twice")

    def test_mutual_disputes_pass(self) -> None:
        other = GOOD_FACT.replace("streets.example.main", "streets.example.other")
        block = (
            GOOD_FACT + "    disputes: [streets.example.other]\n"
            + other + "    disputes: [streets.example.main]\n"
        )
        self.assertEqual(self.errors_for(block), [])

    def test_report_outside_repository(self) -> None:
        self.assert_error(GOOD_FACT.replace("CLAUDE.md", "../outside.md"), "outside the repository")

    def test_report_not_a_string(self) -> None:
        self.assert_error(GOOD_FACT.replace("report: CLAUDE.md", "report: 5"), "relative to the repository")

    def test_missing_report_file(self) -> None:
        self.assert_error(GOOD_FACT.replace("CLAUDE.md", "nope.md"), "does not exist")

    def test_converted_unit_needs_source(self) -> None:
        units = UNITS.replace(", source: s1", "")
        self.assert_error(GOOD_FACT, "needs a known source", units)


    def test_topic_files_merge_and_identical_repeats_count_once(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), GOOD_FACT)
            (root / "sources").mkdir()
            (root / "sources" / "streets.yaml").write_text(
                SOURCES + '  - { identifier: s2, citation: "B. Author, Another Book, 2001", kind: book }\n',
                encoding="utf-8",
            )
            data = facts.load(root)
            self.assertEqual([source["identifier"] for source in data.sources], ["s1", "s2"])
            self.assertEqual(facts.validate(data, REPOSITORY_ROOT), [])

    def test_conflicting_repeat_is_a_duplicate(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), GOOD_FACT)
            (root / "sources").mkdir()
            (root / "sources" / "streets.yaml").write_text(
                'sources:\n  - { identifier: s1, citation: "Other text", kind: book }\n',
                encoding="utf-8",
            )
            errors = facts.validate(facts.load(root), REPOSITORY_ROOT)
            self.assertTrue(any("duplicate identifier" in error for error in errors), errors)


class BuildTests(unittest.TestCase):
    def test_build_converts_to_base_units(self) -> None:
        units = UNITS + "  - { identifier: km, dimension: length, factor: 1000, source: s1 }\n"
        block = GOOD_FACT.replace("unit: m\n", "unit: km\n").replace(
            "minimum: 6\n    maximum: 8\n", "minimum: 0.002\n    maximum: 0.003\n"
        )
        with tempfile.TemporaryDirectory() as directory:
            root = make_root(Path(directory), block, units)
            output = Path(directory) / "facts.sqlite"
            self.assertEqual(build_facts_db.build(root, output), 1)
            database = sqlite3.connect(output)
            try:
                row = database.execute("SELECT topic, base_minimum, base_maximum FROM facts").fetchone()
            finally:
                database.close()
            self.assertEqual(row[0], "streets")
            self.assertAlmostEqual(row[1], 2.0)
            self.assertAlmostEqual(row[2], 3.0)


if __name__ == "__main__":
    unittest.main()
