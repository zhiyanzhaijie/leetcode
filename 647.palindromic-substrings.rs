// @leet start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();

        let bytes = s.as_bytes();

        let mut dp = vec![vec![false; n]; n];

        let mut res = 0;
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;

                dp[l][r] = bytes[l] == bytes[r] && (len <= 2 || dp[l + 1][r - 1]);

                if dp[l][r] {
                    res += 1;
                }
            }
        }

        res
    }
}
// @leet end

