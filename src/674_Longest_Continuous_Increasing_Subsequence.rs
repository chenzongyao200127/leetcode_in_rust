// 674_Longest_Continuous_Increasing_Subsequence
// https://leetcode.cn/problems/longest-continuous-increasing-subsequence/

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                dp[i] = dp[i-1] + 1
            }
        }

        return *dp.iter().max().unwrap()
    }
}


impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut length = 1;
        let mut res = 1;
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                length += 1;
                res = res.max(length);
            } else {
                length = 1;
            }
        }

        res
    }
}