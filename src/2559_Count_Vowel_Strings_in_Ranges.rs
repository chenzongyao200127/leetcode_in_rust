// 2559_Count_Vowel_Strings_in_Ranges
// https://leetcode.cn/problems/count-vowel-strings-in-ranges/

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        let flags: Vec<bool> = words.iter()
            .map(|word| {
                let chars: Vec<char> = word.chars().collect();
                VOWELS.contains(&chars[0]) && VOWELS.contains(&chars[chars.len()-1])
            })
            .collect();

        let mut prefix = vec![0];
        let mut pre_sum = 0;

        for flag in flags {
            if flag {
                pre_sum += 1;
            }
            prefix.push(pre_sum);
        }

        queries.iter()
            .map(|query| {
                let left = query[0] as usize;
                let right = query[1] as usize;
                prefix[right+1] - prefix[left]
            })
            .collect()
    }
}



impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let is_vowel = |b: u8| -> bool { matches!(b, b'a' | b'e' | b'i' | b'o' | b'u') };

        let check = |s: &str| -> i32 {
            let dat = s.as_bytes();
            if is_vowel(dat[0]) && is_vowel(*dat.last().unwrap()) {
                1
            } else {
                0
            }
        };

        let tmp: Vec<i32> = words.into_iter().map(|s| check(s.as_str())).collect();

        let mut prefix = vec![0; tmp.len()];
        let mut suffix = vec![0; tmp.len()];

        for i in 1..tmp.len() {
            prefix[i] = prefix[i - 1] + tmp[i - 1];
        }

        for i in (0..(tmp.len() - 1)).rev() {
            suffix[i] = suffix[i + 1] + tmp[i + 1];
        }

        let total: i32 = tmp.iter().sum();
        queries
            .into_iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;
                total - suffix[r] - prefix[l]
            })
            .collect()
    }
}
