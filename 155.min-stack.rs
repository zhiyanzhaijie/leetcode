// @leet start
struct MinStack {
    stack: Vec<(i32, i32)> // v.0 is val, v.1 is the minimal for val's snapshot
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        if let Some(&(p, min_p)) = self.stack.last() {
           self.stack.push((val, val.min(min_p))); 
        } else {
           self.stack.push((val, val)); 
        } 
    }
    
    fn pop(&mut self) {
       self.stack.pop(); 
    }
    
    fn top(&self) -> i32 {
        let (v, _) = *self.stack.last().unwrap();
        v
    }
    
    fn get_min(&self) -> i32 {
       let (_, min_v) = *self.stack.last().unwrap();
        min_v
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @leet end
