"""Check the documents and exit 1 on a finding (#117, ADR-051). CI runs this on every PR.

Two checks: no hedge word in text a change adds to the documents (research reports under
documentation/research/ are exempt, ADR-023, except the cross-analysis), and every ADR
section in DECISIONS.md has a line naming what it rejected.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

REPOSITORY_ROOT = Path(__file__).resolve().parent.parent

HEDGE_WORDS = (
    "usually",
    "should consider",
    "where appropriate",
    "if needed",
    "maybe",
    "perhaps",
    "roughly",
    "a few",
    "wherever possible",
    "as much as possible",
    "if possible",
)

CHECKED_PATHS = ("CLAUDE.md", "documentation")
EXEMPT_FOLDER = "documentation/research/"
CHECKED_RESEARCH_FOLDER = "documentation/research/cross-analysis/"
DECISIONS = "documentation/DECISIONS.md"
EMPTY_BASES = ("", "0" * 40)

HUNK_HEADER = re.compile(r"^@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@")
QUOTED = re.compile(r'"[^"]*"|“[^”]*”|`[^`]*`')
HEDGE = re.compile(
    r"\b(" + "|".join(re.escape(word).replace(r"\ ", r"\s+") for word in HEDGE_WORDS) + r")\b",
    re.IGNORECASE,
)
SECOND_LEVEL_HEADING = re.compile(r"^## ", re.MULTILINE)
ADR_HEADING = re.compile(r"^## (ADR-\d+.*)$", re.MULTILINE)
REJECTED_LINE = re.compile(r"^Rejected", re.MULTILINE)


def added_lines(diff: str) -> list[tuple[str, int, str]]:
    """Every line a unified diff adds, as (path, line number, text)."""
    found = []
    path = ""
    number = 0
    in_header = True
    for line in diff.splitlines():
        if line.startswith("diff --git "):
            in_header = True
            path = ""
        elif in_header and line.startswith("+++ "):
            target = line[4:].rstrip("\t")
            path = target[2:] if target.startswith("b/") else ""
        elif match := HUNK_HEADER.match(line):
            in_header = False
            number = int(match.group(1))
        elif in_header or line.startswith("\\"):
            continue
        elif line.startswith("+") and path:
            found.append((path, number, line[1:]))
            number += 1
        elif not line.startswith("-"):
            number += 1
    return found


def checked(path: str) -> bool:
    """Whether the hedge rule applies to a file."""
    if not path.endswith(".md"):
        return False
    return not path.startswith(EXEMPT_FOLDER) or path.startswith(CHECKED_RESEARCH_FOLDER)


def hedges(lines: list[tuple[str, int, str]]) -> list[str]:
    """A finding per hedge word used in added text. Consecutive added lines are read together,
    so a phrase split by a line break is found; quoted words are mentioned, not used."""
    findings = []
    runs: list[list[tuple[str, int, str]]] = []
    for line in lines:
        previous = runs[-1][-1] if runs else None
        if previous and previous[0] == line[0] and previous[1] + 1 == line[1]:
            runs[-1].append(line)
        else:
            runs.append([line])
    for run in runs:
        path = run[0][0]
        if not checked(path):
            continue
        text = "\n".join(part[2] for part in run)
        for match in HEDGE.finditer(QUOTED.sub(lambda quoted: " " * len(quoted.group(0)), text)):
            number = run[0][1] + text.count("\n", 0, match.start())
            word = " ".join(match.group(1).split())
            findings.append(f'{path}:{number}: hedge word "{word}"')
    return findings


def sections_without_rejected(decisions: str) -> list[str]:
    """A finding per ADR section that has no line starting with `Rejected`."""
    boundaries = [match.start() for match in SECOND_LEVEL_HEADING.finditer(decisions)]
    findings = []
    for heading in ADR_HEADING.finditer(decisions):
        end = next((start for start in boundaries if start > heading.start()), len(decisions))
        if not REJECTED_LINE.search(decisions[heading.end() : end]):
            findings.append(f"{DECISIONS}: {heading.group(1)} names no rejected alternative")
    return findings


def changed_text(base: str) -> str:
    """The diff of the documents against `base`; an empty or all-zero base (a first push)
    compares against the previous commit."""
    reference = "HEAD~1" if base in EMPTY_BASES else base
    return subprocess.run(
        ["git", "-c", "core.quotepath=false", "diff", "--unified=0", reference, "--", *CHECKED_PATHS],
        cwd=REPOSITORY_ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    ).stdout


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    sys.stderr.reconfigure(encoding="utf-8")
    base = sys.argv[1] if len(sys.argv) > 1 else "origin/main"
    try:
        diff = changed_text(base)
    except subprocess.CalledProcessError as error:
        print(f"cannot compare against {base}: {error.stderr.strip()}", file=sys.stderr)
        return 1
    decisions = (REPOSITORY_ROOT / DECISIONS).read_text(encoding="utf-8")
    findings = hedges(added_lines(diff)) + sections_without_rejected(decisions)
    for finding in findings:
        print(finding, file=sys.stderr)
    print(f"documents checked against {base}: {len(findings)} findings")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
