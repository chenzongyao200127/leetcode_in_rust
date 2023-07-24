// 279_Perfect_Squares
// https://leetcode.cn/problems/perfect-squares/

// 给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。

// 完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![1000; n as usize + 1];
        dp[0] = 0;
        let mut i = 1;
        while i * i <= n {
            dp[(i*i) as usize] = 1;
            i += 1;
        }
        for i in 1..=n as usize {
            if dp[i] == 1000 {
                for j in 1..i/2+1 {
                    dp[i] = dp[i].min(dp[i-j] + dp[j])
                }
            }
        }
        dp[n as usize]
    }    
}


// 优化
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![1000; (n + 1) as usize];
        dp[0] = 0;
        for i in 1..=n as usize {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j*j] + 1);
                j += 1;
            }
        }
        dp[n as usize]
    }
}