// @leet start
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if n < 4 {
            return ans;
        }

        nums.sort();

        for i in 0..(n - 3) {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            for j in (i + 1)..(n - 2) {
                if j > i + 1 && nums[j - 1] == nums[j] {
                    continue;
                }

                let mut l = j + 1;
                let mut r = n - 1;

                while l < r {
                    let t = target as i64;
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;

                    if sum < t {
                        l += 1;
                    } else if sum > t {
                        r -= 1;
                    } else {
                        ans.push(vec![nums[i], nums[j], nums[l], nums[r]]);

                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r - 1] == nums[r] {
                            r -= 1;
                        }

                        l += 1;
                        r -= 1;
                    }
                }
            }
        }

        ans
    }
}
// @leet end

