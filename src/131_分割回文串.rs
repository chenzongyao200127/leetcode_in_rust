// 131_分割回文串
// https://leetcode.cn/problems/palindrome-partitioning/description/?envType=daily-question&envId=2025-03-01

impl Solution {
    fn dfs(
        s: &[u8],
        dp: &Vec<Vec<bool>>,
        start: usize,
        path: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            res.push(path.clone());
            return;
        }
        for i in start..s.len() {
            if dp[start][i] {
                path.push(String::from_utf8(s[start..=i].to_vec()).unwrap());
                // new start is i + 1, we need to find the next palindrome
                Self::dfs(s, dp, i + 1, path, res);
                path.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                // why?
                // because dp[i][j] depends on dp[i + 1][j - 1]
                // so we need to calculate dp[i][j] from the end to the start
                dp[i][j] = s[i] == s[j] && (j - i < 2 || dp[i + 1][j - 1]);
            }
        }
        let mut res = vec![];
        let mut path = vec![];
        Self::dfs(&s, &dp, 0, &mut path, &mut res);
        res
    }
}
