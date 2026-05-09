                                  503. Next Greater Element II
                             Medium │ 9214  242  │ 68.3% of 1.2M



Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.

The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.



󰛨 Example 1:

	│ Input: nums = [1,2,1]
	│ Output: [2,-1,2]
	│ Explanation: The first 1's next greater number is 2; 
	│ The number 2 can't find next greater number. 
	│ The second 1's next greater number needs to search circularly, which is also 2.

󰛨 Example 2:

	│ Input: nums = [1,2,3,4,3]
	│ Output: [2,3,4,-1,4]



 Constraints:

	* 1 <= nums.length <= 10^4
	
	* -10^9 <= nums[i] <= 10^9


## Classic Solution

Same in [496. Next Greater Element](../496/solution.md)

```rust
impl Solution {

    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut next: Vec<i32> = vec![-1; n];

        let mut st: Vec<usize> = Vec::with_capacity(n);

        for i in 0..2 * n {
            let cur = nums[i % n];

            while let Some(&last_idx) = st.last() {
                let last = nums[last_idx];
                if cur > last {
                    next[last_idx] = cur;
                    st.pop();
                } else {
                    break;
                }
            }
            if i < n {
                st.push(i % n);
            }
        }

        next
    }
}
```

## Solution

```rust

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut up_st: Vec<usize> = Vec::new();
        let mut down_st: Vec<usize> = Vec::new();

        let mut next: Vec<i32> = vec![-1; nums.len()];

        for (i, &num) in nums.iter().enumerate() {
            while let Some(&up_last_idx) = up_st.last() {
                let last = nums[up_last_idx];
                if (num > last) {
                    up_st.pop();

                    next[up_last_idx] = num;
                } else {
                    break;
                }
            }

            up_st.push(i);

            if let Some(&down_last_idx) = down_st.last() {
                let last = nums[down_last_idx];
                if (num > last) {
                    down_st.push(i);
                } else if num < last {
                    for j in down_st.iter() {
                        let val = nums[*j];
                        if val > num {
                            next[i] = val;
                            break;
                        }
                    }
                } else {
                }
            } else {
                down_st.push(i);
            }
        }

        next
    }
}
```
