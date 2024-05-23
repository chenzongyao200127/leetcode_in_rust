// 70_爬楼梯
// https://leetcode.cn/problems/climbing-stairs/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut dp = vec![0; n as usize];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize - 1]
    }
}
