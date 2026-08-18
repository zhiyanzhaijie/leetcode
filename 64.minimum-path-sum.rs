// @leet start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // dp[i][j] means the minimun path sum till point(i, j)

        let m = grid.len();
        if m < 1 {
            return 0;
        }

        let n = grid[0].len();

        let mut dp = vec![vec![0; n]; m];

        for row in 0..m {
            for col in 0..n {
                if row == 0 && col == 0 {
                    dp[row][col] = grid[row][col];
                    continue;
                }

                if row == 0 {
                    dp[row][col] = dp[row][col - 1] + grid[row][col];
                    continue;
                }
                if col == 0 {
                    dp[row][col] = dp[row - 1][col] + grid[row][col];
                    continue;
                }

                dp[row][col] = dp[row - 1][col].min(dp[row][col - 1]) + grid[row][col];
            }
        }

        dp[m - 1][n - 1]
    }
}
// @leet end

