// @leet start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut cur_q = VecDeque::new();

        if let Some(node) = &root {
            cur_q.push_back(node.clone());
        }

        let mut expect: HashMap<i32, usize> = HashMap::new();
        let mut seen: HashMap<i32, bool> = HashMap::new();

        while !cur_q.is_empty() {
            let mut level = Vec::new();

            for _ in 0..cur_q.len() {
                if let Some(node) = cur_q.pop_front() {
                    let n = node.borrow();

                    level.push(n.val);

                    if let Some(l) = &n.left {
                        cur_q.push_back(l.clone());
                    }

                    if let Some(r) = &n.right {
                        cur_q.push_back(r.clone());
                    }
                }
            }

            // calculate level
            let mut right_level = level.clone();
            right_level.sort();

            for (i, &v) in right_level.iter().enumerate() {
                expect.insert(v, i);
            }

            for i in 0..level.len() {
                while level[i] != right_level[i] {
                    let target_i = *expect.get(&level[i]).unwrap();
                    level.swap(i, target_i);
                    res += 1;
                }
            }
        }

        res
    }
}
// @leet end

