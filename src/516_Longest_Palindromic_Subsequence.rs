// 516_Longest_Palindromic_Subsequence
// https://leetcode.cn/problems/longest-palindromic-subsequence/

// 给你一个字符串 s ，找出其中最长的回文子序列，并返回该序列的长度。
// 子序列定义为：不改变剩余字符顺序的情况下，删除某些字符或者不删除任何字符形成的一个序列。

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s1 = s.clone();
        let s2 = s.chars().rev().collect::<String>();

        #[inline]
        fn longest_common_subsequence(s1: &str, s2: &str) -> i32 {
            let s1_chars: Vec<char> = s1.chars().collect();
            let s2_chars: Vec<char> = s2.chars().collect();

            let mut dp = vec![vec![0; s1_chars.len() + 1]; s2_chars.len() + 1];
            for i in 1..=s2_chars.len() {
                for j in 1..=s1_chars.len() {
                    if s2_chars[i - 1] == s1_chars[j - 1] {
                        dp[i][j] = dp[i - 1][j - 1] + 1;
                    } else {
                        dp[i][j] = dp[i - 1][j].max(dp[i][j - 1])
                    }
                }
            }

            dp[s2_chars.len()][s1_chars.len()]
        }

        longest_common_subsequence(&s1, &s2)
    }
}