// 217. Contains Duplicate
// https://leetcode.cn/problems/contains-duplicate/

use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for num in nums.into_iter() {
            map.entry(num).and_modify(|counter| *counter += 1).or_insert(1);
        }
        !map.into_iter().all(|x| x.1<=1)
    }
}


use std::collections::{HashSet};
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut H:HashSet<i32>=HashSet::with_capacity(nums.len());
        for i in nums {
            if let Some(_) = H.get(&i) {
                return true
            }
            H.insert(i);
        }
        false
    }
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                return true;
            }
        }

        false
    }
}