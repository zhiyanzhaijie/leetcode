// @leet start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut l = 0usize;

        for r in 0..n {
            if nums[r] != 0 {
                if l != r {
                    nums.swap(l, r);
                }
                l += 1;
            }
        }
    }
}
// @leet end

