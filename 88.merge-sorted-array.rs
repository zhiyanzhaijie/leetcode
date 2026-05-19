// @leet start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut in_cnt = 0;

        let mut i = 0usize;
        let mut k = 0usize;

        while i < (n as usize) {
            while k < (m as usize + in_cnt) && nums1[k] <= nums2[i] {
                k += 1;
            }

            nums1.insert(k, nums2[i]);
            in_cnt += 1;
            k += 1;

            i += 1;
        }

        nums1.truncate(m as usize + n as usize);
    }
}
// @leet end

