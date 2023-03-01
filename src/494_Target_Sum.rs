// 494. Target Sum
// https://leetcode.cn/problems/target-sum/

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let neg = sum - target;
        if neg < 0 || neg & 1 == 1 {
            return 0;
        }
    
        let neg = neg as usize / 2;
        let mut dp = vec![0; neg as usize + 1];
        dp[0] = 1;
        
        nums.iter().for_each(|&n| {
            (n as usize..=neg).rev().for_each(|i| {
                dp[i] = dp[i].max(dp[i] + dp[i - n as usize]);
            })
        });
    
        dp[neg]
    }
}