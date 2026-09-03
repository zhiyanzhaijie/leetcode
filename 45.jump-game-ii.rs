// @leet start
use std::i32;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp = vec![i32::MAX; n];

        dp[0] = 0;

        for i in 0..n {
            let end = (i + nums[i] as usize).min(n - 1);

            for j in (i + 1)..=end {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }

        dp[n - 1]
    }
}
// @leet end

