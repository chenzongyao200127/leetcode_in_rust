use std::collections::HashMap;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Reverse the linked list
    let mut head = Solution::reverse(head);

    // Remove nodes based on the logic
    head = Solution::remove_nodes_internal(head);

    // Reverse the list again to restore the original order
    Solution::reverse(head)
}

fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut current_node) = head.take() {
        let next = current_node.next.take();
        current_node.next = prev.take();
        prev = Some(current_node);
        head = next;
    }
    prev
}

fn remove_nodes_internal(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut cur_max = i32::MIN;
    let mut cur_node = &mut dummy;

    while let Some(ref mut next_node) = cur_node.next {
        if next_node.val >= cur_max {
            cur_max = next_node.val;
            cur_node = cur_node.next.as_mut().unwrap();
        } else {
            cur_node.next = next_node.next.take();
        }
    }

    dummy.next
}

fn main() {
    println!("Hello World!");
}
