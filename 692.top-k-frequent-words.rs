// @leet start
use std::{cmp::Reverse, collections::HashMap};
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut cnt_map: HashMap<String, u8> = HashMap::new();

        for s in words {
            match cnt_map.get_mut(&s) {
                Some(v) => *v += 1,
                None => {
                    cnt_map.insert(s, 1);
                }
            }
        }

        let mut order_map: Vec<(String, u8)> = cnt_map.into_iter().collect();
        order_map.sort_by_key(|(k, _)| k.clone());
        order_map.sort_by_key(|&(_, v)| Reverse(v));

        order_map
            .into_iter()
            .take(k as usize)
            .map(|a| a.0)
            .collect()
    }
}
// @leet end

