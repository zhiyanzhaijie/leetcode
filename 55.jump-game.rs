// @leet start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut max_step = 0;

        for i in 0..n - 1 {
            if i > max_step {
                return false;
            }

            max_step = max_step.max(i + nums[i] as usize);
        }

        max_step >= n - 1
    }
}
// @leet end

