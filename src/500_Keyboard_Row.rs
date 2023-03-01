// 500. Keyboard Row
// https://leetcode.cn/problems/keyboard-row/


use std::collections::HashSet;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let a = "qwertyuiop".chars().collect::<HashSet<_>>();
        let b = "asdfghjkl".chars().collect::<HashSet<_>>();
        let c = "zxcvbnm".chars().collect::<HashSet<_>>();
        let mut ans = vec![];
        for word in words {
            let t = word.to_lowercase().chars().collect::<HashSet<_>>();
            if t.is_subset(&a) || t.is_subset(&b) || t.is_subset(&c) {
                ans.push(word);
            }
        }
        ans
    }
}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let qwert = vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"];

        words.into_iter().fold(vec![], |mut ans, word| {
            let mut i = 0;
            let ch = word.chars().next().unwrap();
            i = loop {
                if qwert[i].contains(ch) || qwert[i].contains(&ch.to_lowercase().to_string()) {
                    break i;
                }
                i += 1;
            };
            let mut can_flag = true;
            for character in word.chars() {
                if !qwert[i].contains(character) && !qwert[i].contains(&character.to_lowercase().to_string()) {
                    can_flag = false; 
                }
            }
            if can_flag {
                ans.push(word);
            }
            ans
        })
    }
}

// 
// class Solution:
//     def findWords(self, words: List[str]) -> List[str]:
//         return [word for word in words if re.match("^[qwertyuiop]*$", word, re.I) or re.match("^[asdfghjkl]*$", word, re.I) or re.match("^[zxcvbnm]*$", word, re.I)]