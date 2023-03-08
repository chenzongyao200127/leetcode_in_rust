// 343_Integer_Break
// https://leetcode.cn/problems/integer-break/

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![1; n as usize];
        if n == 2 {
            return 1;
        }
        dp[0] = 1;
        dp[1] = 1;
        if n == 2 {
            return 1;
        }
        for i in 2..n as usize {
            for j in 1..i+1 {
                dp[i] = dp[i].max(((i+1-j)*j).max(dp[i-j] * j));
            }
        }

        dp[dp.len()-1] as i32
    }
}



impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let un = n as usize;
        let mut dp = vec![0; un + 1];
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..un + 1 {
            for j in 1..i {
                dp[i] = std::cmp::max(dp[i], std::cmp::max(j * (i - j), j * dp[i - j]))
            }
        }
        dp[un] as i32
    }
}


/// 方法一：动态规划
// class Solution:
//     def integerBreak(self, n: int) -> int:
//         dp = [0] * (n + 1)
//         for i in range(2, n + 1):
//             for j in range(i):
//                 dp[i] = max(dp[i], j * (i - j), j * dp[i - j])
//         return dp[n]


/// 方法二：优化的动态规划
// class Solution:
//     def integerBreak(self, n: int) -> int:
//         if n <= 3:
//             return n - 1
        
//         dp = [0] * (n + 1)
//         dp[2] = 1
//         for i in range(3, n + 1):
//             dp[i] = max(2 * (i - 2), 2 * dp[i - 2], 3 * (i - 3), 3 * dp[i - 3])
        
//         return dp[n]


/// 方法三：数学
// class Solution {
//     public int integerBreak(int n) {
//         if (n <= 3) {
//             return n - 1;
//         }
//         int quotient = n / 3;
//         int remainder = n % 3;
//         if (remainder == 0) {
//             return (int) Math.pow(3, quotient);
//         } else if (remainder == 1) {
//             return (int) Math.pow(3, quotient - 1) * 4;
//         } else {
//             return (int) Math.pow(3, quotient) * 2;
//         }
//     }
// }
