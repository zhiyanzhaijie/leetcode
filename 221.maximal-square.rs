// @leet start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }

        let m = matrix[0].len();

        let mut dp = vec![vec![0; m]; n];

        let mut res = 0;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '0' {
                    continue;
                }

                if matrix[i][j] == '1' {
                    dp[i][j] = 1;
                    res = res.max(1);

                    if i == 0 || j == 0 {
                        continue;
                    }

                    if matrix[i - 1][j] == '0'
                        || matrix[i][j - 1] == '0'
                        || matrix[i - 1][j - 1] == '0'
                    {
                        continue;
                    }

                    dp[i][j] = 2.max(dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1])) + 1);

                    res = res.max(dp[i][j] * dp[i][j]);
                }
            }
        }

        res
    }
}
// @leet end

