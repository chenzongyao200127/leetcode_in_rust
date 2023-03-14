// 438_Find_All_Anagrams_in_a_String
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/

/// 好慢...
use std::collections::HashMap;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let mut ans = vec![];
        let mut target_map: HashMap<u8, usize> = HashMap::new();
        p.as_bytes().iter().for_each(|ch| {
            target_map.entry(*ch).and_modify(|cnt| *cnt+=1).or_insert(1);
        });

        let s = s.as_bytes();
        let mut s_map: HashMap<u8, usize> = HashMap::new();
        for i in 0..s.len() {
            if i < p.len() {
                s_map.entry(s[i]).and_modify(|cnt| *cnt+=1).or_insert(1);
            } else {
                s_map.entry(s[i]).and_modify(|cnt| *cnt+=1).or_insert(1);
                s_map.entry(s[i - p.len()]).and_modify(|cnt| *cnt-=1);
                if *s_map.get(&s[i - p.len()]).unwrap() == 0 {
                    s_map.remove(&s[i - p.len()]);
                }
            }

            if s_map == target_map {
                ans.push(i as i32 - p.len() as i32 + 1);
            }
        }

        ans
    }
}


impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return Vec::new();
        }

        let mut ans = vec![];
        let mut p_count = [0; 26];
        for &i in p.as_bytes() {
            p_count[(i - b'a') as usize] += 1;
        }

        let mut s_count = [0; 26];
        let bytes = s.as_bytes();
        for i in 0..s.len() {
            if i < p.len() {
                s_count[(bytes[i] - b'a') as usize] += 1;
            } else {
                s_count[(bytes[i] - b'a') as usize] += 1;
                s_count[(bytes[i - p.len()] - b'a') as usize] -= 1;
            }
            
            if s_count == p_count {
                ans.push((i - p.len() + 1) as i32);
            }
        }

        ans
    }
}