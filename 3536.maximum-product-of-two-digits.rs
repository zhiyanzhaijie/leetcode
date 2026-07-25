// @leet start
impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut now = n;

        let mut arr: Vec<i32> = Vec::new();
        let mut ans = 0;

        loop {
            if now == 0 {
                break;
            }

            let v = now % 10;
            now = now / 10;

            arr.push(v);
        }

        let n = arr.len();

        for i in 0..n {
            for j in i + 1..n {
                ans = ans.max(arr[i] * arr[j]);
            }
        }

        ans
    }
}
// @leet end

