// 10. Regular Expression Matching
// https://leetcode.cn/problems/regular-expression-matching/description/

// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

// '.' Matches any single character.​​​​
// '*' Matches zero or more of the preceding element.
// The matching should cover the entire input string (not partial).

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();
        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();

        let mut dp = vec![vec![false; n + 1]; m + 1];

        dp[0][0] = true;

        for j in 1..=n {
            if p[j-1] == '*' {
                // 当我们处理模式字符串 p 的第一个字符时，如果这个字符是 *，那么模式本身就是无效的，
                // 因为 * 总是应该出现在某个字符之后，(不用担心 j = 1 时进入分支)
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p[j-1] != '*' {
                    dp[i][j] = dp[i-1][j-1] && (s[i-1] == p[j-1] || p[j-1] == '.');
                } else {
                    dp[i][j] = (dp[i][j-2]) || (dp[i-1][j] && (s[i-1] == p[j-2] || p[j-2] == '.'))
                }
            }
        }
        
        dp[m][n]
    }
}


// GPT-4 可读性优化
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Convert string and pattern into character vectors for easier indexing
        let s_chars: Vec<_> = s.chars().collect();
        let p_chars: Vec<_> = p.chars().collect();

        // Initialize dynamic programming table
        let mut matches = vec![vec![false; p.len() + 1]; s.len() + 1];
        matches[0][0] = true;

        // Handle patterns like a* or a*b* or ... which can match empty string
        Self::init_match_empty_string(&mut matches, &p_chars);

        // Fill the dynamic programming table
        for (i, sc) in s_chars.iter().enumerate() {
            for (j, pc) in p_chars.iter().enumerate() {
                if *pc != '*' {
                    matches[i + 1][j + 1] = matches[i][j] && (sc == pc || *pc == '.');
                } else {
                    matches[i + 1][j + 1] = matches[i + 1][j - 1] || 
                                            (matches[i][j + 1] && (*sc == p_chars[j - 1] || p_chars[j - 1] == '.'));
                }
            }
        }
        
        matches[s.len()][p.len()]
    }

    fn init_match_empty_string(matches: &mut Vec<Vec<bool>>, p_chars: &Vec<char>) {
        // When pattern starts with a *, it is invalid. So, skip checking for j = 0.
        for j in 1..p_chars.len() {
            if p_chars[j] == '*' {
                matches[0][j + 1] = matches[0][j - 1];
            }
        }
    }
}
