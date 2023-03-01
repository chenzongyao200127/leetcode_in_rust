// 61. Rotate List
// https://leetcode.cn/problems/rotate-list/

// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}


impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }

        let mut head = head;
        let mut ptr = &head;
        let mut len = 0;
        while let Some(ref t) = ptr {
            ptr = &t.next;
            len += 1;
        }

        let k = k % len;
        if k == 0 {
            return head;
        }

        let mut ptr = &mut head;

        for _ in 1..len - k {
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        let mut newhead = ptr.as_mut().unwrap().next.take();
        let mut tail = &mut newhead;
        while tail.is_some() &&  tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = head;
        newhead
    }
}