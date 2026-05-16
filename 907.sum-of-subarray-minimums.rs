// @leet start
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = arr.len();
        let mut ans = 0i64;

        let mut st: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            while let Some(&pre) = st.last() {
                if arr[pre] <= arr[i] {
                    break;
                } else {
                    st.pop();

                    let l_cnt = if let Some(&ppre) = st.last() {
                        pre - ppre
                    } else {
                        pre + 1
                    };
                    let r_cnt = i - pre;
                    let contrib = arr[pre] as i64 * l_cnt as i64 * r_cnt as i64;
                    ans = (ans + contrib) % MOD;
                }
            }
            st.push(i);
        }

        while let Some(left) = st.pop() {
            let l_cnt = if let Some(&pre) = st.last() {
                left - pre
            } else {
                left + 1
            };
            let r_cnt = n - left;
            let contrib = arr[left] as i64 * l_cnt as i64 * r_cnt as i64;
            ans = (ans + contrib) % MOD;
        }

        (ans % MOD) as i32
    }
}
// @leet end
