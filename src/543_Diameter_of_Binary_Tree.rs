// 543. 二叉树的直径
// https://leetcode.cn/problems/diameter-of-binary-tree/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        depth(&root, &mut ans);
        ans
    }
}

pub fn depth(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
    if let Some(n) = node {
        let left_depth = depth(&n.borrow().left, ans);
        let right_depth = depth(&n.borrow().right, ans);
        *ans = (*ans).max(left_depth + right_depth);
        left_depth.max(right_depth) + 1
    } else {
        0
    }
}