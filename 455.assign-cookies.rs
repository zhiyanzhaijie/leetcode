// @leet start
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut res = 0;
        let mut i = 0;
        for &child in &g {
            while i < s.len() {
                if child <= s[i] {
                    res += 1;
                    i += 1;
                    break;
                }

                i += 1;
            }

            if i == s.len() {
                break;
            }
        }

        res
    }
}
// @leet end

