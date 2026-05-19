                  https://leetcode.com/problems/3sum/
                                    
                                15. 3Sum
           Medium │ 35686  3290  │ 39.1% of 15.3M │ 󰛨 Hints


Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.


󰛨 Example 1:

	│ Input: nums = [-1,0,1,2,-1,-4]
	│ Output: [[-1,-1,2],[-1,0,1]]
	│ Explanation: 
	│ nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
	│ nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
	│ nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
	│ The distinct triplets are [-1,0,1] and [-1,-1,2].
	│ Notice that the order of the output and the order of the triplets does not matter.

󰛨 Example 2:

	│ Input: nums = [0,1,1]
	│ Output: []
	│ Explanation: The only possible triplet does not sum up to 0.

󰛨 Example 3:

	│ Input: nums = [0,0,0]
	│ Output: [[0,0,0]]
	│ Explanation: The only possible triplet sums up to 0.


 Constraints:

	* 3 <= nums.length <= 3000
	
	* -10^5 <= nums[i] <= 10^5

## Solution - Double Pointer

Tips:
- nums[i] + nums[j] + nums[k] = 0 means to nums[i] + nums[j] = (-nums[k] as target). So it's a further version of SumⅡ

```rust
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        nums.sort();

        let n = nums.len();
        for i in 0..n {
            // select one as target
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = n.saturating_sub(1);

            // same logic as (l, r) equal to (-nums[i])
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    ans.push(vec![nums[i], nums[l], nums[r]]);

                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
            }
        }

        ans
    }
}
```
