// @leet start

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seed = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let need = target - num;
            if let Some(&j) = seed.get(&need) {
                return vec![i as i32, j];
            }

            seed.insert(num, i as i32);
        }

        vec![]
    }
}
// @leet end
