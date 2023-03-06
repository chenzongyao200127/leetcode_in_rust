// 1653. Minimum Deletions to Make String Balanced
// https://leetcode.cn/problems/minimum-deletions-to-make-string-balanced/

// 动态规划（一次遍历）
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let (mut f, mut cnt_b) = (0, 0);
        for i in 0..s_chars.len() {
            if s_chars[i] == 'b' {
                cnt_b += 1;
            } else {
                f = (f+1).min(cnt_b);
            }
        }
        f
    }
}


// 前后缀分解（两次遍历）
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut del = 0;
        for ch in s_chars.iter() {
            del += 'b' as u8 - *ch as u8; //// 统计 'a' 的个数
        }
        let mut ans = del;
        for ch in s_chars {
            del += (ch as u8 - 'a' as u8) * 2 - 1;
            ans = ans.min(del);
        }
        ans as i32
    }
}


// class Solution {
//     public:
//         int minimumDeletions(string s) {
//             int countB = 0, dp = 0;  // 令dp表示将s[0,i]变平衡最少需要删除的次数
    
//             for (char c : s) {
//                 if (c == 'a') {
//                     dp = min(dp + 1, countB);  // 末尾出现a的时候，2个选择：1、删除这个a，然后将s[0,i-1]变平衡；2、保留这个a，删除前面所有的b
//                 }
//                 else {
//                     ++countB;  // 出现b的时候，最后面的b不需要删除，dp不变，b的个数加1就好
//                 }
//             }
    
//             return dp;
//         }
//     };

