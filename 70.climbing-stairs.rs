// @leet start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize];

        if n <= 1 {
            return n;
        }

        dp[0] = 1;
        dp[1] = 2;

        for i in 2..n as usize {
            dp[i] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
        }

        dp[(n - 1) as usize]
    }
}
// @leet end
