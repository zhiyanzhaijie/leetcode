---
name: leetcode-write-notes
description: >-
  Generate or update personal LeetCode solution notes from a leetcode.nvim
  solution file, especially Rust files named like `2471.problem-slug.rs`. Use
  when Codex is asked to整理、生成、补全 or document a completed LeetCode
  problem under the local nvim LeetCode workspace: acquire the exact problem
  summary and examples, extract and explain the user's submitted solution as
  My Solution, select and implement the most canonical classic solution, and
  draft an optional Mermaid diagram for later hand-drawn SVG creation.
---

# LeetCode Solution Notes

Create an accurate, concise `solution.md` while preserving the author's existing note style and hand-drawn assets.

Operate only inside the repository containing this skill. Do not install, copy, or update a global skill under `~/.codex/skills`.

## Workflow

1. Resolve the repository root as the directory containing `.agents/skills/leetcode-write-notes`. Resolve the source Rust file supplied by the user; if only an ID or current file is given, search `<repo-root>/<id>.*.rs`.
2. Read `references/note-format.md` completely before drafting or updating a note.
3. Run `python3 scripts/collect_problem.py <source.rs>` to collect the ID, slug, code, local problem-list metadata, existing note, and output path. Add `--fetch` when exact statement content is not already present and network access is available.
4. Treat the fetched/plugin statement as the source of truth. Never reconstruct examples, constraints, titles, or difficulty from model memory. If exact statement data cannot be acquired locally or from LeetCode, stop and ask the user to open/copy the plugin description or allow fetching.
5. Analyze the code between `// @leet start` and `// @leet end` as the author's first solution. Explain what it actually does, including invariants, chosen data structures, complexity, strengths, and correctness or robustness concerns. Do not silently repair it or claim correctness when a defect exists.
6. Select classic solutions from established algorithm knowledge. Default to exactly one: the best-known, clearest approach using the most appropriate data structure. Include additional classic solutions only when they teach a genuinely different reusable idea or the user asks for them.
7. Write fresh classic Rust code. Preserve the LeetCode signature, prefer `use foo::bar` imports, and never use `unwrap`. Keep it idiomatic, minimal, and compatible with the problem constraints.
8. Decide whether a visual materially clarifies state transitions, structure, or an invariant. If yes, include a compact Mermaid draft. If no, omit it. Never generate or overwrite SVG/image files unless explicitly requested.
9. Create or update `<repo-root>/web/src/solutions/<id>/solution.md`. Preserve existing prose, manually edited code, image links, Mermaid, and SVG files unless the user explicitly requests replacement. For an existing note, merge only missing or requested sections.
10. Read `<repo-root>/web/src/solutions/tag_groups.md` and build a tag-coverage checklist before editing it. Derive candidates from the fetched official `topicTags` and every reusable technique actually used by My Solution or the classic solution. For each candidate, decide whether it is a common reusable pattern or only a generic data domain/implementation detail. Register the problem under every applicable reusable pattern, not only the first matching group. If a pattern has no suitable group, add a concise English group or subgroup and continue the existing numeric sequence. Do not create tags for problem-specific details or duplicate a problem within the same pattern.
11. Verify the final Markdown structurally: exact examples retained, fences balanced, source code copied faithfully under My Solution, classic code contains no `unwrap`, relative asset links remain valid, and no unsupported correctness claim remains. Treat tag coverage as a completion gate: confirm every reusable candidate in the checklist is registered in `tag_groups.md`, every excluded candidate has an explicit reason, and tag-group numbering remains consistent.

## Acquisition Rules

- Prefer exact sources in this order: existing note statement; explicit statement supplied by the user; `python3 scripts/collect_problem.py --fetch`; a readable leetcode.nvim description buffer/export.
- Use `~/.cache/nvim/leetcode/problemlist` only for metadata such as title, slug, difficulty, and URL. It does not contain the statement.
- Do not use `~/.cache/nvim/leetcode/body` as the statement; it is the most recent test/submit payload.
- Keep remote problem images as their original LeetCode URLs. Do not download them unless asked.
- Never expose or copy the LeetCode cookie cache.

## Output Contract

- Write solution notes in English by default, regardless of the conversation language. Use another language only when the user explicitly requests that language for the note itself.
- Keep official problem content in its source language unless translation is requested.
- Order sections as problem overview, My Solution, then Classic Solution 1..N.
- Use the author's original code verbatim for My Solution. Put proposed fixes in a clearly labeled note or separate corrected snippet.
- Keep tag-group names and newly added entries in English. Preserve the existing checkbox, difficulty, numbering, and separator style of `tag_groups.md`.
- Report the note path and any unresolved uncertainty after writing.
- Do not run frontend lint/test/build merely for a Markdown note update unless the user asks.
