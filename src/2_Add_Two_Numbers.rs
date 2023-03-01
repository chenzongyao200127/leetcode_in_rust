// https://leetcode.cn/problems/add-two-numbers/

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        carried(l1, l2, 0)
    }
}

pub fn carried(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| {carry += x.val; x.next}), 
                l2.and_then(|x| {carry += x.val; x.next}), 
                carry / 10
            ),
            val: carry % 10
        }))
    }
}
