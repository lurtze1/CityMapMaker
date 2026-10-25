# Research facts

The numbers behind the generator's rules, one record per measured fact, taken from the
research reports under `documentation/research/` (ADR-025). The YAML files here are the source of
truth. `output/facts.sqlite` is generated from them for queries and is never edited.

## Files

| File | Holds |
|---|---|
| `sources.yaml` | every source once: `identifier`, `citation`, `kind` (paper, book, map, dataset, web, archive), optional `url`, `language`, `note` |
| `units.yaml` | every unit with its `dimension` and `factor` to the base unit; a factor other than 1 names the `source` it comes from |
| `quantities.yaml` | what a fact can measure, with its dimension |
| `facts/<topic>.yaml` | the facts of one topic, under a top-level `facts:` list |
| `sources/<topic>.yaml`, `units/<topic>.yaml`, `quantities/<topic>.yaml` | entries a topic adds, so topic PRs never edit the same file; an entry repeated identically in two files counts once, a repeated identifier with different content is a duplicate identifier error |

Base units: length in metres, area in square metres, population density in people per
hectare, count, ratio, angle in degrees, time in years.

## One fact

```yaml
facts:
  - identifier: streets.lubeck.main-1300         # topic.place.what, unique across all files
    quantity: street_width                       # an identifier from quantities.yaml
    minimum: 8                                   # either value, or minimum and maximum
    maximum: 12
    unit: m                                      # an identifier from units.yaml, same dimension as the quantity
    place: Lübeck
    culture: hanseatic                           # optional, free word
    region: northern_europe                      # optional, free word
    approximate: false                           # optional: true when the report gives an estimate
    year_from: 1250                              # optional, whole years, negative for BCE
    year_to: 1350
    year_basis: measured                         # required with years: what the year refers to
    year_approximate: false                      # optional: true when the report's year is approximate
    era: high_medieval                           # optional, free word
    source: example-source-identifier            # the original source of the figure
    source_via: null                             # optional: the work through which the report cites it
    locator: "p. 112, table 4"                   # optional: page, table, figure or map sheet
    status: verified                             # verified, unverified, corrected, superseded, disputed
    superseded_by: null                          # only with status superseded: the fact that replaces it
    disputes: []                                 # optional: facts that contradict this one
    report: documentation/research/settlement/streets.md  # the report the fact was taken from
    note: main street; the report gives 8 to 12 m
```

- **`approximate`**: the report gives the figure as an estimate ("about", "c.", "~").
- **`year_basis`**: one of `measured`, `plan_date`, `census`, `built`, `founded`,
  `described`, `estimated`, `depicted`, `valid_until`. `depicted`: the years are the moment
  or period a plan or reconstruction shows (Pompeii as in 79, Hattusa in the 13th century
  BC), not the date the plan was drawn (that is `plan_date`). `valid_until`: the figure held
  until that year and the report gives no start ("before 1634", "until abandoned c. 1837");
  what happened afterwards does not matter. Only `year_to` is set. Required whenever `year_from` or `year_to` is given. A year the
  report does not give is left out; it is never filled in from another figure in the same
  report row.
- **`year_approximate`**: the report's year is approximate ("c. 1300", "early 1600s"). The
  years stay as the report gives them; "c. 1300" is `year_from: 1300, year_to: 1300` with
  `year_approximate: true`, never an invented range. A century is its full range
  (13th century: 1201 to 1300).
- **`source_via`**: when the report cites an original through another work (Grenville cited
  by Dodson), `source` is the original and `source_via` the work the report used.
- **`disputes`**: every fact listed must list this one back; a fact with status `disputed`
  lists at least one.

- **verified**: checked against the source by a verification report or by reading the source.
- **unverified**: taken from a report, not checked against the source yet.
- **corrected**: a verification report corrected the figure; the corrected value is recorded
  and the note gives the old one.
- **superseded**: a later fact replaces this one; it stays, so the history is visible.
- **disputed**: sources disagree and neither is settled; `disputes` names the competing
  facts, and the cross-analysis report for the topic explains the disagreement.

A fact reports its source's own uncertainty: a range stays a range and an approximate figure
sets `approximate` (ADR-023).

## Commands

```bash
python -m venv analysis/.venv
analysis/.venv/Scripts/python -m pip install -r analysis/requirements.txt   # Linux: analysis/.venv/bin/python
analysis/.venv/Scripts/python analysis/validate_facts.py
analysis/.venv/Scripts/python analysis/build_facts_db.py                    # writes output/facts.sqlite
analysis/.venv/Scripts/python -m unittest discover -s analysis/tests
```

CI runs the validator and the tests on Windows and Linux; a broken fact turns CI red.
Browsing the database with Datasette (`datasette output/facts.sqlite`) is optional and not a
project dependency.
