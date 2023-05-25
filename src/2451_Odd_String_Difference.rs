// 2451_Odd_String_Difference
// https://leetcode.cn/problems/odd-string-difference/

use std::collections::HashMap;
impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut cnt = HashMap::new();
        for s in words.iter() {
            let k = s.as_bytes()
                    .windows(2)
                    .map(|v| (v[1] as i32 - v[0] as i32)
                    .to_string() + ",")
                    .collect::<String>();
            let v = cnt.entry(k).or_insert(Vec::new());
            v.push(s);
        }
        cnt.values().find(|v| v.len() == 1).unwrap()[0].clone()
    }
}



impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut mp = std::collections::HashMap::new();

        for word in words.iter() {
            let mut tmp = String::new();
            let bytes = word.as_bytes();
            let n = bytes.len();

            for i in 1..n {
                tmp.push_str(&(bytes[i] as i32 - bytes[i-1] as i32).to_string());
                tmp.push(',');
            }
            println!("{}", tmp);
            mp.entry(tmp).or_insert(vec![]).push(word);
        }

        for (_, v) in mp {
            if v.len() == 1 {
                return v[0].to_string();
            }
        }

        "".to_string()
    }
}