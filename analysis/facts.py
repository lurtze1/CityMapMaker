"""Load and validate the research facts under data/research/ (ADR-025).

The YAML files are the source of truth. `validate` returns a list of error strings; an empty
list means the data is consistent. `load` reads every file into plain dictionaries.
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field
from pathlib import Path

import yaml

STATUSES = ("verified", "unverified", "corrected", "superseded", "disputed")
SOURCE_KINDS = ("paper", "book", "map", "dataset", "web", "archive")
DIMENSIONS = ("length", "area", "population_density", "count", "ratio", "angle", "time")

FACT_KEYS = {
    "id", "quantity", "value", "min", "max", "unit", "place", "culture", "region",
    "year_from", "year_to", "era", "source", "locator", "status", "superseded_by",
    "report", "note",
}
FACT_REQUIRED = ("id", "quantity", "unit", "place", "source", "status", "report")
SOURCE_KEYS = {"id", "citation", "kind", "url", "language", "note"}
UNIT_KEYS = {"id", "name", "dimension", "factor", "source", "note"}
QUANTITY_KEYS = {"id", "dimension", "description"}
SQLITE_INTEGER_MIN = -(2**63)
SQLITE_INTEGER_MAX = 2**63 - 1
SOURCE_TEXT = ("citation", "url", "language", "note")
UNIT_TEXT = ("name", "note")
QUANTITY_TEXT = ("description",)
FACT_TEXT = ("place", "culture", "region", "era", "locator", "note")


@dataclass
class Dataset:
    """Everything under one data root, as read from disk."""

    sources: list[dict] = field(default_factory=list)
    units: list[dict] = field(default_factory=list)
    quantities: list[dict] = field(default_factory=list)
    facts: list[dict] = field(default_factory=list)
    fact_files: dict[str, str] = field(default_factory=dict)


class UniqueKeyLoader(yaml.SafeLoader):
    """SafeLoader that rejects a mapping with the same key twice."""


def _construct_mapping(loader: UniqueKeyLoader, node: yaml.MappingNode, deep: bool = False) -> dict:
    seen = set()
    for key_node, _ in node.value:
        key = loader.construct_object(key_node, deep=deep)
        if key in seen:
            raise yaml.constructor.ConstructorError(
                None, None, f"duplicate key {key!r}", key_node.start_mark
            )
        seen.add(key)
    return loader.construct_mapping(node, deep)


UniqueKeyLoader.add_constructor(
    yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, _construct_mapping
)


def _read(path: Path) -> object:
    with path.open(encoding="utf-8") as handle:
        return yaml.load(handle, Loader=UniqueKeyLoader)


def _read_document(path: Path, name: str) -> list[dict]:
    """Read one file that holds a top-level `name:` list of mappings."""
    content = _read(path)
    if content is None:
        return []
    if not isinstance(content, dict) or name not in content:
        raise ValueError(f"{path}: expected a mapping with a top-level {name!r} key")
    items = content[name] or []
    if not isinstance(items, list) or not all(isinstance(item, dict) for item in items):
        raise ValueError(f"{path}: {name!r} must be a list of mappings")
    return items


def _read_list(root: Path, name: str) -> list[dict]:
    """Read `<name>.yaml` and every `<name>/*.yaml`; an identical repeated entry counts once."""
    paths = [root / f"{name}.yaml"] + sorted((root / name).glob("*.yaml"))
    items: list[dict] = []
    for path in paths:
        if not path.is_file():
            continue
        for item in _read_document(path, name):
            if item not in items:
                items.append(item)
    return items


def load(root: Path) -> Dataset:
    """Read the shared and per-topic sources, units and quantities, and every facts/*.yaml."""
    data = Dataset()
    data.sources = _read_list(root, "sources")
    data.units = _read_list(root, "units")
    data.quantities = _read_list(root, "quantities")
    for path in sorted((root / "facts").glob("*.yaml")):
        for fact in _read_document(path, "facts"):
            data.facts.append(fact)
            data.fact_files[str(fact.get("id"))] = path.name
    return data


def _is_whole(value: object) -> bool:
    return (
        isinstance(value, int)
        and not isinstance(value, bool)
        and SQLITE_INTEGER_MIN <= value <= SQLITE_INTEGER_MAX
    )


def _is_number(value: object) -> bool:
    if isinstance(value, int) and not isinstance(value, bool):
        return _is_whole(value)
    return isinstance(value, float) and math.isfinite(value)


def _ref(value: object) -> str | None:
    """A reference to another entry is text; anything else matches nothing."""
    return value if isinstance(value, str) else None


def _check_text(kind: str, item: dict, keys: tuple[str, ...], errors: list[str]) -> None:
    for key in keys:
        if item.get(key) is not None and not isinstance(item[key], str):
            errors.append(f"{kind} {item.get('id')!r}: {key} must be text")


def _check_keys(kind: str, item: dict, allowed: set[str], errors: list[str]) -> None:
    for key in sorted(set(item) - allowed):
        errors.append(f"{kind} {item.get('id')!r}: unknown field {key!r}")


def _check_unique(kind: str, items: list[dict], errors: list[str]) -> set[str]:
    ids: set[str] = set()
    for item in items:
        item_id = item.get("id")
        if not isinstance(item_id, str) or not item_id:
            errors.append(f"{kind} without an id: {item!r}")
        elif item_id in ids:
            errors.append(f"{kind} {item_id!r}: duplicate id")
        else:
            ids.add(item_id)
    return ids


def validate(data: Dataset, repo_root: Path) -> list[str]:
    """Return every consistency error in `data`; report paths are checked under `repo_root`."""
    errors: list[str] = []

    source_ids = _check_unique("source", data.sources, errors)
    for source in data.sources:
        _check_keys("source", source, SOURCE_KEYS, errors)
        _check_text("source", source, SOURCE_TEXT, errors)
        if not source.get("citation"):
            errors.append(f"source {source.get('id')!r}: missing citation")
        if source.get("kind") not in SOURCE_KINDS:
            errors.append(f"source {source.get('id')!r}: kind must be one of {SOURCE_KINDS}")

    _check_unique("unit", data.units, errors)
    unit_dimension: dict[str, str] = {}
    for unit in data.units:
        _check_keys("unit", unit, UNIT_KEYS, errors)
        _check_text("unit", unit, UNIT_TEXT, errors)
        if unit.get("dimension") not in DIMENSIONS:
            errors.append(f"unit {unit.get('id')!r}: dimension must be one of {DIMENSIONS}")
        if not _is_number(unit.get("factor")) or unit.get("factor") <= 0:
            errors.append(f"unit {unit.get('id')!r}: factor must be a positive number")
        if unit.get("factor") != 1 and _ref(unit.get("source")) not in source_ids:
            errors.append(f"unit {unit.get('id')!r}: a converted unit needs a known source")
        unit_dimension[str(unit.get("id"))] = str(unit.get("dimension"))

    _check_unique("quantity", data.quantities, errors)
    quantity_dimension: dict[str, str] = {}
    for quantity in data.quantities:
        _check_keys("quantity", quantity, QUANTITY_KEYS, errors)
        _check_text("quantity", quantity, QUANTITY_TEXT, errors)
        if quantity.get("dimension") not in DIMENSIONS:
            errors.append(f"quantity {quantity.get('id')!r}: dimension must be one of {DIMENSIONS}")
        quantity_dimension[str(quantity.get("id"))] = str(quantity.get("dimension"))

    fact_ids = _check_unique("fact", data.facts, errors)
    for fact in data.facts:
        name = f"fact {fact.get('id')!r} ({data.fact_files.get(str(fact.get('id')), '?')})"
        _check_keys("fact", fact, FACT_KEYS, errors)
        _check_text(name.split(" (")[0], fact, FACT_TEXT, errors)
        for key in FACT_REQUIRED:
            if fact.get(key) in (None, ""):
                errors.append(f"{name}: missing {key}")
        has_value = "value" in fact
        has_range = "min" in fact or "max" in fact
        if has_value == has_range:
            errors.append(f"{name}: give either value or min and max")
        if has_value and not _is_number(fact["value"]):
            errors.append(f"{name}: value must be a number")
        if has_range:
            low, high = fact.get("min"), fact.get("max")
            if not (_is_number(low) and _is_number(high)):
                errors.append(f"{name}: min and max must both be numbers")
            elif low > high:
                errors.append(f"{name}: min is greater than max")
        if _ref(fact.get("source")) not in source_ids:
            errors.append(f"{name}: unknown source {fact.get('source')!r}")
        unit = _ref(fact.get("unit"))
        quantity = _ref(fact.get("quantity"))
        if unit not in unit_dimension:
            errors.append(f"{name}: unknown unit {fact.get('unit')!r}")
        if quantity not in quantity_dimension:
            errors.append(f"{name}: unknown quantity {fact.get('quantity')!r}")
        if unit in unit_dimension and quantity in quantity_dimension:
            if unit_dimension[unit] != quantity_dimension[quantity]:
                errors.append(
                    f"{name}: unit {unit!r} is {unit_dimension[unit]}, "
                    f"quantity {quantity!r} is {quantity_dimension[quantity]}"
                )
        if fact.get("status") not in STATUSES:
            errors.append(f"{name}: status must be one of {STATUSES}")
        replacement = fact.get("superseded_by")
        if fact.get("status") == "superseded":
            if _ref(replacement) not in fact_ids:
                errors.append(f"{name}: superseded needs superseded_by naming an existing fact")
            elif replacement == fact.get("id"):
                errors.append(f"{name}: superseded_by names itself")
        elif replacement is not None:
            errors.append(f"{name}: superseded_by is only allowed with status superseded")
        year_from, year_to = fact.get("year_from"), fact.get("year_to")
        for key, year in (("year_from", year_from), ("year_to", year_to)):
            if year is not None and not _is_whole(year):
                errors.append(f"{name}: {key} must be a whole year (negative for BCE)")
        if _is_whole(year_from) and _is_whole(year_to) and year_from > year_to:
            errors.append(f"{name}: year_from is after year_to")
        report = fact.get("report")
        if report is not None:
            if not isinstance(report, str) or Path(report).is_absolute():
                errors.append(f"{name}: report must be a path relative to the repository")
            else:
                root = repo_root.resolve()
                target = (root / report).resolve()
                if not target.is_relative_to(root):
                    errors.append(f"{name}: report {report!r} lies outside the repository")
                elif not target.is_file():
                    errors.append(f"{name}: report {report!r} does not exist")
    return errors


def to_base(fact: dict, units: list[dict]) -> tuple[float | None, float | None]:
    """Return the fact's lower and upper value in its dimension's base unit."""
    factor = next(unit["factor"] for unit in units if unit["id"] == fact["unit"])
    if "value" in fact:
        return fact["value"] * factor, fact["value"] * factor
    return fact["min"] * factor, fact["max"] * factor
