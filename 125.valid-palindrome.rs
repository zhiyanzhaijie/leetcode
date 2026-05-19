// @leet start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .into_iter()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        let (mut l, mut r) = (0usize, chars.len().saturating_sub(1));

        while l < r {
            let left = chars[l];
            let right = chars[r];
            if left.to_ascii_lowercase() != right.to_ascii_lowercase() {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }
}
// @leet end

