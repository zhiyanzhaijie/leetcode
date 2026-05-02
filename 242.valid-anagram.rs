// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut cnt = [0i32; 26];

        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        for b in t.bytes() {
            cnt[(b - b'a') as usize] -= 1;
        }

        cnt.iter().all(|&v| v == 0)
    }
}
// @leet end

