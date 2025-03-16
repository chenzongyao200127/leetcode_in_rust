// 53_Maximum_Subarray
// https://leetcode.cn/problems/maximum-subarray/

// 53. 最大子数组和
// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
// 子数组 是数组中的一个连续部分。
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.iter().filter(|&x| *x < 0).count() == nums.len() {
            let mut ns = nums.clone();
            ns.sort_unstable();
            return ns[nums.len() - 1];
        }
        let mut pre_sum = nums[0];
        let mut max = i32::MIN;
        for i in 1..nums.len() {
            pre_sum = pre_sum.max(0);
            max = max.max(pre_sum);
            pre_sum += nums[i];
        }
        max = max.max(pre_sum);

        max
    }
}

use std::cmp;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        let mut ans = nums[0];
        for i in 1..nums.len() {
            if dp[i - 1] <= 0 {
                dp[i] = nums[i];
            } else {
                dp[i] = dp[i - 1] + nums[i];
            }
            ans = cmp::max(ans, dp[i]);
        }
        ans
    }
}
