// 1625_Lexicographically_Smallest_String_After_Applying_Operations
// https://leetcode.cn/problems/lexicographically-smallest-string-after-applying-operations/

/// 解题失败，看题解了... 
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let mut vis = vec![0; n];
        let mut res = s; 
        // 将 s 延长一倍，方便截取轮转后的字符串 t
        s.repeate(2);


    }
}