// 213_House_Robber_II
// https://leetcode.cn/problems/house-robber-ii/

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        for i in 0..nums.len()-1 {
            nums1.push(nums[i]);
        }
        for i in 1..nums.len() {
            nums2.push(nums[i]);
        }
        return (ori_rob(nums1)).max(ori_rob(nums2))
    }
}

pub fn ori_rob(nums: Vec<i32>) -> i32 {
    let mut dp = vec![];
    if nums.len() == 0 {
        return 0;
    }
    dp.push(0);
    dp.push(nums[0]);
    for i in 1..nums.len() {
        dp.push((nums[i] + dp[i-1]).max(dp[i]));
    }
    
    dp[dp.len()-1] as i32
}


impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        // Helper function to calculate the maximum amount for a linear array
        fn rob_linear(nums: &[i32]) -> i32 {
            let mut prev = 0;
            let mut curr = 0;
            for &num in nums {
                let temp = curr;
                curr = std::cmp::max(prev + num, curr);
                prev = temp;
            }
            curr
        }

        // Calculate the maximum amount for two cases: including the first house and excluding the last house
        std::cmp::max(rob_linear(&nums[0..n - 1]), rob_linear(&nums[1..n]))

    }
}