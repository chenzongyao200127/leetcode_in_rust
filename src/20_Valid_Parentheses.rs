// 20. Valid Parentheses
// https://leetcode.cn/problems/valid-parentheses/
// 0 ms 100%
// 2 MB 62.9%
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk: Vec<char> = vec!['0'];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => { stk.push(c) },
                ')' => {if stk.pop().unwrap() != '(' {return false}},
                ']' => {if stk.pop().unwrap() != '[' {return false}},
                '}' => {if stk.pop().unwrap() != '{' {return false}},
                _ => (),
            }   
        }

        stk.len() == 1
    }
}