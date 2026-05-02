// @leet start
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len());

        for val in nums {
            if !set.insert(val) {
                return true;
            }
        }
        false
    }
}
// @leet end

