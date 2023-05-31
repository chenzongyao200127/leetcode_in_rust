// 1110. Delete Nodes And Return Forest
// https://leetcode.cn/problems/delete-nodes-and-return-forest/

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

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let to_delete_set: HashSet<_> = to_delete.into_iter().collect();
        let mut roots = Vec::new();
        
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            is_root: bool,
            to_delete_set: &HashSet<i32>,
            roots: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node_ref) = node {
                let deleted;
                {
                    let node_borrow = node_ref.borrow();
                    deleted = to_delete_set.contains(&node_borrow.val);
                }
                let next_node_ref = node_ref.clone();
                {
                    let mut node_borrow = next_node_ref.borrow_mut();
                    node_borrow.left = dfs(node_borrow.left.take(), deleted, &to_delete_set, roots);
                    node_borrow.right = dfs(node_borrow.right.take(), deleted, &to_delete_set, roots);
                }
                if deleted {
                    None
                } else {
                    if is_root {
                        roots.push(Some(next_node_ref.clone()));
                    }
                    Some(next_node_ref)
                }
            } else {
                None
            }
        }

        dfs(root, true, &to_delete_set, &mut roots);
        roots
    }
}