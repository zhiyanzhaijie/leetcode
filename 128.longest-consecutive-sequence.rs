// @leet start
use std::{cmp::max, collections::HashSet};
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut res = 0;

        // set example [3 1 2 7 9 8 10]
        // 3 -> pass,
        // 1 -> 2 -> 3, 2 -> pass,
        // 7 -> 8 -> 9 -> 10, 9 -> pass, 8 -> pass, 10 -> pass

        for &n in &set {
            // Only iterate head `n`, the heads is nums `n` subjects the condition of `set.contains(&(n - 1))`
            if !set.contains(&(n - 1)) {
                let mut t = n;
                let mut len = 1;

                // loop for `n` within `n + 1`
                while set.contains(&(t + 1)) {
                    len += 1;
                    t += 1;
                }

                res = res.max(len);
            }
        }

        res
    }
}
// @leet end
