// @leet start
use std::i32::MAX;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        };

        let mut l = 0;
        let mut r = l + 1;

        while r < n {
            while r < n && nums[l] == nums[r] {
                nums[r] = i32::MAX;
                r += 1;
            }
            l = r;
            r = l + 1;
        }

        nums.retain(|&x| x != i32::MAX);

        nums.len() as i32
    }
}
// @leet end

