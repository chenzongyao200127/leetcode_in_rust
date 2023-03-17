// 198_House_Robber
// https://leetcode.cn/problems/house-robber/description/?utm_source=LCUS&utm_medium=ip_redirect&utm_campaign=transfer2china

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        dp.push(0);
        dp.push(nums[0]);
        for i in 1..nums.len() {
            dp.push((nums[i] + dp[i-1]).max(dp[i]));
        }
        
        dp[dp.len()-1] as i32
    }
}

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