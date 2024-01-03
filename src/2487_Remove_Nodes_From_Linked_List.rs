// 2487_Remove_Nodes_From_Linked_List
// https://leetcode.cn/problems/remove-nodes-from-linked-list/description/

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

impl Solution {
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
}

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let reversed_head = Solution::reverse_recursive(head);
        let filtered_head = Solution::remove_nodes_recursive(reversed_head, i32::MIN);
        Solution::reverse_recursive(filtered_head)
    }

    fn reverse_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::reverse_recursive_helper(head, None)
    }

    fn reverse_recursive_helper(
        current: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match current {
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                Solution::reverse_recursive_helper(next, Some(node))
            }
            None => prev,
        }
    }

    fn remove_nodes_recursive(head: Option<Box<ListNode>>, cur_max: i32) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let next = node.next.take();
                if node.val >= cur_max {
                    node.next = Solution::remove_nodes_recursive(next, node.val);
                    Some(node)
                } else {
                    Solution::remove_nodes_recursive(next, cur_max)
                }
            }
            None => None,
        }
    }
}

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // If the list is empty, return None
        if head.is_none() {
            return None;
        }

        // Unwrap the head node since we know it's not None
        let mut boxed_head = head.unwrap();

        // If the next node exists, process it recursively
        if let Some(next_node) = boxed_head.next {
            let node = Self::remove_nodes(Some(next_node));

            // Check if the current node's value is less than the next node's value
            if boxed_head.val < node.as_ref().unwrap().val {
                return node;
            } else {
                boxed_head.next = node;
            }
        }

        // Return the current node
        Some(boxed_head)
    }
}
