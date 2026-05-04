// @leet start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut total = 1;
        let mut zero_idx = Vec::new();

        for (idx, &i) in nums.iter().enumerate() {
            match i {
                0 => zero_idx.push(idx),
                _ => total *= i,
            }
        }

        nums.into_iter()
            .map(|num| match zero_idx.len() {
                0 => total / num,
                1 => {
                    if num == 0 {
                        total
                    } else {
                        0
                    }
                }
                _ => 0,
            })
            .collect()
    }
}
// @leet end

