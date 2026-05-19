// @leet start
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        if n <= 2 {
            return n as i32;
        };

        let mut l = 0;
        let mut r = 1;

        while r < n {
            while r < n && chars[l] == chars[r] {
                r += 1;
            }

            let cnt = r - l;
            let compress_len = if cnt > 1 {
                let cnt_chars: Vec<char> = cnt.to_string().chars().collect();
                chars.drain(l + 1..r);
                for (i, &c) in cnt_chars.iter().enumerate() {
                    chars.insert(l + 1 + i, c);
                }
                cnt_chars.len()
            } else {
                0
            };

            l = r;
        }

        chars.len() as i32
    }
}
// @leet end

