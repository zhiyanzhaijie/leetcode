// @leet start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&v| v != val);
        nums.len() as i32
    }
}
// @leet end

