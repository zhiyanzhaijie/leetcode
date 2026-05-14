// @leet start
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        // sentry: add it just for reduce code
        heights.push(0);

        let mut st: Vec<usize> = Vec::with_capacity(heights.len());
        let mut ans = 0;

        for i in 0..heights.len() {
            while let Some(&mid) = st.last() {
                if heights[mid] <= heights[i] {
                    break;
                }

                st.pop();
                let width = if let Some(&left) = st.last() {
                    i - left - 1
                } else {
                    i
                };

                ans = ans.max(heights[mid] * width as i32);
            }

            st.push(i);
        }

        ans
    }
}
// @leet end

