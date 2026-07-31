// @leet start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let sum: i32 = nums.iter().copied().sum();
        let target = (sum / 2) as usize;
        if sum % 2 > 0 {
            return false;
        }

        // dp[i][j]: for range(0 - i), we can get sum equal to `j`
        let mut dp = vec![vec![false; target + 1]; n];
        dp[0][0] = true;

        for i in 1..n {
            let v = nums[i - 1] as usize;

            for j in 0..=target {
                dp[i][j] = dp[i - 1][j] || (j >= v && dp[i - 1][j - v]);
            }
        }

        dp[n - 1][target]
    }
}
// @leet end

