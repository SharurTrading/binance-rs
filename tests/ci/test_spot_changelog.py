# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Offline contracts for last-update versus last-review detection."""

from datetime import date
from contextlib import redirect_stdout, redirect_stderr
from io import StringIO
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from scripts.ci import check_spot_changelog as monitor


class ChangelogCheckTests(unittest.TestCase):
    def test_older_and_equal_updates_have_already_been_reviewed(self):
        review = date(2026, 9, 27)
        self.assertFalse(monitor.needs_review(date(2026, 9, 18), review))
        self.assertFalse(monitor.needs_review(review, review))

    def test_newer_update_needs_a_review(self):
        self.assertTrue(monitor.needs_review(date(2026, 9, 28), date(2026, 9, 27)))

    def test_announcement_dates_do_not_override_last_updated(self):
        text = "# CHANGELOG for Binance's API\n\n**Last Updated: 2026-09-18**\n\n### 2026-10-01\nFuture rollout.\n"
        self.assertEqual(monitor.last_updated(text), date(2026, 9, 18))

    def test_empty_html_missing_duplicate_and_invalid_dates_are_errors(self):
        for text in [
            "", "<html>upstream error</html>",
            "# CHANGELOG for Binance's API\n### 2026-09-18\n",
            "# CHANGELOG for Binance's API\n**Last Updated: 2026-02-30**\n",
            "# CHANGELOG for Binance's API\n**Last Updated: 2026-09-18**\n**Last Updated: 2026-09-19**\n",
        ]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                monitor.last_updated(text)

    def test_fetch_error_cannot_be_reported_as_already_reviewed(self):
        with patch.object(monitor, "fetch_changelog", side_effect=OSError("unavailable")), redirect_stderr(StringIO()):
            self.assertEqual(monitor.main([]), 2)


    def test_cli_exit_and_summary_report_the_review_comparison(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            review, source, summary = root / "review.json", root / "source.md", root / "summary.md"
            review.write_text(json.dumps({"last_reviewed": "2026-09-27"}), encoding="utf-8")
            for updated, status in [("2026-09-18", 0), ("2026-09-28", 1)]:
                source.write_text(f"# CHANGELOG for Binance's API\n\n**Last Updated: {updated}**\n", encoding="utf-8")
                with patch.dict(os.environ, {"GITHUB_STEP_SUMMARY": str(summary)}), redirect_stdout(StringIO()):
                    self.assertEqual(monitor.main(["--review", str(review), "--input", str(source)]), status)
                text = summary.read_text(encoding="utf-8")
                self.assertIn(updated, text)
                self.assertIn("2026-09-27", text)
                self.assertIn(monitor.PAGE, text)

    def test_missing_review_configuration_fails_without_a_freshness_claim(self):
        with tempfile.TemporaryDirectory() as directory, redirect_stderr(StringIO()):
            self.assertEqual(monitor.main(["--review", str(Path(directory) / "missing.json")]), 2)


if __name__ == "__main__":
    unittest.main()
