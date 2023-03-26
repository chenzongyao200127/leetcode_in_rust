// 2395_Find_Subarrays_With_Equal_Sum
// https://leetcode.cn/problems/find-subarrays-with-equal-sum/

use std::collections::HashSet;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..nums.len()-1 {
            let tmp_sum = nums[i] + nums[i+1];
            if set.contains(&tmp_sum) {
                return true;
            } else {
                set.insert(tmp_sum);
            }
        }
        
        false
    }
}