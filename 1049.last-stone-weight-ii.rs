// @leet start
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let n = stones.len();

        let sum: i32 = stones.iter().copied().sum();
        let perfect = (sum / 2) as usize;

        let mut dp = vec![vec![false; perfect + 1]; n + 1];
        dp[0][0] = true;

        for i in 1..=n {
            let v = stones[i - 1] as usize;

            for j in 0..=perfect {
                dp[i][j] = dp[i - 1][j] || (j >= v && dp[i - 1][j - v]);
            }
        }

        for i in (0..=perfect).rev() {
            if dp[n][i] {
                return sum - 2 * i as i32;
            }
        }

        sum
    }
}
// @leet end
