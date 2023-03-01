// 532. K-diff Pairs in an Array
// https://leetcode.cn/problems/k-diff-pairs-in-an-array/

use std::collections::HashSet;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..nums.len() {
            if nums[0..i].contains(&(nums[i]-k)) || (i != nums.len() && nums[i+1..nums.len()].contains(&(nums[i]-k))) {
                set.insert((nums[i], nums[i]-k));
            }
        }

        set.len() as i32
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut M = HashMap::new();
        for n in nums {
            *M.entry(n).or_insert(0) += 1;
        }
        let mut res = 0;
        for (&num, &cnt) in M.iter() {
            if k == 0 {
                if cnt > 1 {
                    res += 1;
                }
            } else {
                if M.contains_key(&(num+k)) {
                    res += 1;
                }
            }
        }
        ans
    }
}