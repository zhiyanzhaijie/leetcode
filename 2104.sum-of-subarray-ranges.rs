// @leet start
impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut up_st: Vec<usize> = Vec::with_capacity(n);
        let mut down_st: Vec<usize> = Vec::with_capacity(n);

        let mut ans = 0i64;

        for i in 0..n {
            while let Some(&up_j) = up_st.last() {
                if nums[up_j] <= nums[i] {
                    break;
                } else {
                    up_st.pop();
                    let l_cnt = if let Some(&up_j_pre) = up_st.last() {
                        up_j - up_j_pre
                    } else {
                        up_j + 1
                    };

                    let r_cnt = i - up_j;

                    let up_contri = nums[up_j] as i64 * l_cnt as i64 * r_cnt as i64;
                    ans -= up_contri;
                }
            }

            while let Some(&down_j) = down_st.last() {
                if nums[down_j] > nums[i] {
                    break;
                } else {
                    down_st.pop();
                    let l_cnt = if let Some(&down_j_pre) = down_st.last() {
                        down_j - down_j_pre
                    } else {
                        down_j + 1
                    };

                    let r_cnt = i - down_j;

                    let down_contri = nums[down_j] as i64 * l_cnt as i64 * r_cnt as i64;
                    ans += down_contri;
                }
            }

            up_st.push(i);
            down_st.push(i);
        }

        while let Some(up_left) = up_st.pop() {
            let l_cnt = if let Some(&up_left_pre) = up_st.last() {
                up_left - up_left_pre
            } else {
                up_left + 1
            };

            let r_cnt = n - up_left;
            let contri = nums[up_left] as i64 * l_cnt as i64 * r_cnt as i64;
            ans -= contri;
        }

        while let Some(down_left) = down_st.pop() {
            let l_cnt = if let Some(&down_left_pre) = down_st.last() {
                down_left - down_left_pre
            } else {
                down_left + 1
            };

            let r_cnt = n - down_left;
            let contri = nums[down_left] as i64 * l_cnt as i64 * r_cnt as i64;
            ans += contri;
        }

        ans
    }
}
// @leet end
