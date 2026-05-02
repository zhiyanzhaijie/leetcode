// @leet start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut n = l1;
        let mut m = l2;
        let mut carry = 0;

        let mut head = Box::new(ListNode::new(-1));
        let mut tail = &mut head;

        while (n.is_some() || m.is_some() || carry > 0) {
            let (v1, n_next) = match (n) {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };
            n = n_next;

            let (v2, m_next) = match (m) {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };
            m = m_next;

            let sum = v1 + v2 + carry;
            carry = sum / 10;

            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}
// @leet end
