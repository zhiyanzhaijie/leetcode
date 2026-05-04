// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut cnt_map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            match cnt_map.get(&num) {
                Some(v) => {
                    cnt_map.insert(num, v + 1);
                }
                None => {
                    cnt_map.insert(num, 1);
                }
            }
        }

        let mut order_vec: Vec<(i32, i32)> = cnt_map.into_iter().collect();
        order_vec.sort_by_key(|&(_, v)| v);

        let mut res = Vec::new();

        for i in 0..k {
            let (a, b) = order_vec[order_vec.len() - 1 - i as usize];

            res.push(a);
        }

        res
    }
}
// @leet end

