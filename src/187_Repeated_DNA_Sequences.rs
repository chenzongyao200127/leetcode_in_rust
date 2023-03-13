// 187_Repeated_DNA_Sequences
// https://leetcode.cn/problems/repeated-dna-sequences/

use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut map: HashMap<&str, usize> = HashMap::new();
        let mut ans = vec![];
        let window = 10 as usize;
        for i in 0..s.len()-window+1 {
            let sub = &s[i..i+window];
            map.entry(sub).and_modify(|cnt| {
                *cnt += 1;
                if *cnt == 2 {
                    ans.push(sub.to_string());
                }
            }).or_insert(1);
        }
        ans
    }
}

/// 示例代码
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        if s.len() < 10 {
            return vec![];
        }

        let sv = s.as_str();
        for i in 0..=(s.len() - 10) {
            map.entry(&s[i..(i+10)]).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        map.iter().filter(|(_k, &v)|v > 1).map(|(k, _v)| k.to_string()).collect()
    }
}


use std::collections::HashSet;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }

        let mut set: HashSet<Vec<char>> = HashSet::new();
        let mut ans1: HashSet<Vec<char>> = HashSet::new();
        let s: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = 9;
        while right < s.len() {
            let mut tmp = vec![];
            for i in left..=right {
                tmp.push(s[i]);
            }
            if set.contains(&tmp) {
                ans1.insert(tmp.clone());
            }
            set.insert(tmp);
            left += 1;
            right += 1;
        }
        let ans = ans1.into_iter().collect::<Vec<_>>();
        let mut result = vec![];
        for item in ans.into_iter() {
            result.push(item.iter().collect::<String>())
        }

        result
    }
}