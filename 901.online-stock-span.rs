// @leet start
struct StockSpanner {
    st: Vec<(i32, i32)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
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

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
// @leet end
