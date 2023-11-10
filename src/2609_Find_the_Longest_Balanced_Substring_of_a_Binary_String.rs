// 2609_Find_the_Longest_Balanced_Substring_of_a_Binary_String
// https://leetcode.cn/problems/find-the-longest-balanced-substring-of-a-binary-string/description/

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut s: Vec<_> = s.chars().collect();
        s.push('0');
        let mut idx = 0;
        while idx < s.len() && s[idx] != '0' {
            idx += 1;
        }

        let mut cnts_0: Vec<i32> = vec![];
        let mut cnts_1: Vec<i32> = vec![];
        let mut len = 0;
        let mut pre = '0';
        while idx < s.len() {
            if s[idx] == pre {
                len += 1;
            } else {
                if pre == '0' {
                    cnts_1.push(len);
                } else {
                    cnts_0.push(len);
                }
                len = 1
            }
            pre = s[idx];
            idx += 1
        }   

        let mut ans = 0;
        // println!("{:?}", (cnts_0.clone(),cnts_1.clone()));
        for i in 0..cnts_0.len().min(cnts_1.len()) {
            ans = ans.max(cnts_0[i].min(cnts_1[i]))
        }
        
        ans * 2
    }
}