// 409. Longest Palindrome
// https://leetcode.cn/problems/longest-palindrome/

use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut ans = 0;
        let mut even_cnt = 0;
        let mut map: HashMap<char, usize> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();
        for char in s_chars {
            map.entry(char).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        for (&_, &v) in map.iter() {
            if v & 1 == 1 {
                ans += v - 1;
                even_cnt += 1;
            } else {
                ans += v;
            }
        }
        if even_cnt >= 1 {
            ans as i32 + 1
        } else {
            ans as i32
        }
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let counter = s.as_bytes().iter().fold(HashMap::new(), |mut acc, x| {
            let ptr = acc.entry(*x).or_insert(0);
            *ptr += 1;
            acc
        });

        let mut ans = 0;

        for (_, val) in counter {
            ans += val / 2 * 2;
            if ans % 2 == 0 && val % 2 == 1 {
                ans += 1;
            }
        }

        ans
    }
}