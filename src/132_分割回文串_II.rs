// 132_分割回文串_II
// https://leetcode.cn/problems/palindrome-partitioning-ii/description/?envType=daily-question&envId=2025-03-02

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // dp[i][j] means whether s[i..=j] is a palindrome
        let mut dp = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                dp[i][j] = s[i] == s[j] && (j - i < 2 || dp[i + 1][j - 1]);
            }
        }
        // cut[i] means the minimum cut of s[0..=i]
        let mut cut = vec![n as i32; n];
        for i in 0..n {
            // if s[0..=i] is a palindrome, we don't need to cut
            if dp[0][i] {
                cut[i] = 0;
            } else {
                // if s[0..=i] is not a palindrome, we need to find the minimum cut
                for j in 0..i {
                    // if s[j + 1..=i] is a palindrome, we
                    // need to compare cut[i] and cut[j] + 1
                    if dp[j + 1][i] {
                        cut[i] = cut[i].min(cut[j] + 1);
                    }
                }
            }
        }
        cut[n - 1]
    }
}
