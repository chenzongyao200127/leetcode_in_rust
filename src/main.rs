pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        let width = (right - left) as i32;
        let (left_height, right_height) = (height[left], height[right]);

        max_area = max_area.max(width * left_height.min(right_height));

        if left_height < right_height {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut ans = 0;
    for (idx, &h) in height.iter().enumerate() {
        while let Some(&last_idx) = stack.last() {
            if height[last_idx] < h {
                let low = stack.pop().unwrap();
                if let Some(&last_idx) = stack.last() {
                    let bounded_height = h.min(height[last_idx]) - height[low];
                    ans += bounded_height * (idx - last_idx - 1) as i32;
                }
            } else {
                break;
            }
        }
        stack.push(idx);
    }

    ans
}

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let s = s.chars().collect::<Vec<char>>();
    let mut cnt = vec![0; 26];
    let mut ans = vec![];
    for &c in p.as_bytes() {
        cnt[(c - 'a' as u8) as usize] += 1;
    }
    let mut r = p.len() - 1;
    let mut l = 0;
    let mut window = vec![0; 26];
    if p.len() > s.len() {
        return ans;
    }
    for i in 0..p.len() {
        window[(s[i] as u8 - 'a' as u8) as usize] += 1;
    }
    // println!("{:?}", cnt);
    while l < s.len() - p.len() {
        // println!("{:?}", window);
        if window == cnt {
            ans.push(l as i32);
        }
        l += 1;
        r += 1;
        window[(s[l - 1] as u8 - 'a' as u8) as usize] -= 1;
        window[(s[r] as u8 - 'a' as u8) as usize] += 1;
    }
    if window == cnt {
        ans.push(l as i32);
    }

    ans
}

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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = Vec::new();
    dfs(&root, &mut ans);
    ans
}

pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if let Some(node) = node {
        let node_borrowed = node.borrow();
        dfs(&node_borrowed.left, ans);
        ans.push(node_borrowed.val);
        dfs(&node_borrowed.right, ans);
    }
}

fn main() {
    let ans = trap(vec![4, 2, 0, 3, 2, 5]);
    assert_eq!(ans, 9);
}
