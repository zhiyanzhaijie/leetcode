                     https://leetcode.com/problems/move-zeroes/
                                          
                                  283. Move Zeroes
                   Easy │ 19524  607  │ 63.8% of 8.1M │ 󰛨 Hints



Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array.


󰛨 Example 1:

	│ Input: nums = [0,1,0,3,12]
	│ Output: [1,3,12,0,0]

󰛨 Example 2:

	│ Input: nums = [0]
	│ Output: [0]



 Constraints:

	* 1 <= nums.length <= 10^4
	
	* -2^31 <= nums[i] <= 2^31 - 1



Follow up: Could you minimize the total number of operations done?


## Solution - Two pointers

Tips: 
1. one pointer fast as checker, one pointer slow as target

```rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        // slow pointer
        let mut l = 0usize;

        // fast pointer
        for r in 0..n {
            // fast jump
            if nums[r] != 0 {
                if l != r {
                    nums.swap(l, r);
                }
                // slow change in condition
                l += 1;
            }
        }
    }
}

```
