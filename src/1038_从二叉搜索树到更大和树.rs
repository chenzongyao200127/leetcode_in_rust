// 1038_从二叉搜索树到更大和树
// https://leetcode.cn/problems/binary-search-tree-to-greater-sum-tree/description/

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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
            if let Some(x) = node {
                let mut x = x.borrow_mut();
                dfs(x.right.as_ref(), s);
                *s += x.val;
                x.val = *s;
                dfs(x.left.as_ref(), s);
            }
        }
        let mut s = 0;
        dfs(root.as_ref(), &mut s);
        root
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = VecDeque::new();
        let mut node = root.clone();
        let mut sum = 0;

        while node.is_some() || !stack.is_empty() {
            // Go as far right as possible
            while let Some(n) = node {
                stack.push_back(n.clone());
                node = n.borrow().right.clone();
            }

            node = stack.pop_back();
            if let Some(n) = node.clone() {
                let mut n_borrowed = n.borrow_mut();
                sum += n_borrowed.val;
                n_borrowed.val = sum;

                // Move to the left node
                node = n_borrowed.left.clone();
            }
        }

        root
    }
}
