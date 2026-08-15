// @leet start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = nums.clone();
        let mut n_dp = nums.clone();

        let mut res = dp[0];

        for i in 1..n {
            let v = nums[i] * dp[i - 1];
            let n_v = nums[i] * n_dp[i - 1];

            dp[i] = nums[i].max(v).max(n_v);
            n_dp[i] = nums[i].min(v).min(n_v);

            res = res.max(dp[i]);
        }

        res
    }
}
// @leet end
