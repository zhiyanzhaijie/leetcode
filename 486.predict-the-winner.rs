// @leet start
use std::cmp::max;
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            dp[i][i] = nums[i];
        }

        for len in 2..=n {
            for l in 0..=n - len {
                let r = l + len - 1;

                let take_l = nums[l] - dp[l + 1][r];
                let take_r = nums[r] - dp[l][r - 1];

                dp[l][r] = max(take_l, take_r);
            }
        }

        dp[0][n - 1] >= 0
    }
}
// @leet end

