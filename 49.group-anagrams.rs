// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 关键思路：String的anagrams唯一性建模为[0u8; 26]

        let mut idx_map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut byte_key = [0u8; 26];

            for ch in s.bytes() {
                byte_key[(ch - b'a') as usize] += 1;
            }

            idx_map.entry(byte_key).or_default().push(s);
        }

        idx_map.into_values().collect()
    }
}
// @leet end

