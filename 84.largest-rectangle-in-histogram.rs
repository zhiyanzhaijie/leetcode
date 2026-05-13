// @leet start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut st: Vec<usize> = Vec::new();
        let mut max_area = 0;

        for (i, &h) in heights.iter().enumerate() {
            if i == 0 {
                st.push(i);
                continue;
            }

            while let Some(&last_idx) = st.last() {
                let v = *heights.get(last_idx).unwrap();
                if v > h {
                    st.pop();
                    let w = if let Some(&left) = st.last() {
                        i - left - 1
                    } else {
                        i
                    };
                    max_area = max_area.max(v * w as i32);
                } else {
                    break;
                }
            }

            st.push(i);
        }

        while let Some(last_idx) = st.pop() {
            let v = heights[last_idx];

            let w = if let Some(&left) = st.last() {
                heights.len() - left - 1
            } else {
                heights.len()
            };

            max_area = max_area.max(v * w as i32);
        }

        max_area
    }
}
// @leet end

