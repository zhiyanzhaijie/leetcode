#!/usr/bin/env python3
"""Collect local LeetCode note inputs and optionally fetch the exact statement."""

from __future__ import annotations

import argparse
import html
import json
import re
import sys
import urllib.error
import urllib.request
from html.parser import HTMLParser
from pathlib import Path


DEFAULT_ROOT = Path(__file__).resolve().parents[4]
DEFAULT_LIST = Path.home() / ".cache/nvim/leetcode/problemlist"


class MarkdownParser(HTMLParser):
    BLOCKS = {"p", "div", "pre", "ul", "ol", "li", "h1", "h2", "h3", "h4"}

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.parts: list[str] = []
        self.hrefs: list[str | None] = []
        self.in_pre = 0

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if tag in self.BLOCKS:
            self.parts.append("\n")
        if tag == "li":
            self.parts.append("- ")
        elif tag == "br":
            self.parts.append("\n")
        elif tag == "pre":
            self.in_pre += 1
            self.parts.append("```text\n")
        elif tag == "code" and not self.in_pre:
            self.parts.append("`")
        elif tag == "img":
            src = values.get("src") or ""
            alt = values.get("alt") or "img"
            self.parts.append(f"![{alt}]({src})")
        elif tag == "a":
            self.hrefs.append(values.get("href"))
            self.parts.append("[")

    def handle_endtag(self, tag: str) -> None:
        if tag == "pre":
            self.in_pre = max(0, self.in_pre - 1)
            self.parts.append("\n```\n")
        elif tag == "code" and not self.in_pre:
            self.parts.append("`")
        elif tag == "a":
            href = self.hrefs.pop() if self.hrefs else None
            self.parts.append(f"]({href})" if href else "]")
        elif tag in self.BLOCKS:
            self.parts.append("\n")

    def handle_data(self, data: str) -> None:
        self.parts.append(data)

    def markdown(self) -> str:
        text = html.unescape("".join(self.parts)).replace("\xa0", " ")
        text = re.sub(r"[ \t]+\n", "\n", text)
        text = re.sub(r"\n{3,}", "\n\n", text)
        return text.strip()


def parse_source(path: Path) -> tuple[int, str, str]:
    match = re.fullmatch(r"(\d+)\.([^.].*)\.rs", path.name)
    if not match:
        raise ValueError("source filename must match <id>.<slug>.rs")
    source = path.read_text(encoding="utf-8")
    marked = re.search(r"// @leet start\s*\n(.*?)\n// @leet end", source, re.DOTALL)
    code = marked.group(1).strip() if marked else source.strip()
    return int(match.group(1)), match.group(2), code


def load_metadata(problem_id: int, slug: str, cache: Path) -> dict[str, object]:
    if not cache.is_file():
        return {"frontend_id": problem_id, "title_slug": slug}
    payload = json.loads(cache.read_text(encoding="utf-8"))
    entries = payload.get("data", payload) if isinstance(payload, dict) else payload
    if not isinstance(entries, list):
        return {"frontend_id": problem_id, "title_slug": slug}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        if str(entry.get("frontend_id")) == str(problem_id) or entry.get("title_slug") == slug:
            return entry
    return {"frontend_id": problem_id, "title_slug": slug}


def fetch_question(slug: str) -> dict[str, object]:
    query = """query ($titleSlug: String!) { question(titleSlug: $titleSlug) {
      questionFrontendId title titleSlug difficulty content topicTags { name slug }
    }}"""
    body = json.dumps({"query": query, "variables": {"titleSlug": slug}}).encode()
    request = urllib.request.Request(
        "https://leetcode.com/graphql/",
        data=body,
        headers={"Content-Type": "application/json", "User-Agent": "leetcode-write-notes/1.0"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            payload = json.load(response)
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as error:
        raise RuntimeError(f"failed to fetch LeetCode question: {error}") from error
    question = payload.get("data", {}).get("question")
    if not isinstance(question, dict):
        raise RuntimeError(f"LeetCode returned no question for slug {slug}")
    parser = MarkdownParser()
    parser.feed(str(question.get("content") or ""))
    question["content_markdown"] = parser.markdown()
    question.pop("content", None)
    return question


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path, help="LeetCode solution source file")
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT, help="LeetCode workspace root")
    parser.add_argument("--problem-list", type=Path, default=DEFAULT_LIST, help="leetcode.nvim problem-list cache")
    parser.add_argument("--fetch", action="store_true", help="fetch exact public statement from LeetCode GraphQL")
    args = parser.parse_args()

    try:
        source = args.source.expanduser().resolve(strict=True)
        problem_id, filename_slug, code = parse_source(source)
        metadata = load_metadata(problem_id, filename_slug, args.problem_list.expanduser())
        slug = str(metadata.get("title_slug") or filename_slug)
        output = args.root.expanduser() / "web/src/solutions" / str(problem_id) / "solution.md"
        result: dict[str, object] = {
            "source_path": str(source),
            "problem_id": problem_id,
            "slug": slug,
            "url": str(metadata.get("link") or f"https://leetcode.com/problems/{slug}/"),
            "metadata": metadata,
            "my_solution": code,
            "note_path": str(output),
            "existing_note": output.read_text(encoding="utf-8") if output.is_file() else None,
        }
        if args.fetch:
            result["question"] = fetch_question(slug)
        json.dump(result, sys.stdout, ensure_ascii=False, indent=2)
        sys.stdout.write("\n")
        return 0
    except (OSError, ValueError, RuntimeError, json.JSONDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
