// @leet start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        // dp[i][j]: slice within i - j is palindrome
        let mut dp = vec![vec![false; n]; n];
        let mut base = 0;
        let mut gap = 0;

        let bytes = s.as_bytes();
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;

                dp[l][r] = bytes[l] == bytes[r] && (len <= 2 || dp[l + 1][r - 1]);

                if dp[l][r] && len > gap {
                    gap = len;
                    base = l;
                }
            }
        }

        s[base..base + gap].to_string()
    }
}
// @leet end
