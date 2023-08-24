// 44_Wildcard_Matching
// https://leetcode.cn/problems/wildcard-matching/description/

// 给你一个输入字符串 (s) 和一个字符模式 (p) ，请你实现一个支持 '?' 和 '*' 匹配规则的通配符匹配：
// '?' 可以匹配任何单个字符。
// '*' 可以匹配任意字符序列（包括空字符序列）。
// 判定匹配成功的充要条件是：字符模式必须能够 完全匹配 输入字符串（而不是部分匹配）。

// 示例 1：
// 输入：s = "aa", p = "a"
// 输出：false
// 解释："a" 无法匹配 "aa" 整个字符串。

// 示例 2：
// 输入：s = "aa", p = "*"
// 输出：true
// 解释：'*' 可以匹配任意字符串。

// 示例 3：
// 输入：s = "cb", p = "?a"
// 输出：false
// 解释：'?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。

// 直接转换成第10题
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let p = p.replace("*", "?*");
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
                    matches[i + 1][j + 1] = matches[i][j] && (sc == pc || *pc == '?');
                } else {
                    matches[i + 1][j + 1] = matches[i + 1][j - 1] || 
                                            (matches[i][j + 1] && (*sc == p_chars[j - 1] || p_chars[j - 1] == '?'));
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
