// 143_Reorder_List
// https://leetcode.cn/problems/reorder-list/

// 给定一个单链表 L 的头节点 head ，单链表 L 表示为：

// L0 → L1 → … → Ln - 1 → Ln
// 请将其重新排列后变为：

// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
// 不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。


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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let len = Self::get_list_len(head.as_ref());
        if len <= 2 { return; }
        
        let (prev, next) = Self::split_list(head.take(), len);
        let reverse_next = Self::reverse(next);
        *head = Self::merge(prev, reverse_next);
    }

    fn get_list_len(mut head: Option<&Box<ListNode>>) -> usize {
        let mut len = 0;
        while let Some(node) = head.as_ref() {
            head = node.next.as_ref();
            len += 1;
        }
        len   
    }

    fn split_list(mut head: Option<Box<ListNode>>, len: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mid = (len + 1) / 2;
        let mut i = 1;
        let mut cur = head.as_mut().unwrap();

        while i < mid {
            cur = cur.next.as_mut().unwrap();
            i += 1;
        }

        let next = cur.next.take();
        (head, next)
    }

    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reverse_head = None;

        while let Some(mut node) = head {
            head = node.next.take();
            node.next = reverse_head.take();
            reverse_head = Some(node);
        }
        
        reverse_head
    }
    
    fn merge(mut head1: Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) ->Option<Box<ListNode>> {
        let mut cur = head1.as_mut().unwrap();
        while let Some(mut node) = head2 {
            head2 = node.next.take();
            node.next = cur.next.take();
            cur.next = Some(node);

            match cur.next.as_mut().unwrap().next.as_mut() {
                Some(next) => cur = next,
                None => break,
            }
        }

        head1
    }
}
