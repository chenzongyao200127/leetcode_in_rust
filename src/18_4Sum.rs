// 18. 4Sum
// https://leetcode.cn/problems/4sum/

use std::collections::HashSet;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        for i in 0..nums.len()-3 {
            for j in i+1..nums.len()-2 {
                for m in j+1..nums.len()-1 {
                    for n in m+1..nums.len() {
                        if nums[i]+nums[j]+nums[m]+nums[n] == target {
                            set.insert(vec![nums[i],nums[j],nums[m],nums[n]]);
                        }
                    }
                }
            }
        }
        for item in set {
            ans.push(item);
        }
        ans
    }
}