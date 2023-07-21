// 413_Arithmetic_Slices
// https://leetcode.cn/problems/arithmetic-slices/description/

// 如果一个数列 至少有三个元素 ，并且任意两个相邻元素之差相同，则称该数列为等差数列。

// 例如，[1,3,5,7,9]、[7,7,7,7] 和 [3,-1,-5,-9] 都是等差数列。
// 给你一个整数数组 nums ，返回数组 nums 中所有为等差数组的 子数组 个数。

// 子数组 是数组中的一个连续序列。

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if len(nums) < 3 {
            return 0;
        }

        let mut dp = vec![0; n];
        dp[0] = 0;
        dp[1] = 0;

        let mut total = 0;
        for i in 2..n {
            if nums[i-2] + nums[i] == 2 * nums[i-1] {
                dp[i] = dp[i-1] + 1;
                total += dp[i];
            }
        }

        total
    }
}