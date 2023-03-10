// 15_3Sum
// https://leetcode.cn/problems/3sum/

// 15. 三数之和
// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，
// 同时还满足 nums[i] + nums[j] + nums[k] == 0 。请
// 你返回所有和为 0 且不重复的三元组。
// 注意：答案中不可以包含重复的三元组。


// 排序 + 双指针
// 本题的难点在于如何去除重复解。
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort_unstable();
        let mut ans:Vec<Vec<i32>> = vec![];
        for i in 0..nums.len()-2 {
            if nums[i] > 0 {
                break;
            }
            if i>0 && nums[i] == nums[i-1] {
                continue;
            }
            let target = -nums[i];
            let mut left = i+1;
            let mut right = nums.len()-1;
            while left < right {
                if nums[left] + nums[right] > target || (right<nums.len()-1 && nums[right+1] == nums[right]) {
                    right -= 1;
                } else if nums[left] + nums[right] < target || (left>i+1 && nums[left] == nums[left-1]) {
                    left += 1;
                } else {
                    ans.push(vec![nums[i] as i32, nums[left] as i32, nums[right] as i32]);
                    left += 1;
                    right -= 1;
                }
            }
        }
        ans
    }
}


use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        for i in 0..nums.len()-2 {
            let target = -nums[i];
            let mut left = i+1;
            let mut right = nums.len()-1;
            while left < right {
                if nums[left] + nums[right] > target {
                    right -= 1;
                } else if nums[left] + nums[right] < target {
                    left += 1;
                } else {
                    set.insert(vec![nums[i] as i32, nums[left] as i32, nums[right] as i32]);
                    left += 1;
                    right -= 1
                }
            }
        }
        set.into_iter().collect::<Vec<_>>()
    }
}


impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..n-2 {
            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                if left > i + 1 && nums[left-1] == nums[left] {
                    left += 1;
                    continue;
                }
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        res
    }
}