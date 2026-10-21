# Research facts

The numbers behind the generator's rules, one record per measured fact, taken from the
research reports under `docs/research/` (ADR-025). The YAML files here are the source of
truth. `out/facts.sqlite` is generated from them for queries and is never edited.

## Files

| File | Holds |
|---|---|
| `sources.yaml` | every source once: `id`, `citation`, `kind` (paper, book, map, dataset, web, archive), optional `url`, `language`, `note` |
| `units.yaml` | every unit with its `dimension` and `factor` to the base unit; a factor other than 1 names the `source` it comes from |
| `quantities.yaml` | what a fact can measure, with its dimension |
| `facts/<topic>.yaml` | the facts of one topic, under a top-level `facts:` list |
| `sources/<topic>.yaml`, `units/<topic>.yaml`, `quantities/<topic>.yaml` | entries a topic adds, so topic PRs never edit the same file; an entry repeated identically in two files counts once, a repeated id with different content is a duplicate-id error |

Base units: length in metres, area in square metres, population density in people per
hectare, count, ratio, angle in degrees, time in years.

## One fact

```yaml
facts:
  - id: streets.lubeck.main-1300        # topic.place.what, unique across all files
    quantity: street_width              # an id from quantities.yaml
    min: 8                              # either value, or min and max
    max: 12
    unit: m                             # an id from units.yaml, same dimension as the quantity
    place: Lübeck
    culture: hanseatic                  # optional, free word
    region: northern_europe             # optional, free word
    year_from: 1250                     # optional, whole years, negative for BCE
    year_to: 1350
    era: high_medieval                  # optional, free word
    source: example-source-id           # an id from sources.yaml
    locator: "p. 112, table 4"          # optional: page, table, figure or map sheet
    status: verified                    # verified, unverified, corrected, superseded, disputed
    superseded_by: null                 # only with status superseded: the fact that replaces it
    report: docs/research/settlement/streets.md   # the report the fact was taken from
    note: main street; the report gives 8 to 12 m
```

- **verified**: checked against the source by a verification report or by reading the source.
- **unverified**: taken from a report, not checked against the source yet.
- **corrected**: a verification report corrected the figure; the corrected value is recorded
  and the note gives the old one.
- **superseded**: a later fact replaces this one; it stays, so the history is visible.
- **disputed**: sources disagree and neither is settled; the cross-analysis report for the
  topic names the competing facts.

A fact reports its source's own uncertainty: a range stays a range and an approximate figure
says so in `note` (ADR-023).

## Commands

```bash
python -m venv analysis/.venv
analysis/.venv/Scripts/python -m pip install -r analysis/requirements.txt   # Linux: analysis/.venv/bin/python
analysis/.venv/Scripts/python analysis/validate_facts.py
analysis/.venv/Scripts/python analysis/build_facts_db.py                    # writes out/facts.sqlite
analysis/.venv/Scripts/python -m unittest discover -s analysis/tests
```

CI runs the validator and the tests on Windows and Linux; a broken fact turns CI red.
Browsing the database with Datasette (`datasette out/facts.sqlite`) is optional and not a
project dependency.
