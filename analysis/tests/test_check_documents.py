"""Unit tests for analysis/check_documents.py (#117, ADR-051)."""

from __future__ import annotations

import sys
import textwrap
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import check_documents  # noqa: E402

DIFF = textwrap.dedent(
    """\
    diff --git a/documentation/CODING_STANDARDS.md b/documentation/CODING_STANDARDS.md
    --- a/documentation/CODING_STANDARDS.md
    +++ b/documentation/CODING_STANDARDS.md
    @@ -10,0 +11,2 @@ heading
    +A rule that maybe applies.
    +A rule that names "maybe" as a word.
    diff --git a/documentation/research/streets.md b/documentation/research/streets.md
    --- a/documentation/research/streets.md
    +++ b/documentation/research/streets.md
    @@ -3 +3 @@
    -old line
    +Streets were roughly 4 m wide.
    """
)


class AddedLinesTest(unittest.TestCase):
    def test_added_lines_carry_their_path_and_new_line_number(self) -> None:
        self.assertEqual(
            check_documents.added_lines(DIFF),
            [
                ("documentation/CODING_STANDARDS.md", 11, "A rule that maybe applies."),
                ("documentation/CODING_STANDARDS.md", 12, 'A rule that names "maybe" as a word.'),
                ("documentation/research/streets.md", 3, "Streets were roughly 4 m wide."),
            ],
        )

    def test_a_path_with_a_space_keeps_its_trailing_tab_out(self) -> None:
        diff = "diff --git a/x b/x\n+++ b/documentation/my file.md\t\n@@ -0,0 +1 @@\n+maybe\n"
        self.assertEqual(
            check_documents.added_lines(diff), [("documentation/my file.md", 1, "maybe")]
        )

    def test_a_no_newline_marker_and_a_line_starting_with_plus_signs(self) -> None:
        diff = (
            "diff --git a/x b/x\n+++ b/CLAUDE.md\n@@ -1 +1,2 @@\n-old\n"
            "\\ No newline at end of file\n+first\n+++ second\n"
        )
        self.assertEqual(
            check_documents.added_lines(diff),
            [("CLAUDE.md", 1, "first"), ("CLAUDE.md", 2, "++ second")],
        )

    def test_a_deleted_file_adds_no_lines(self) -> None:
        diff = "--- a/CLAUDE.md\n+++ /dev/null\n@@ -1 +0,0 @@\n-gone\n"
        self.assertEqual(check_documents.added_lines(diff), [])


class HedgesTest(unittest.TestCase):
    def test_a_hedge_word_is_found_but_not_in_quotes_or_research(self) -> None:
        findings = check_documents.hedges(check_documents.added_lines(DIFF))
        self.assertEqual(findings, ['documentation/CODING_STANDARDS.md:11: hedge word "maybe"'])

    def test_whole_words_only_and_any_case(self) -> None:
        lines = [
            ("CLAUDE.md", 1, "Perhaps later."),
            ("CLAUDE.md", 2, "A fewer words; maybeline."),
            ("CLAUDE.md", 3, "Use `if needed` in code."),
        ]
        self.assertEqual(check_documents.hedges(lines), ['CLAUDE.md:1: hedge word "Perhaps"'])

    def test_a_phrase_split_over_two_lines_is_found_on_its_first_line(self) -> None:
        lines = [("CLAUDE.md", 4, "Apply it where"), ("CLAUDE.md", 5, "appropriate.")]
        self.assertEqual(
            check_documents.hedges(lines), ['CLAUDE.md:4: hedge word "where appropriate"']
        )

    def test_typographic_quotes_and_quotes_over_two_lines_are_mentions(self) -> None:
        lines = [
            ("CLAUDE.md", 1, "The brief says “maybe a tower”."),
            ("CLAUDE.md", 2, 'The list: "usually", "where'),
            ("CLAUDE.md", 3, 'appropriate".'),
        ]
        self.assertEqual(check_documents.hedges(lines), [])

    def test_the_cross_analysis_is_checked_and_other_research_is_not(self) -> None:
        lines = [
            ("documentation/research/cross-analysis/core.md", 1, "Perhaps two."),
            ("documentation/research/streets.md", 1, "Perhaps two."),
        ]
        self.assertEqual(
            check_documents.hedges(lines),
            ['documentation/research/cross-analysis/core.md:1: hedge word "Perhaps"'],
        )

    def test_only_markdown_files_are_checked(self) -> None:
        lines = [("documentation/diagram.svg", 1, "maybe")]
        self.assertEqual(check_documents.hedges(lines), [])


class RejectedTest(unittest.TestCase):
    def test_a_section_without_rejected_is_named(self) -> None:
        decisions = textwrap.dedent(
            """\
            # Decisions

            ## ADR-001 First (2026-10-19)
            Decision: one.
            Rejected: two.

            ## ADR-002 Second (2026-10-19)
            Decision: three.

            ## Open decisions
            """
        )
        self.assertEqual(
            check_documents.sections_without_rejected(decisions),
            [
                "documentation/DECISIONS.md: ADR-002 Second (2026-10-19) names no rejected "
                "alternative"
            ],
        )

    def test_a_rejected_line_under_open_decisions_does_not_count(self) -> None:
        decisions = "## ADR-001 Only (2026-10-19)\nDecision: one.\n\n## Open decisions\nRejected: x\n"
        self.assertEqual(
            check_documents.sections_without_rejected(decisions),
            ["documentation/DECISIONS.md: ADR-001 Only (2026-10-19) names no rejected alternative"],
        )

    def test_rejected_counts_only_at_the_start_of_a_line(self) -> None:
        decisions = "## ADR-001 Only (2026-10-19)\nDecision: one, as Rejected before.\n"
        self.assertEqual(len(check_documents.sections_without_rejected(decisions)), 1)

    def test_an_empty_base_compares_against_the_previous_commit(self) -> None:
        self.assertIsInstance(check_documents.changed_text(""), str)
        self.assertIsInstance(check_documents.changed_text("0" * 40), str)

    def test_the_real_decision_log_passes(self) -> None:
        decisions = (check_documents.REPOSITORY_ROOT / check_documents.DECISIONS).read_text(
            encoding="utf-8"
        )
        self.assertEqual(check_documents.sections_without_rejected(decisions), [])


if __name__ == "__main__":
    unittest.main()
