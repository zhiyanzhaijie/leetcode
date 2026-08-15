// @leet start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // dp[i]: end as nums[i]'s largest subarrary sum
        let mut dp = vec![0; n];

        for i in 0..n {
            dp[i] = nums[i];
        }

        let mut res = dp[0];

        for i in 1..n {
            if dp[i - 1] > 0 {
                dp[i] += dp[i - 1];
            }

            res = res.max(dp[i]);
        }

        res
    }
}
// @leet end

