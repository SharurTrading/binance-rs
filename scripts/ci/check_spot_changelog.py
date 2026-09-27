# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Read-only comparison of Binance's last changelog update with our last review."""

import argparse
from datetime import date
import json
import os
from pathlib import Path
import re
import sys
import urllib.request

PAGE = "https://developers.binance.com/en/docs/products/spot/CHANGELOG"
SOURCE = "https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/CHANGELOG.md"
DEFAULT_REVIEW = Path(__file__).resolve().parents[2] / ".github/spot-changelog-review.json"


def last_updated(text: str) -> date:
    """Read the explicit update date, never a future announcement heading."""
    if not text.lstrip().startswith("# CHANGELOG for Binance's API\n"):
        raise ValueError("missing official changelog heading")
    matches = re.findall(r"^\*\*Last Updated: (\d{4}-\d{2}-\d{2})\*\*\s*$", text, re.MULTILINE)
    if len(matches) != 1:
        raise ValueError("expected exactly one Last Updated date")
    return date.fromisoformat(matches[0])


def needs_review(updated: date, reviewed: date) -> bool:
    """An update at/before the review date has already been reviewed."""
    return updated > reviewed


def fetch_changelog() -> str:
    """Fetch only the fixed, official public Markdown source, without credentials."""
    request = urllib.request.Request(SOURCE, headers={"User-Agent": "binance-rs-changelog-check"})
    # Ambient proxies are unnecessary for this fixed public documentation read.
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    with opener.open(request, timeout=30) as response:
        if response.status != 200 or response.geturl() != SOURCE:
            raise ValueError("unexpected changelog HTTP status or redirect")
        return response.read().decode("utf-8")


def main(argv=None) -> int:
    """Return 0 when reviewed, 1 when newer, and 2 on fetch/configuration failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--review", type=Path, default=DEFAULT_REVIEW, help="review-date JSON file")
    parser.add_argument("--input", type=Path, help="local Markdown fixture instead of a network fetch")
    args = parser.parse_args(argv)
    try:
        reviewed = date.fromisoformat(json.loads(args.review.read_text(encoding="utf-8"))["last_reviewed"])
        text = args.input.read_text(encoding="utf-8") if args.input else fetch_changelog()
        updated = last_updated(text)
        newer = needs_review(updated, reviewed)
        status = "Review required" if newer else "Already reviewed"
        report = (
            f"{status}: Spot changelog updated {updated}; our last review {reviewed}.\n\n"
            f"[Spot changelog]({PAGE}) · [Official Markdown source]({SOURCE})\n"
        )
        print(report, end="")
        summary = os.environ.get("GITHUB_STEP_SUMMARY")
        if summary:
            with Path(summary).open("a", encoding="utf-8") as output:
                output.write(report)
        return 1 if newer else 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"Changelog check failed ({type(error).__name__}); no freshness conclusion.", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
