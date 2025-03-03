// 1278_分割回文串_III
// https://leetcode.cn/problems/palindrome-partitioning-iii/description/

// dp[i][j] 表示将 s[i..=j] 分割成 k 个回文串的最小修改次数
// dp2[i][j] 表示将 s[0..=i] 分割成 j 个回文串的最小修改次数]
// dp[i][j] = dp[i + 1][j - 1] if s[i] == s[j] else dp[i + 1][j - 1] + 1
// dp2[i][1] = dp[0][i] 即 s[0..=i] 是一个回文串，不需要修改
impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // dp[i][j] means the minimum changes to make s[i..=j] a palindrome
        let mut dp = vec![vec![0; n]; n];
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                dp[i][j] = if s[i] == s[j] {
                    // if s[i] == s[j], we don't need to change
                    dp[i + 1][j - 1]
                } else {
                    // if s[i] != s[j], we need to change
                    dp[i + 1][j - 1] + 1
                };
            }
        }
        // dp2[i][j] means the minimum changes to make s[0..=i] have j palindromes
        // dp2[i][j] = min(dp2[i][j], dp2[l][j - 1] + dp[l + 1][i])
        // dp2[l][j - 1] + dp[l + 1][i] means we split s[0..=i] into j palindromess
        // dp2[l][j - 1] means we split s[0..=l] into j - 1 palindromes
        // dp[l + 1][i] means we split s[l + 1..=i] into 1 palindrome
        let mut dp2 = vec![vec![n as i32; k as usize + 1]; n];
        for i in 0..n {
            // if s[0..=i] is a palindrome, we don't need to change
            dp2[i][1] = dp[0][i] as i32;
            for j in 2..=k as usize {
                for l in 0..i {
                    // if s[l + 1..=i] is a palindrome, we need to compare dp2[i][j] and dp2[l][j - 1] + dp[l + 1][i]
                    // why?
                    dp2[i][j] = dp2[i][j].min(dp2[l][j - 1] + dp[l + 1][i] as i32);
                }
            }
        }
        dp2[n - 1][k as usize]
    }
}
