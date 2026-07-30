// @leet start
use std::collections::HashMap;
impl Solution {
    fn combination(n: i64, r: i64, limit: i64) -> i64 {
        let r = r.min(n - r);
        let mut result = 1_i64;

        for i in 1..=r {
            result = result * (n - i + 1) / i;
            if result >= limit {
                return limit;
            }
        }

        result
    }
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut pre_res = String::new();

        let n = s.len() / 2;

        let bytes = s.as_bytes();
        let mid = if s.len() % 2 > 0 {
            (bytes[s.len() / 2] as char).to_string()
        } else {
            "".to_string()
        };

        let mut cnt_vec = vec![0_i64; 26];
        for byte in s.bytes() {
            let i = (byte - b'a') as usize;
            cnt_vec[i] += 1;
        }

        for i in 0..26 {
            cnt_vec[i] /= 2;
        }

        let mut cur_i = 0;
        let mut base = 0_i64;
        let target = i64::from(k);
        let mut ok = false;
        while (cur_i != n) {
            ok = false;
            for i in 0..26 {
                if cnt_vec[i] <= 0 {
                    continue;
                }

                let cur: char = (b'a' + i as u8) as char;
                pre_res.push(cur);
                cnt_vec[i] -= 1;
                let mut total: i64 = cnt_vec.iter().sum();
                let gap = cnt_vec
                    .iter()
                    .filter(|&&value| value > 0)
                    .map(|&value| {
                        let combination = Self::combination(total, value, target);
                        total -= value;
                        combination
                    })
                    .reduce(|acc, value| {
                        if acc >= target || value >= target {
                            target
                        } else {
                            (acc * value).min(target)
                        }
                    })
                    .map_or(1, |value| value);
                if base + gap < target {
                    // this char way is wrong in this posi
                    pre_res.pop();
                    cnt_vec[i] += 1;
                    base += gap;
                } else {
                    // right char in this posi
                    cur_i += 1;
                    ok = true;
                    break;
                }
            }

            if !ok {
                return String::new();
            }
        }

        let next_res: String = pre_res.clone().chars().rev().collect();
        format!("{pre_res}{mid}{next_res}")
    }
}
// @leet end
