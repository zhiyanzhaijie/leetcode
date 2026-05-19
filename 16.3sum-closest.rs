// @leet start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let n = nums.len();

        if n < 3 {
            return 0;
        }

        let mut best = nums[0] + nums[1] + nums[2];

        for i in 0..(n - 2) {
            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (best - target).abs() {
                    best = sum;
                }

                if sum > target {
                    while l < r && nums[r - 1] == nums[r] {
                        r -= 1;
                    }

                    r -= 1;
                } else if sum < target {
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }

                    l += 1;
                } else {
                    return target;
                }
            }
        }

        best
    }
}
// @leet end

