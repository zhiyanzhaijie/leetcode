https://leetcode.com/problems/partition-equal-subset-sum/

# 416. Partition Equal Subset Sum

Medium

Return `true` if the array can be partitioned into two subsets with equal sums; otherwise, return `false`.

## Example 1

Input: `nums = [1,5,11,5]`

Output: `true`

Explanation: The array can be partitioned as `[1, 5, 5]` and `[11]`.

## Example 2

Input: `nums = [1,2,3,5]`

Output: `false`

Explanation: The array cannot be partitioned into equal sum subsets.

## Constraints

- `1 <= nums.length <= 200`
- `1 <= nums[i] <= 100`

## My Solution - Two-Dimensional 0/1 Knapsack

### Approach

Let `sum` be the total. An odd total cannot be split equally. Otherwise, finding an equal partition is equivalent to finding a subset whose sum is exactly `target = sum / 2`; the unselected numbers automatically form the other subset.

Each row records which sums are achievable after considering a prefix. The transition for current weight `v` and target sum `j` is:

```text
dp[i][j] = dp[i - 1][j] OR (j >= v AND dp[i - 1][j - v])
             do not take v             take v once
```

For `nums = [1,5,11,5]` and `target = 11`, the meaningful reachable sums are:

| Row | Newly considered value | Reachable sums up to `11` |
| --- | ---: | --- |
| `dp[0]` | none | `{0}` |
| `dp[1]` | `1` | `{0,1}` |
| `dp[2]` | `5` | `{0,1,5,6}` |
| `dp[3]` | `11` | `{0,1,5,6,11}` |

Once `11` is reachable, the answer is `true`. The transition reads only the previous row, so a value can contribute at most once.

This submission uses `n` rows, with `dp[0]` representing the empty prefix, and processes `nums[0..n - 1)`. It does not explicitly process the last number. The result is still correct for this exact equal-partition problem: if a valid subset of sum `target` contains the final number, its complement has the same sum and excludes that number. This relies on the complement symmetry of equal partitioning; the conventional general 0/1-knapsack layout uses `n + 1` rows and processes every input value.

### Complexity

- Time: `O(n * target)`
- Space: `O(n * target)`

```rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let sum: i32 = nums.iter().copied().sum();
        let target = (sum / 2) as usize;
        if sum % 2 > 0 {
            return false;
        }

        // dp[i][j]: for range(0 - i), we can get sum equal to `j`
        let mut dp = vec![vec![false; target + 1]; n];
        dp[0][0] = true;

        for i in 1..n {
            let v = nums[i - 1] as usize;

            for j in 0..=target {
                dp[i][j] = dp[i - 1][j] || (j >= v && dp[i - 1][j - v]);
            }
        }

        dp[n - 1][target]
    }
}
```

## Classic Solution - 1 - One-Dimensional 0/1 Knapsack

### Approach

`dp[j]` means that some subset of the values processed so far sums exactly to `j`. For each value `v`, either leave `dp[j]` unchanged or set it from the previous ability to reach `j - v`.

Iterate `j` from `target` down to `v`. Descending order ensures `dp[j - v]` belongs to the state before taking the current value, so the current value cannot be reused. Ascending order would allow it to update itself repeatedly and would model an unbounded knapsack instead.

For the first example, the state set grows as follows:

| Processed value | New relevant reachable sums |
| ---: | --- |
| start | `{0}` |
| `1` | `{0,1}` |
| `5` | `{0,1,5,6}` |
| `11` | `{0,1,5,6,11}` -> `target` reached |

### Complexity

- Time: `O(n * target)`
- Space: `O(target)`

```rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().copied().sum();
        if sum % 2 != 0 {
            return false;
        }

        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for value in nums {
            let weight = value as usize;

            for capacity in (weight..=target).rev() {
                dp[capacity] = dp[capacity] || dp[capacity - weight];
            }
        }

        dp[target]
    }
}
```
