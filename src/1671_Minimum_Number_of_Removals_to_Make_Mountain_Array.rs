// 1671_Minimum_Number_of_Removals_to_Make_Mountain_Array
// https://leetcode.cn/problems/minimum-number-of-removals-to-make-mountain-array/description/

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lis = vec![1; n];
        let mut lds = vec![1; n];

        // 计算 LIS
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    lis[i] = lis[i].max(lis[j] + 1);
                }
            }
        }

        // 计算 LDS
        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] > nums[j] {
                    lds[i] = lds[i].max(lds[j] + 1);
                }
            }
        }

        // 找到最大的 lis[i] + lds[i] - 1
        let mut max_length = 0;
        for i in 0..n {
            if lis[i] > 1 && lds[i] > 1 {
                // 确保 i 可以作为山峰
                max_length = max_length.max(lis[i] + lds[i] - 1);
            }
        }

        (n as i32) - (max_length as i32)
    }
}

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let lis = Self::longest_increasing_subsequence(&nums);
        let lds = Self::longest_increasing_subsequence(&nums.into_iter().rev().collect());

        // 找到最大的 lis[i] + lds[n-i-1] - 1
        let mut max_length = 0;
        for i in 0..n {
            if lis[i] > 1 && lds[n - i - 1] > 1 {
                // 确保 i 可以作为山峰
                max_length = max_length.max(lis[i] + lds[n - i - 1] - 1);
            }
        }

        (n as i32) - (max_length as i32)
    }

    // 计算最长递增子序列
    fn longest_increasing_subsequence(nums: &Vec<i32>) -> Vec<usize> {
        let n = nums.len();
        let mut dp = vec![1; n];

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        dp
    }
}
