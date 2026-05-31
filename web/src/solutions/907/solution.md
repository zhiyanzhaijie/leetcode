
          https://leetcode.com/problems/sum-of-subarray-minimums/
                                     
                       907. Sum of Subarray Minimums
                   Medium │ 9266  741  │ 38.6% of 1.2M



Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.



󰛨 Example 1:

	│ Input: arr = [3,1,2,4]
	│ Output: 17
	│ Explanation: 
	│ Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4]. 
	│ Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
	│ Sum is 17.

󰛨 Example 2:

	│ Input: arr = [11,81,94,43,3]
	│ Output: 444



 Constraints:

	* 1 <= arr.length <= 3 * 10^4
	
	* 1 <= arr[i] <= 3 * 10^4

## Solution - monotonic stack

Tips:
1. find the large boundry of arr[i] as miniinum (L, i, R)
2. count of i will be `(L, i]` * `[i, R]`, a math play

```rust

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = arr.len();
        let mut ans = 0i64;

        let mut st: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            while let Some(&pre) = st.last() {
                if arr[pre] <= arr[i] {
                    break;
                } else {
                    st.pop();

                    let l_cnt = if let Some(&ppre) = st.last() {
                        pre - ppre
                    } else {
                        pre + 1
                    };
                    let r_cnt = i - pre;
                    let contrib = arr[pre] as i64 * l_cnt as i64 * r_cnt as i64;
                    ans = (ans + contrib) % MOD;
                }
            }
            st.push(i);
        }

        while let Some(left) = st.pop() {
            let l_cnt = if let Some(&pre) = st.last() {
                left - pre
            } else {
                left + 1
            };
            let r_cnt = n - left;
            let contrib = arr[left] as i64 * l_cnt as i64 * r_cnt as i64;
            ans = (ans + contrib) % MOD;
        }

        (ans % MOD) as i32
    }
}

```
