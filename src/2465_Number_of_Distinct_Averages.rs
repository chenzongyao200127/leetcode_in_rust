// 2465_Number_of_Distinct_Averages
// https://leetcode.cn/problems/number-of-distinct-averages/

use std::collections::HashSet;
impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut sum_set: HashSet<i32> = HashSet::new();
        nums.sort_unstable();
        for i in 0..nums.len()/2 {
            sum_set.insert(nums[i] + nums[nums.len() - i - 1]);
        }
        sum_set.len() as i32
    }
}

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
            nums.sort();
            let mut set = std::collections::HashSet::new();
            let mut l = 0;
            let mut r = nums.len() - 1;
            while l < r {
                set.insert(nums[l] + nums[r]);
                l += 1;
                r -= 1;
            }
            set.len() as i32
        }
    
    }