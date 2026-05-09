// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums2.len();
        let mut mono_st: Vec<i32> = Vec::with_capacity(n);

        let mut next_map: HashMap<i32, i32> = HashMap::new();

        for num in nums2 {
            while let Some(&last) = mono_st.last() {
                if (last < num) {
                    mono_st.pop();
                    next_map.insert(last, num);
                } else {
                    break;
                }
            }
            mono_st.push(num);
        }

        nums1
            .iter()
            .map(|v| next_map.get(v).copied().unwrap_or(-1))
            .collect()
    }
}
// @leet end

