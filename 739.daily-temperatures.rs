// @leet start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut ans = vec![0; n];
        let mut st: Vec<usize> = Vec::with_capacity(n);

        for i in (0..n).rev() {
            while let Some(&j) = st.last() {
                if temperatures[j] <= temperatures[i] {
                    st.pop();
                } else {
                    break;
                }
            }
            if let Some(&j) = st.last() {
                ans[i] = (j - i) as i32;
            }
            st.push(i);
        }
        ans
    }
}
// @leet end
