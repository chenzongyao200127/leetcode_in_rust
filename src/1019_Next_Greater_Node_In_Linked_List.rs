// https://leetcode.cn/problems/next-greater-node-in-linked-list/submissions/

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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut cur = 0;
        let mut head = head.as_ref();

        while let Some(h) = head {
            ans.push(0);

            while !stack.is_empty() && stack[stack.len() - 1].0 < h.val {
                ans[stack[stack.len() - 1].1] = h.val;
                stack.pop();
            }

            stack.push((h.val, cur));
            cur += 1;
            head = h.next.as_ref();
        }

        ans
    }
}

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut cur = 0;

        while let Some(mut node) = head {
            ans.push(0);

            while !stack.is_empty() && stack[stack.len() - 1].0 < node.val {
                ans[stack[stack.len() - 1].1] = node.val;
                stack.pop();
            }

            stack.push((node.val, cur));
            cur += 1;
            head = node.next.take();
        }

        ans
    }
}