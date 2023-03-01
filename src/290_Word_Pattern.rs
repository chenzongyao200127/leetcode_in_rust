// 290. Word Pattern
// https://leetcode.cn/problems/word-pattern/

use std::collections::{HashMap};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split(" ").collect();
        let pat_chars:Vec<char> = pattern.chars().collect();
        if words.len() != pat_chars.len() {
            return false;
        }
        let mut map: HashMap<&str, char> = HashMap::new();
        for i in 0..words.len() {
            map.entry(words[i]).or_insert(pat_chars[i]);
        }
        let mut new_str = String::new();
        for &str in words.iter() {
            let &v = map.get(&str).unwrap();
            new_str.push(v);
        }

        let mut map2: HashMap<char, &str> = HashMap::new();
        for i in 0..pat_chars.len() {
            map2.insert(pat_chars[i], words[i]);
        }
        let mut new_str2 = String::new();
        for i in pat_chars {
            let &v = map2.get(&i).unwrap();
            new_str2.push_str(v);
            new_str2.push(' ');
        }
        new_str2.pop();
        
        new_str2 == s &&  new_str == pattern
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut map = HashMap::new();
        let mut rev = HashMap::new();
        for (ch, &word) in pattern.chars().zip(words.iter()) {
            if let Some(&w) = map.get(&ch) {
                if w != word {
                    return false;
                }
            } else {
                if rev.contains_key(word) {
                    return false;
                }
                map.insert(ch, word);
                rev.insert(word, ch);
            }
        }
        true
    }
}