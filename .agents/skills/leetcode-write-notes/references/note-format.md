# Note Format

## Target

Write to:

```text
<repo-root>/web/src/solutions/<frontend-id>/solution.md
```

Follow nearby notes when they establish a stronger local convention. Use this baseline for new notes:

```markdown
https://leetcode.com/problems/<slug>/

# <id>. <title>

<difficulty>

<short exact problem summary>

## Example 1

Input: ...
Output: ...
Explanation: ...

## Example 2

...

## Constraints

- ...

## My Solution - <approach name>

### Approach

<Explain the author's actual reasoning and invariant.>

### Complexity

- Time: `O(...)`
- Space: `O(...)`

```rust
<verbatim code from the solution file>
```

## Classic Solution - 1 - <canonical approach name>

### Approach

<Explain why the data structure/algorithm fits and how it proceeds.>

### Complexity

- Time: `O(...)`
- Space: `O(...)`

```mermaid
<optional visualization>
```

```rust
<fresh canonical implementation>
```
```

## Problem Content

- Copy the title, summary, examples, explanations, and constraints exactly enough to preserve semantics and values.
- Remove volatile popularity counters and decorative terminal glyphs unless the surrounding notes consistently require them.
- Prefer a concise overview over copying the entire statement, but never paraphrase example inputs/outputs or constraints.
- Preserve official image URLs using `[img](https://...)` or the neighboring note's syntax.

## Solution Labels

- The first submitted implementation is always `My Solution`, even when it resembles a canonical approach.
- Use `Classic Solution - 1`, `Classic Solution - 2`, and so on only for distinct classic approaches.
- Default to one classic solution. Do not pad the note with minor implementation variants.
- Name approaches by the core reusable concept: `BFS + permutation cycles`, `monotonic stack`, `two pointers`, `dynamic programming`, and similar.

## Explanation Standard

For each solution, cover:

1. What subproblem is solved at each step.
2. The invariant or reason the greedy/data-structure operation is correct.
3. Why the chosen data structure is appropriate.
4. Time and auxiliary-space complexity, including sorting or recursion costs.
5. For My Solution only, any concrete bug, unused state, panic risk, stale state, or hidden constraint dependency found in the code.

Do not turn the note into a line-by-line code narration.

## Mermaid Selection

Add Mermaid only when a diagram improves understanding. Good candidates:

- tree/graph traversal and the order nodes enter a queue;
- permutation cycles and swaps;
- monotonic stack push/pop transitions;
- dynamic-programming state dependencies;
- pointer movement across several states.

Choose the smallest fitting diagram:

- `flowchart` for algorithm phases or decisions;
- `graph` for trees, graphs, and cycle structure;
- `sequenceDiagram` only for interactions over time;
- a compact table in Markdown when exact mappings matter more than geometry.

Use one official example where possible. Keep labels short, quote labels containing punctuation, avoid renderer-specific styling, and ensure the diagram compiles conceptually. Mermaid is a planning artifact for later SVG drawing, so emphasize the algorithmic state rather than decoration.

## Rust Rules

- Never use `unwrap` in generated classic code. Use pattern matching, `if let`, `let ... else`, indexing only where proven safe, or error-aware access as appropriate.
- Prefer imports such as `use std::collections::VecDeque;` over fully qualified paths repeated in code.
- Do not add crates unavailable on LeetCode.
- Preserve required commented data-structure definitions only when they help the note; avoid duplicating boilerplate in every solution.
- Do not edit the original Rust submission as part of note generation unless explicitly asked.

## Existing Notes

- Read the complete existing `solution.md` before editing.
- Preserve all manual SVG references and files.
- Do not reorder existing solutions solely to match this template.
- Merge by semantic section, not brittle text replacement.
- If the existing My Solution code differs from the current Rust file, flag the discrepancy instead of overwriting either version silently.

## Tag Groups

- After writing a solution, inspect `web/src/solutions/tag_groups.md` and register the problem under each clearly applicable, reusable algorithmic pattern.
- Prefer an existing subgroup. Add a new group or subgroup only when the technique is a common problem category rather than a detail unique to one solution.
- Continue the document's existing top-level and subgroup numbering. Keep new names and entries in English.
- Follow the local entry format, including completion checkbox, difficulty, problem ID, and official title. Do not add duplicate entries within the same subgroup.
