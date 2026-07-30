https://leetcode.com/problems/assign-cookies/

# 455. Assign Cookies

Easy

Each child `i` has greed factor `g[i]`, the minimum cookie size needed to be content. Each cookie `j` has size `s[j]`, and can be assigned to at most one child. Return the maximum number of content children.

## Example 1

Input: `g = [1,2,3], s = [1,1]`

Output: `1`

Explanation: You have 3 children and 2 cookies. The greed factors of 3 children are 1, 2, 3. And even though you have 2 cookies, since their size is both 1, you could only make the child whose greed factor is 1 content. You need to output 1.

## Example 2

Input: `g = [1,2], s = [1,2,3]`

Output: `2`

Explanation: You have 2 children and 3 cookies. The greed factors of 2 children are 1, 2. You have 3 cookies and their sizes are big enough to gratify all of the children. You need to output 2.

## Constraints

- `1 <= g.length <= 3 * 10^4`
- `0 <= s.length <= 3 * 10^4`
- `1 <= g[i], s[j] <= 2^31 - 1`

## My Solution - Sorting + Greedy Scan

### Approach

Sort both arrays. For each child from the least greedy to the most greedy, advance through the remaining cookies until finding the first one large enough. Assigning that smallest feasible cookie preserves every larger cookie for children with greater requirements, so it cannot reduce the number of possible future assignments.

The cookie index only moves forward. Once a cookie is too small for the current child, it is too small for every later child as well and can be discarded. When the index reaches the end after a successful assignment, no more children can be satisfied, so the loop ends early.

### Complexity

- Time: `O(g.length log g.length + s.length log s.length)`
- Space: `O(1)` auxiliary space, excluding the sorting implementation

```rust
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut res = 0;
        let mut i = 0;
        for &child in &g {
            while i < s.len() {
                if child <= s[i] {
                    res += 1;
                    i += 1;
                    break;
                }

                i += 1;
            }

            if i == s.len() {
                break;
            }
        }

        res
    }
}
```

## Classic Solution - 1 - Sorting + Two Pointers

### Approach

After sorting, keep `child` at the least greedy unassigned child and `cookie` at the smallest unassigned cookie. A cookie smaller than `g[child]` cannot satisfy that child or any later child, so advance `cookie`. Otherwise, assign it to `child` and advance both pointers.

Using the first feasible cookie for the least greedy remaining child is greedy-optimal: replacing it with any larger feasible cookie would leave no additional option for the current child and could remove the only suitable option for a later child.

### Complexity

- Time: `O(g.length log g.length + s.length log s.length)`
- Space: `O(1)` auxiliary space, excluding the sorting implementation

```rust
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut child = 0usize;
        let mut cookie = 0usize;

        while child < g.len() && cookie < s.len() {
            if s[cookie] >= g[child] {
                child += 1;
            }
            cookie += 1;
        }

        child as i32
    }
}
```
