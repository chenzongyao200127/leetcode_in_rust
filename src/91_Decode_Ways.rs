// 91. Decode Ways
// https://leetcode.cn/problems/decode-ways/description/

// A message containing letters from A-Z can be encoded into numbers using the following mapping:

// 'A' -> "1"
// 'B' -> "2"
// ...
// 'Z' -> "26"
// To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:

// "AAJF" with the grouping (1 1 10 6)
// "KJF" with the grouping (11 10 6)
// Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".

// Given a string s containing only digits, return the number of ways to decode it.

// The test cases are generated so that the answer fits in a 32-bit integer.

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        if s[0] == '0' {
            return 0;
        }
        let (mut pre, mut cur) = (1, 1);
        for i in 1..s.len() {
            match (s[i - 1], s[i]) {
                ('0' | '3'..='9', '0') => return 0,
                (_, '0') => {
                    let t = pre;
                    pre = cur;
                    cur = t;
                }
                ('1', '1'..='9') | ('2', '1'..='6') => {
                    let t = cur + pre;
                    pre = cur;
                    cur = t;
                }
                _ => pre = cur,
            }
        }
        cur
    }
}
