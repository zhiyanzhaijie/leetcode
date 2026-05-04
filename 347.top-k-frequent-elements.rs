// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut cnt_map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            match cnt_map.get_mut(&num) {
                Some(v) => *v += 1,
                None => {
                    cnt_map.insert(num, 1);
                }
            }
        }

        let mut order_vec: Vec<(i32, i32)> = cnt_map.into_iter().collect();
        order_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        order_vec
            .into_iter()
            .take(k as usize)
            .map(|(num, _)| num)
            .collect()
    }
}
// @leet end
