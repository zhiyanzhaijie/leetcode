// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = HashMap::new();
        let mut left = 0usize;
        let mut ans = 0usize;
        for (right, ch) in s.chars().enumerate() {
            if let Some(&prev) = last_seen.get(&ch) {
                left = left.max(prev + 1);
            }

            last_seen.insert(ch, right);
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}
// @leet end