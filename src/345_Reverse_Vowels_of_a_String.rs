// 345_Reverse_Vowels_of_a_String
// https://leetcode.cn/problems/reverse-vowels-of-a-string/

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = vec!['a','e','i','o','u'];
        let mut i = 0;
        let mut j = s.len()-1;
        let mut s_chars: Vec<char> = s.chars().collect();
        while i < j {
            while !vowels.contains(s_chars[i]) {
                i += 1;
            }
            while !vowels.contains(s_chars[j]) {
                j -= 1;
            }
            s
        }
    }
}