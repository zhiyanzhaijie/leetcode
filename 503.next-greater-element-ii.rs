// @leet start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut next: Vec<i32> = vec![-1; n];

        let mut st: Vec<usize> = Vec::with_capacity(n);

        for i in 0..2 * n {
            let cur = nums[i % n];

            while let Some(&last_idx) = st.last() {
                let last = nums[last_idx];
                if cur > last {
                    next[last_idx] = cur;
                    st.pop();
                } else {
                    break;
                }
            }
            if i < n {
                st.push(i % n);
            }
        }

        next
    }
}
// @leet end
