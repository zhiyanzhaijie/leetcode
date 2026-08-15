// @leet start
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();

        if n == 3 {
            return values.iter().fold(1, |res, i| res * i);
        }

        let mut dp = vec![vec![i32::MAX; n]; n];
        // init
        for i in 0..n {
            dp[i][i] = 0;
            if i + 1 < n {
                dp[i][i + 1] = 0;
            }
        }

        for len in 3..=n {
            for i in 0..=n - len {
                let j = i + len - 1;

                for k in i + 1..j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[k] * values[j]);
                }
            }
        }

        dp[0][n - 1]
    }
}
// @leet end

