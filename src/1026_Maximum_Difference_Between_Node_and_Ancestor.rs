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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut maximum: i32, mut minimum: i32) -> i32 {
            if let Some(node) = root {
                let val = node.borrow().val;

                maximum = maximum.max(val);
                minimum = minimum.min(val);

                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();

                (maximum - minimum).max(dfs(left, maximum, minimum).max(dfs(right, maximum, minimum)))
            } else { 0 }
        }
        let val = root.as_ref().unwrap().borrow().val;
        
        dfs(root, val, val)
    }
}
