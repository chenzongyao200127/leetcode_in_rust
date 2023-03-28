// 1092_Shortest_Common_Supersequence
// https://leetcode.cn/problems/shortest-common-supersequence/

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1: Vec<char> = str1.chars().collect();
        let s2: Vec<char> = str2.chars().collect();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 0..s1.len()+1 {
            dp[i][0] = 0;
        }

        for j in 0..s2.len()+1 {
            dp[0][j] = 0;
        }

        for i in 0..s1.len() {
            for j in 0..s2.len() {
                if s1[i] == s2[j] {
                    dp[i+1][j+1] = dp[i][j] + 1;
                } else {
                    dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
                }
            }
        }

        let mut sub_s = String::new();
        let mut i = s1.len();
        let mut j = s2.len();

        while i > 0 || j > 0 {
            if i == 0 {
                sub_s.push(s2[j - 1]);
                j -= 1;
            } else if j == 0 {
                sub_s.push(s1[i - 1]);
                i -= 1;
            } else if s1[i - 1] == s2[j - 1] {
                sub_s.push(s1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                sub_s.push(s1[i - 1]);
                i -= 1;
            } else {
                sub_s.push(s2[j - 1]);
                j -= 1;
            }
        }

        sub_s.chars().rev().collect()
    }
}


impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let (m, n) = (str1.len(), str2.len());

        // dp[i][j]: str1中的前i个字符和str2中的前j个字符的最长公共子序列的长度
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        // 回溯寻找答案
        let mut ans = String::new();
        let (mut x, mut y) = (m, n);
        while x != 0 || y != 0 {
            if x == 0 {
                ans.insert(0, s2[y - 1] as char);
                y -= 1;
                continue;
            }
            if y == 0 {
                ans.insert(0, s1[x - 1] as char);
                x -= 1;
                continue;
            }
            if s1[x - 1] == s2[y - 1] {
                ans.insert(0, s1[x - 1] as char);
                x -= 1;
                y -= 1;
            } else {
                if dp[x - 1][y] > dp[x][y - 1] {
                    ans.insert(0, s1[x - 1] as char);
                    x -= 1;
                } else {
                    ans.insert(0, s2[y - 1] as char);
                    y -= 1;
                }
            }
        }
        ans
    }
}