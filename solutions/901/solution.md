                                 901. Online Stock Span
                         Medium │ 7282  505  │ 69.0% of 877.6K

Design an algorithm that collects daily price quotes for some stock and returns the span of that stock's price for the current day.

The span of the stock's price in one day is the maximum number of consecutive days (starting from that day and going backward) for which the stock price was less than or equal to the price of that day.

	* For example, if the prices of the stock in the last four days is [7,2,1,2] and the price of the stock today is 2, then the span of today is 4 because starting from today, the price of the stock was less than or equal 2 for 4 consecutive days.
	
	* Also, if the prices of the stock in the last four days is [7,34,1,2] and the price of the stock today is 8, then the span of today is 3 because starting from today, the price of the stock was less than or equal 8 for 3 consecutive days.

Implement the StockSpanner class:

	* StockSpanner() Initializes the object of the class.
	
	* int next(int price) Returns the span of the stock's price given that today's price is price.



󰛨 Example 1:

	│ Input
	│ ["StockSpanner", "next", "next", "next", "next", "next", "next", "next"]
	│ [[], [100], [80], [60], [70], [60], [75], [85]]
	│ Output
	│ [null, 1, 1, 1, 2, 1, 4, 6]
	│ 
	│ Explanation
	│ StockSpanner stockSpanner = new StockSpanner();
	│ stockSpanner.next(100); // return 1
	│ stockSpanner.next(80);  // return 1
	│ stockSpanner.next(60);  // return 1
	│ stockSpanner.next(70);  // return 2
	│ stockSpanner.next(60);  // return 1
	│ stockSpanner.next(75);  // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
	│ stockSpanner.next(85);  // return 6

 Constraints:

	* 1 <= price <= 10^5
	
	* At most 10^4 calls will be made to next.

## Classic Solution(Monotonic stack) 

Tips:
- current just can reused prev
- prev shoule be a subset of current, consider use monotonic stack

```rust
struct StockSpanner {
    st: Vec<(i32, i32)>
}

impl StockSpanner {

    fn new() -> Self {
       Self {
            st: Vec::new()
        } 
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;

        while let Some(&(pre_v, pre_span)) = self.st.last() {
            if (pre_v <= price) {
                span += pre_span;
                self.st.pop();
            } else {
                break;
            }
        }

        self.st.push((price, span));

        span
    }
}
```


## Solution (Bad)

Tips:
1. if pre is smaller that current, we can jump with pre's span

```rust
struct StockSpanner {
    ans: Vec<(i32, usize, usize)>
}

impl StockSpanner {

    fn new() -> Self {
       Self {
            ans: Vec::new()
        } 
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        if let Some(&(mut pre_v, mut pre_idx, mut pre_span)) = self.ans.last() { 
            loop { 
                if pre_v <= price {
                    span += pre_span;
                } else {
                    break;
                }
                
                if pre_idx == 0 {
                    break;
                }

                if let Some(&(jump_v, jump_idx, jump_span)) = self.ans.get(pre_idx - pre_span) {
                    pre_v = jump_v;
                    pre_idx = jump_idx;
                    pre_span = jump_span;
                } else {
                    break;
                };
            }; 
        } 

        self.ans.push((price, self.ans.len(), span));

        span as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
```
