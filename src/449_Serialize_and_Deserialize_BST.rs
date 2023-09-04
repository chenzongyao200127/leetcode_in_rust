// 449_Serialize_and_Deserialize_BST
// https://leetcode.cn/problems/serialize-and-deserialize-bst/description/?envType=daily-question&envId=2023-09-04
// Definition for a binary tree node.


// #[derive(Debug, PartialEq, Eq)]
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
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        self._serialize(root, &mut res);
        res
    }
    
    fn _serialize(&self, node: Option<Rc<RefCell<TreeNode>>>, res: &mut String) {
        if let Some(n) = node {
            res.push_str(&n.borrow().val.to_string());
            res.push(',');
            self._serialize(n.borrow().left.clone(), res);
            self._serialize(n.borrow().right.clone(), res);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals: Vec<i32> = data.split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        self._deserialize(&mut vals, i32::MIN, i32::MAX)
    }
    
    fn _deserialize(&self, vals: &mut Vec<i32>, lower: i32, upper: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let val = *vals.first().unwrap();
        if val < lower || val > upper {
            return None;
        }
        
        vals.remove(0);
        let root = Rc::new(RefCell::new(TreeNode::new(val)));
        root.borrow_mut().left = self._deserialize(vals, lower, val);
        root.borrow_mut().right = self._deserialize(vals, val, upper);
        
        Some(root)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */