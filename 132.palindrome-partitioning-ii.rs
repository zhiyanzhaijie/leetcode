use std::cmp::Reverse;

// @leet start
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();

        let mut dp = vec![vec![false; n]; n];

        for i in 0..n {
            dp[i][i] = true;
        }

        for len in 2..=n {
            for l in 0..=n - len {
                let r = l + len - 1;

                dp[l][r] = bytes[l] == bytes[r] && (len <= 2 || dp[l + 1][r - 1]);
            }
        }

        // p_dp[i] means (0..=i) mininum parts divide needed
        let mut p_dp = vec![n - 1; n];
        p_dp[0] = 0;

        for r in 1..n {
            for l in 0..=r {
                if dp[l][r] {
                    let parts = if l == 0 { 0 } else { p_dp[l - 1] + 1 };
                    p_dp[r] = p_dp[r].min(parts);
                }
            }
        }

        p_dp[n - 1] as i32
    }
}
// @leet end

