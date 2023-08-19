//  115_Distinct_Subsequences
// https://leetcode.cn/problems/distinct-subsequences/

// Given two strings s and t, return the number of distinct 
// subsequences of s which equals t.

// The test cases are generated so that the answer fits 
// on a 32-bit signed intege

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_chars: Vec<_> = s.chars().collect();
        let t_chars: Vec<_> = t.chars().collect();

        let mut dp = vec![vec![0; s_chars.len() + 1]; t_chars.len() + 1];

        // 初始化 dp 的第一行
        for i in 0..s_chars.len() {
            if s_chars[i] == t_chars[0] {
                dp[1][i + 1] = dp[1][i] + 1;
            } else {
                dp[1][i + 1] = dp[1][i];
            }
        }

        // 填充 dp 的其余部分
        for i in 1..t_chars.len() {
            for j in 0..s_chars.len() {
                if s_chars[j] == t_chars[i] {
                    dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j];
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j];
                }
            }
        }

        dp[t_chars.len()][s_chars.len()]
    }
}
