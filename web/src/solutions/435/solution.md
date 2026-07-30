https://leetcode.com/problems/non-overlapping-intervals/

# 435. Non-overlapping Intervals

Medium

Given an array of intervals, return the minimum number of intervals to remove so that the remaining intervals are non-overlapping. Intervals that only touch at a point are non-overlapping.

## Example 1

Input: `intervals = [[1,2],[2,3],[3,4],[1,3]]`

Output: `1`

Explanation: `[1,3]` can be removed and the rest of the intervals are non-overlapping.

## Example 2

Input: `intervals = [[1,2],[1,2],[1,2]]`

Output: `2`

Explanation: You need to remove two `[1,2]` to make the rest of the intervals non-overlapping.

## Example 3

Input: `intervals = [[1,2],[2,3]]`

Output: `0`

Explanation: You don't need to remove any of the intervals since they're already non-overlapping.

## Constraints

- `1 <= intervals.length <= 10^5`
- `intervals[i].length == 2`
- `-5 * 10^4 <= start_i < end_i <= 5 * 10^4`

## My Solution - Earliest End Greedy Scan

### Approach

Sort intervals by their right endpoint. `r` is the right endpoint of the most recently retained interval. For each sorted interval `[a, b]`:

- If `a < r`, it overlaps the retained interval, so remove the current interval and increment `res`.
- Otherwise, it is compatible, so retain it and update `r = b`.

The greedy invariant is that the retained interval always has the smallest possible ending point among the choices considered so far. When two intervals overlap, the one ending earlier leaves at least as much room for every future interval, so removing the later-ending interval cannot reduce the maximum number of retained intervals.

For the first example, sorting by right endpoint gives the following decisions:

| Interval | Current `r` | Decision | New `r` |
| --- | ---: | --- | ---: |
| `[1,2]` | `1` | retain | `2` |
| `[2,3]` | `2` | retain, touching is allowed | `3` |
| `[1,3]` | `3` | remove, because `1 < 3` | `3` |
| `[3,4]` | `3` | retain | `4` |

`l` is updated alongside `r` when an interval is retained, but it is not used by the final count; it records the left endpoint of the retained interval without changing the greedy decision.

### Complexity

- Time: `O(n log n)` for sorting and `O(n)` for the scan
- Space: `O(1)` auxiliary space, excluding the sorting implementation

The two `unwrap` calls are safe under the problem constraints: after the length check, the first interval exists, and every interval has two elements. The explicit `std::intrinsics::unreachable` import is unused; the source code below is preserved exactly as submitted.

```rust
use std::intrinsics::unreachable;
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return 0;
        }

        let mut res = 0;

        intervals.sort_by_key(|arr| arr[arr.len() - 1]);

        let mut l = *intervals.get(0).unwrap().get(0).unwrap();
        let mut r = l;

        for arr in intervals {
            let [a, b] = arr.as_slice() else {
                unreachable!("not pair vec like [l, r]");
            };
            if *a < r {
                res += 1;
            } else {
                l = *a;
                r = *b;
            }
        }

        res as i32
    }
}
```

## Classic Solution - 1 - Interval Scheduling by Earliest Finish Time

### Approach

The equivalent goal is to maximize the number of intervals kept. Sort by ending position and keep the first interval, which finishes earliest. For every later interval, keep it only when its start is at least the end of the last kept interval; otherwise remove it.

The earliest-finish invariant is optimal because a retained interval with a smaller end leaves a superset of the room available after a retained interval with a larger end. Therefore, in every overlap, keeping the earlier-ending interval is never worse for the remaining scan.

The same first example becomes a short state sequence:

`keep [1,2]` -> `keep [2,3]` -> `remove [1,3]` -> `keep [3,4]` -> remove count `1`.

### Complexity

- Time: `O(n log n)`
- Space: `O(1)` auxiliary space, excluding the sorting implementation

```rust
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return 0;
        }

        intervals.sort_unstable_by_key(|interval| interval[1]);

        let mut right = intervals[0][1];
        let mut removed = 0;

        for interval in intervals.into_iter().skip(1) {
            if interval[0] < right {
                removed += 1;
            } else {
                right = interval[1];
            }
        }

        removed
    }
}
```
