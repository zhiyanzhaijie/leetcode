// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();

        let mut arr: Vec<(i32, i32)> = position
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, speed[i]))
            .collect();

        arr.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut st: Vec<(i32, i32)> = Vec::new();

        for (posi, speed) in arr {
            if let Some(&(pre_posi, pre_speed)) = st.last() {
                let cur_cost = (target - posi) as f64 / speed as f64;
                let pre_cost = (target - pre_posi) as f64 / pre_speed as f64;

                if cur_cost <= pre_cost {
                    continue;
                } else {
                    st.push((posi, speed));
                }
            } else {
                st.push((posi, speed));
            }
        }

        st.len() as i32
    }
}
// @leet end
