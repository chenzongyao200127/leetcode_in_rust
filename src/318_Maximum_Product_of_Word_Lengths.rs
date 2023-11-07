// 318_Maximum_Product_of_Word_Lengths
// https://leetcode.cn/problems/maximum-product-of-word-lengths/description/



// Too Slow
use std::collections::HashSet;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        #[inline] 
        fn str_to_set(str: &str) -> HashSet<char> {
            let mut set = HashSet::new();

            for c in str.chars() {
                set.insert(c);
            }

            set
        }

        let words_sets = words.iter().map(|s| {
            str_to_set(&s)
        }).collect::<Vec<_>>();

        let mut ans = 0;
        for i in 0..words_sets.len()-1 {
            for j in i+1..words_sets.len() {
                let intersection: HashSet<_> = words_sets[i].intersection(&words_sets[j]).collect();
                if intersection.is_empty() {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }

        ans as i32
    }
}




use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut word_sets: Vec<HashSet<char>> = Vec::new();
        let mut word_masks: Vec<u32> = Vec::new();

        for word in &words {
            let mut set = HashSet::new();
            let mut mask = 0u32;

            for c in word.chars() {
                set.insert(c);
                mask |= 1 << (c as u8 - b'a');
            }

            word_sets.push(set);
            word_masks.push(mask);
        }

        let mut max_product = 0;

        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if word_masks[i] & word_masks[j] == 0 {
                    let product = words[i].len() * words[j].len();
                    max_product = max_product.max(product);
                }
            }
        }

        max_product as i32
    }
}



impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mask: Vec<i32> = words
            .iter()
            .map(|word| {
                word.chars()
                    .fold(0, |acc, c| acc | 1 << (c as u8 - 'a' as u8))
            })
            .collect();
        
        let mut ans = 0;
        for i in 0..mask.len() {
            for j in i + 1..mask.len() {
                if mask[i] & mask[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }
        ans as i32
    }
}