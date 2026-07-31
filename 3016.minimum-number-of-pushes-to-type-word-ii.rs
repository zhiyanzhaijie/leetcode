// @leet start
use std::cmp::Reverse;
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut cnt_vec = vec![0; 26];
        for byte in word.bytes() {
            let i = (byte - b'a') as usize;
            cnt_vec[i] += 1;
        }

        let mut weight_vec: Vec<(usize, i32)> = cnt_vec
            .iter()
            .filter(|&&v| v > 0)
            .enumerate()
            .map(|(i, &cnt)| (i, cnt))
            .collect();

        weight_vec.sort_by_key(|&(i, cnt)| Reverse(cnt));

        let mut allow = 8;
        let mut weight = 1;

        let mut res = 0;
        for (i, mut cnt) in weight_vec {
            while cnt > 0 {
                res += weight;
                cnt -= 1;
            }

            allow -= 1;
            if allow == 0 {
                allow = 8;
                weight += 1;
            }
        }

        res
    }
}
// @leet end
