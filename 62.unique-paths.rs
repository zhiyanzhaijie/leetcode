// @leet start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];

        for col in 0..n {
            dp[0][col] = 1;
        }

        for row in 0..m {
            dp[row][0] = 1;
        }

        for row in 1..m {
            for col in 1..n {
                dp[row][col] = dp[row - 1][col] + dp[row][col - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}
// @leet end

