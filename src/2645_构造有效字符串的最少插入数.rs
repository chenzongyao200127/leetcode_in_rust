// 2645_构造有效字符串的最少插入数
// https://leetcode.cn/problems/minimum-additions-to-make-valid-string/description/

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let s = word.as_bytes();
        let mut t = 1;
        for i in 1..s.len() {
            if s[i - 1] >= s[i] {
                t += 1;
            }
        }
        return t * 3 - s.len() as i32;
    }
}
