// @leet start
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();

        // dp[l][r]: the mininum steps insert for turn s[l..=r] to a palindrome string
        let mut dp = vec![vec![0; n]; n];

        for len in 2..=n {
            for l in 0..=n - len {
                let r = l + len - 1;

                if bytes[l] == bytes[r] {
                    dp[l][r] = dp[l + 1][r - 1];
                } else {
                    dp[l][r] = dp[l + 1][r].min(dp[l][r - 1]) + 1;
                }
            }
        }

        dp[0][n - 1]
    }
}
// @leet end

