// @leet start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        // match -> clear, unmatch -> push
        // empty(valid) - noempty(invalid)
        let mut vec_ch: Vec<char> = vec![];

        for ch in s.chars() {
            match vec_ch.last() {
                Some(&last) => match (last, ch) {
                    ('(', ')') | ('{', '}') | ('[', ']') => {
                        vec_ch.pop();
                    }
                    _ => vec_ch.push(ch),
                },
                None => {
                    vec_ch.push(ch);
                }
            }
        }

        vec_ch.is_empty()
    }
}
// @leet end

