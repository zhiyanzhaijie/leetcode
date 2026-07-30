https://leetcode.com/problems/jump-game/

# 55. Jump Game

Medium

You are initially at the first index of `nums`, where `nums[i]` is the maximum jump length from index `i`. Return `true` if the last index is reachable; otherwise, return `false`.

## Example 1

Input: `nums = [2,3,1,1,4]`

Output: `true`

Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

## Example 2

Input: `nums = [3,2,1,0,4]`

Output: `false`

Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

## Constraints

- `1 <= nums.length <= 10^4`
- `0 <= nums[i] <= 10^5`

## My Solution - Forward Greedy Reachability

### Approach

`max_step` is the farthest index reachable using positions scanned so far. Before processing index `i`, `i <= max_step` means that there is a valid sequence of jumps to `i`. Extending the reachable boundary with `i + nums[i]` therefore considers every useful jump from a reachable position.

If `i > max_step`, index `i` is unreachable. Since every later index is even farther away, no future position can repair that gap, so return `false` immediately. Otherwise, after the scan, reaching or passing the last index proves the answer is `true`.

The solution keeps only the envelope of all reachable choices instead of storing an exact jump path:

| Processed index | `nums[i]` | `max_step` after processing | Meaning |
| --- | ---: | ---: | --- |
| Start | - | `0` | Index `0` is the initial reachable position. |
| `0` | `2` | `2` | Indices `0..=2` can be reached. |
| `1` | `3` | `4` | The reachable boundary expands to the last index. |

The decision at each index is:

- `i > max_step`: the current index is unreachable, so return `false`.
- `i <= max_step`: extend the boundary with `max(i + nums[i])`.

### Complexity

- Time: `O(n)`
- Space: `O(1)`

```rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut max_step = 0;

        for i in 0..n - 1 {
            if i > max_step {
                return false;
            }

            max_step = max_step.max(i + nums[i] as usize);
        }

        max_step >= n - 1
    }
}
```

## Classic Solution - 1 - Backward Greedy Reachability

### Approach

Start with the last index as the current `goal`. Scan backward: if index `i` can jump to `goal` or beyond, then `i` itself becomes the new goal. The invariant is that the current `goal` is the leftmost position known to reach the last index.

When the scan ends, the last index is reachable exactly when the goal has moved back to index `0`. This is the same greedy property as the forward solution, viewed from the destination instead of the start.

For `nums = [2,3,1,1,4]`, the goal moves left whenever the current index can reach it:

`goal = 4` -> `i = 3` reaches 4, so `goal = 3` -> `i = 2` reaches 3, so `goal = 2` -> `i = 1` reaches 2, so `goal = 1` -> `i = 0` reaches 1, so `goal = 0`.

Reaching `goal = 0` means the original starting position can reach the destination.

### Complexity

- Time: `O(n)`
- Space: `O(1)`

```rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut goal = nums.len() - 1;

        for i in (0..nums.len()).rev() {
            if i + nums[i] as usize >= goal {
                goal = i;
            }
        }

        goal == 0
    }
}
```
