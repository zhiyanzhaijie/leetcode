use std::intrinsics::unreachable;
// @leet start
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return 0;
        }

        let mut res = 0;

        intervals.sort_by_key(|arr| arr[arr.len() - 1]);

        let mut l = *intervals.get(0).unwrap().get(0).unwrap();
        let mut r = l;

        for arr in intervals {
            let [a, b] = arr.as_slice() else {
                unreachable!("not pair vec like [l, r]");
            };
            if *a < r {
                res += 1;
            } else {
                l = *a;
                r = *b;
            }
        }

        res as i32
    }
}
// @leet end

