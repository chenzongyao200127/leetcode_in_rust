// 32. Longest Valid Parentheses 
// Hard
// https://leetcode.cn/problems/longest-valid-parentheses/

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut stk = vec![-1];
        s.chars().enumerate().for_each(|(i, c)| {
            if c == '(' {
                stk.push(i as i32);
            } else {
                stk.pop();
                if stk.is_empty() {
                    stk.push(i as i32);
                } else {
                    ans = ans.max(i as i32 - stk.last().unwrap());
                }
            }
        });
    
        ans
    }
}