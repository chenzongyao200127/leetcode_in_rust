// 368_Largest_Divisible_Subset
// https://leetcode.cn/problems/largest-divisible-subset/submissions/

use std::collections::HashMap;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut dp: HashMap<usize, Vec<i32>> = HashMap::new();
        for i in 0..n {
            let mut tmp = vec![];
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if dp[&j].len() > tmp.len() {
                        tmp = dp[&j].clone();
                    }
                }
            }
            dp.insert(i, tmp);
            dp.get_mut(&i).unwrap().push(nums[i]);
        }
        let mut ans = vec![];
        for (_, v) in dp {
            if v.len() > ans.len() {
                ans = v;
            }
        }
        ans
    }
}