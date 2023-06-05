// 17_Letter_Combinations_of_a_Phone_Number
// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: std::collections::HashMap<char, &str> = vec![
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ].into_iter().collect();

        let strs = digits.chars().map(
            |c| map.get(&c).unwrap()
        );

        let mut results: Vec<String> = vec![];

        for s in strs {
            let new_results = s.chars().map(
                |c| {
                    if results.is_empty() {
                        vec![c.to_string()]
                    } else {
                        results.iter().cloned().map(
                        |mut s| {
                            s.push(c);
                            s
                        }).collect()
                    }
                }
            ).flatten().collect();

            results = new_results;            
        }

        results
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = hashmap! {
            b'2' => "abc",
            b'3' => "def",
            b'4' => "ghi",
            b'5' => "jkl",
            b'6' => "mno",
            b'7' => "pqrs",
            b'8' => "tuv",
            b'9' => "wxyz"
        };

        let mut res = vec![];
        recursive(&digits, &mut res, &mut vec![], &map);
        res
    }
}

fn recursive<'a>(
    s: &str,
    res: &mut Vec<String>,
    buf: &mut Vec<&'a str>,
    map: &'a HashMap<u8, &str>,
) {
    let current = match s.as_bytes().first() {
        Some(it) => it,
        _ => {
            if !buf.is_empty() {
                res.push(buf.iter().map(|&s| s).collect());
            }
            return;
        }
    };

    let mapping = map[current];
    for i in 0..mapping.len() {
        buf.push(&mapping[i..=i]);
        recursive(&s[1..], res, buf, map);
        buf.pop();
    }
}

#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert( $key, $val ); )*
            map
        }
    };
}


// 队列
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let phone: &'static [&'static str] = &["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back("".to_string());

        for digit in digits.chars() {
            let phone_letters = phone[(digit as u8 - b'2') as usize];
            let queue_len = queue.len();
            for _ in 0..queue_len {
                let mut tmp = queue.pop_front().unwrap();
                for letter in phone_letters.chars() {
                    let mut str = tmp.clone();
                    str.push(letter);
                    queue.push_back(str);
                }
                tmp.clear();
            }
        }

        queue.into_iter().collect::<Vec<String>>()
    }
}