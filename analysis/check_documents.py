"""Check the documents and exit 1 on a finding (#117, ADR-051). CI runs this on every PR.

Two checks: no hedge word on a line a change adds to the documents (research reports under
documentation/research/ are exempt, ADR-023), and every ADR section in DECISIONS.md names
what it rejected.
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
DECISIONS = "documentation/DECISIONS.md"

HUNK_HEADER = re.compile(r"^@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@")
QUOTED = re.compile(r'"[^"]*"|`[^`]*`')
HEDGE = re.compile(r"\b(" + "|".join(re.escape(word) for word in HEDGE_WORDS) + r")\b", re.IGNORECASE)
ADR_HEADING = re.compile(r"^## (ADR-\d+.*)$", re.MULTILINE)


def added_lines(diff: str) -> list[tuple[str, int, str]]:
    """Every line a unified diff adds, as (path, line number, text)."""
    found = []
    path = ""
    number = 0
    for line in diff.splitlines():
        if line.startswith("+++ "):
            path = line[6:] if line.startswith("+++ b/") else ""
        elif match := HUNK_HEADER.match(line):
            number = int(match.group(1))
        elif line.startswith("+") and path:
            found.append((path, number, line[1:]))
            number += 1
        elif not line.startswith("-"):
            number += 1
    return found


def hedges(lines: list[tuple[str, int, str]]) -> list[str]:
    """A finding per hedge word used on an added line; words in quotes or backticks are only
    mentioned, not used, and the research reports are exempt."""
    findings = []
    for path, number, text in lines:
        if not path.endswith(".md") or path.startswith(EXEMPT_FOLDER):
            continue
        for match in HEDGE.finditer(QUOTED.sub("", text)):
            findings.append(f"{path}:{number}: hedge word \"{match.group(1)}\"")
    return findings


def sections_without_rejected(decisions: str) -> list[str]:
    """A finding per ADR section that has no line naming what it rejected."""
    headings = list(ADR_HEADING.finditer(decisions))
    findings = []
    for index, heading in enumerate(headings):
        end = headings[index + 1].start() if index + 1 < len(headings) else len(decisions)
        body = decisions[heading.end() : end]
        if "Rejected" not in body:
            findings.append(f"{DECISIONS}: {heading.group(1)} names no rejected alternative")
    return findings


def main() -> int:
    base = sys.argv[1] if len(sys.argv) > 1 else "origin/main"
    diff = subprocess.run(
        ["git", "diff", "--unified=0", base, "--", *CHECKED_PATHS],
        cwd=REPOSITORY_ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    ).stdout
    decisions = (REPOSITORY_ROOT / DECISIONS).read_text(encoding="utf-8")
    findings = hedges(added_lines(diff)) + sections_without_rejected(decisions)
    for finding in findings:
        print(finding, file=sys.stderr)
    print(f"documents checked against {base}: {len(findings)} findings")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
