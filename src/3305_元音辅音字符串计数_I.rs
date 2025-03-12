// 3305_元音辅音字符串计数_I
// https://leetcode.cn/problems/count-of-substrings-containing-every-vowel-and-k-consonants-i/description/?envType=daily-question&envId=2025-03-12

// 给你一个字符串 word 和一个 非负 整数 k。
// 返回 word 的 子字符串 中，每个元音字母（'a'、'e'、'i'、'o'、'u'）至少 出现一次，并且 恰好 包含 k 个辅音字母的子字符串的总数。
impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let k = k as usize;
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        let mut res = 0;

        for i in 0..=word.len() - 5 - k {
            let mut vowel_count = [0; 5];
            let mut consonant_count = 0;

            for j in i..word.len() {
                let c = word.as_bytes()[j];
                if vowels.contains(&c) {
                    vowel_count[vowels.iter().position(|&v| v == c).unwrap()] += 1;
                } else {
                    consonant_count += 1;
                }

                if consonant_count > k {
                    break;
                }

                if vowel_count.iter().all(|&count| count > 0) && consonant_count == k {
                    res += 1;
                }
            }
        }

        res
    }
}
