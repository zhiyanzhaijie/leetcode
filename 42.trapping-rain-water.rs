// @leet start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();

        let mut stack: Vec<usize> = Vec::with_capacity(n);
        let mut ans = 0i32;

        for i in 0..n {
            // iter (l, mid, r)
            while let Some(&mid) = stack.last() {
                if height[i] <= height[mid] {
                    break;
                }
                stack.pop();

                let Some(&left) = stack.last() else {
                    break;
                };

                let width = (i - left - 1) as i32;
                let h = height[left].min(height[i]) - height[mid];
                if h > 0 {
                    ans += width * h;
                }
            }
            stack.push(i);
        }

        ans
    }
}
// @leet end
