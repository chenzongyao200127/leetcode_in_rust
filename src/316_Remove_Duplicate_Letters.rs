// 316_Remove_Duplicate_Letters
// https://leetcode.cn/problems/remove-duplicate-letters/

// 316. 去除重复字母
// 给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack: Vec<char> = vec![];

        let last = s.as_bytes().iter().enumerate().fold([0; 26], |mut acc, (i, &u)| {
            acc[(u - b'a') as usize] = i;
            acc
        });

        let mut set = HashSet::new();

        for (i, x) in s.chars().enumerate() {

            if set.contains(&x) {
                continue;
            }

            loop {
                match stack.last() {
                    Some(&top) if top as u8 > x as u8 && last[(top as u8 - b'a') as usize] > i => {
                        stack.pop();
                        set.remove(&top);
                    }
                    _ => { break; }
                }
            }

            stack.push(x);
            set.insert(x);
        }
        
        stack.iter().collect()
    }
}



/// 思路就是 遇到一个新字符 如果比栈顶小 并且在新字符后面还有和栈顶一样的 就把栈顶的字符抛弃了
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stk = vec![];
        let mut flag_vec = vec![false; 26];
        // let mut flag_vec = vec![false; 4];
        let s_chars: Vec<char> = s.chars().collect();
        for i in 0..s_chars.len() {
            if flag_vec[(s_chars[i] as u8 - 'a' as u8) as usize] == true {
                continue;
            }
            while !stk.is_empty() && (s_chars[i] as u8) < (stk[stk.len()-1] as u8) 
                && is_exist_behind(s.clone(), stk[stk.len()-1], i) {
                    flag_vec[(stk[stk.len()-1] as u8 - 'a' as u8) as usize] = false;
                    stk.pop();
                }
            stk.push(s_chars[i]);
            flag_vec[(s_chars[i] as u8 - 'a' as u8) as usize] = true;
        }

        stk.iter().collect::<String>()
    }
}

pub fn is_exist_behind(s: String, peek: char, cur_idx: usize) -> bool {
    let s: Vec<char> = s.chars().collect();
    for i in cur_idx..s.len() {
        if s[i] == peek {
            return true;
        }
    }
    false
}