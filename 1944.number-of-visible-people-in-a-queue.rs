// @leet start
impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut st: Vec<usize> = Vec::with_capacity(n);
        let mut gap: Vec<usize> = vec![0; n];
        let mut ans: Vec<i32> = vec![0; n];

        for i in 0..n {
            while let Some(&last) = st.last() {
                if heights[last] < heights[i] {
                    let mut seen = 0i32;
                    let mut cur = last;

                    while cur < i {
                        cur += gap[cur].max(1);
                        seen += 1;
                    }

                    let dist = i - last;
                    gap[last] = dist;
                    ans[last] = seen;
                    st.pop();
                } else {
                    break;
                }
            }

            st.push(i);
        }

        st.reverse();
        while let Some(left) = st.pop() {
            if let Some(&next) = st.last() {
                let mut seen = 0i32;
                let mut cur = left;

                while cur < next {
                    cur += gap[cur].max(1);
                    seen += 1;
                }

                let dist = next - left;
                gap[left] = dist;
                ans[left] = seen;
            } else {
                ans[left] = 0;
            }
        }

        ans
    }
}
// @leet end
