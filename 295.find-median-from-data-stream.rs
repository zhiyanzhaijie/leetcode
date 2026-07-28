
// @leet start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.left.push(num);
        if let Some(max_left) = self.left.pop() {
            self.right.push(Reverse(max_left));
        }
        if self.left.len() < self.right.len() {
            if let Some(Reverse(min_right)) = self.right.pop() {
                self.left.push(min_right);
            }
        }
    }

    fn find_median(&self) -> f64 {
        match (self.left.peek(), self.right.peek()) {
            (Some(&l), Some(&Reverse(r))) => {
                if self.left.len() > self.right.len() {
                    l as f64
                } else {
                    (l as f64 + r as f64) / 2.0
                }
            }
            (Some(&l), None) => l as f64,
            _ => 0.0,
        }
    }
}

// Your MedianFinder object will be instantiated and called as such:
// let obj = MedianFinder::new();
// obj.add_num(num);
// let ret_2: f64 = obj.find_median();
// @leet end
