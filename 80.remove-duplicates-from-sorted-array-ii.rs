// @leet start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();

        let mut l = 0;
        let mut r = l + 1;

        while r < n {
            let mut remains = 1;
            while r < n && nums[l] == nums[r] {
                if remains > 0 {
                    remains -= 1;
                } else {
                    nums[r] = i32::MAX;
                }

                r += 1;
            }

            l = r;
            r = l + 1;
        }

        nums.retain(|&v| v != i32::MAX);

        nums.len() as i32
    }
}
// @leet end

