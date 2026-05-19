// @leet start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0usize;
        let mut r = height.len().saturating_sub(1);
        let mut ans = 0;

        while l < r {
            let w = (r - l) as i32;
            let h = height[l].min(height[r]);

            ans = ans.max(w * h);

            // the key is always move the lower one, since moving the higher one make a smaller situation
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        ans
    }
}
// @leet end

