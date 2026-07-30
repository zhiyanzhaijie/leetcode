https://leetcode.com/problems/partition-labels/

# 763. Partition Labels

Medium

Partition the string into as many parts as possible so that each letter appears in at most one part. The parts must preserve the original order, and the answer is the list of their lengths.

## Example 1

Input: `s = "ababcbacadefegdehijhklij"`

Output: `[9,7,8]`

Explanation: The partition is `"ababcbaca"`, `"defegde"`, `"hijhklij"`. This is a partition so that each letter appears in at most one part. A partition like `"ababcbacadefegde"`, `"hijhklij"` is incorrect, because it splits `s` into less parts.

## Example 2

Input: `s = "eccbbbbdec"`

Output: `[10]`

## Constraints

- `1 <= s.length <= 500`
- `s` consists of lowercase English letters.

## My Solution - Last Occurrence Map + Boundary Expansion

### Approach

First record the last index of every character in `map`. During the second scan, `[l, r]` is the current candidate partition. For each character at index `i`, its last occurrence `map[ch]` must be included, so expand `r` to `max(r, map[ch])`.

When `i > r`, the previous interval has already ended. No character in that interval appears later, so emit its length, then start a new interval at `i`. After the scan, emit the final interval.

The greedy decision is to close a partition at the earliest position where its right boundary has caught up with the scan. Closing earlier is impossible because some character in the interval still occurs later; closing later would only merge it with characters that could have formed a separate partition.

For the first official example, the boundary evolves as follows:

| Partition | Scanned range | Final `r` | Emitted length |
| --- | --- | ---: | ---: |
| 1 | `0..=8` (`ababcbaca`) | `8` | `9` |
| 2 | `9..=15` (`defegde`) | `15` | `7` |
| 3 | `16..=23` (`hijhklij`) | `23` | `8` |

### Complexity

- Time: `O(n)` expected, with `O(n)` for each of the two string scans
- Space: `O(k)` expected for `k` distinct characters in the map

The submitted code uses `map.get(&ch).unwrap()`. Under the problem constraints this lookup is guaranteed to succeed because the map was built from the same string; it would still panic if the map and scan source were ever changed independently.

```rust
use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut res = Vec::new();

        for (i, ch) in s.chars().enumerate() {
            map.insert(ch, i);
        }

        let mut l = 0usize;
        let mut r = 0usize;
        for (i, ch) in s.chars().enumerate() {
            if i > r {
                res.push((r - l + 1) as i32);
                l = i;
                r = i;
            }

            let ch_r = *map.get(&ch).unwrap();
            r = r.max(ch_r);
        }

        res.push((r - l + 1) as i32);

        res
    }
}
```

## Classic Solution - 1 - Last Occurrence Array + Greedy Partitioning

### Approach

Because the input contains only lowercase English letters, store each character's last occurrence in a fixed array of length 26 instead of a hash map. Scan the string once more while maintaining the current segment's right boundary `end`.

At index `i`, update `end` with the last occurrence of `s[i]`. The segment can be closed exactly when `i == end`: every character seen in the segment has no occurrence after `end`, so this is the earliest valid cut and maximizes the number of parts.

The boundary states for the first example are:

| Segment start | Boundary reaches | Cut when | Length |
| ---: | ---: | ---: | ---: |
| `0` | `8` | `i = 8` | `9` |
| `9` | `15` | `i = 15` | `7` |
| `16` | `23` | `i = 23` | `8` |

### Complexity

- Time: `O(n)`
- Space: `O(1)` because the alphabet size is fixed at 26

```rust
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut last = [0usize; 26];

        for (i, &byte) in bytes.iter().enumerate() {
            last[(byte - b'a') as usize] = i;
        }

        let mut result = Vec::new();
        let mut start = 0usize;
        let mut end = 0usize;

        for (i, &byte) in bytes.iter().enumerate() {
            end = end.max(last[(byte - b'a') as usize]);
            if i == end {
                result.push((end - start + 1) as i32);
                start = i + 1;
            }
        }

        result
    }
}
```
