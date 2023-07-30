
// 1170_Compare_Strings_by_Frequency_of_the_Smallest_Character
// https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character/

impl Solution {
    fn min_char_freq(s: &str) -> usize {
        let min_char = s.chars().min().unwrap();
        s.chars().filter(|&c| c == min_char).count()
    }

    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let words_freq: Vec<usize> = words.iter().map(|word| Self::min_char_freq(word)).collect();
        let mut answer: Vec<i32> = Vec::new();

        for query in queries {
            let query_freq = Self::min_char_freq(&query);
            let count = words_freq.iter().filter(|&&word_freq| query_freq < word_freq).count();
            answer.push(count as i32);
        }

        answer
    }
}