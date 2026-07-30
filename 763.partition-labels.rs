// @leet start
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
// @leet end

