// 2182_Construct_String_With_Repeat_Limit
// https://leetcode.cn/problems/construct-string-with-repeat-limit/description/

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        const ASCII_LOWER_A: u8 = b'a';
        const ASCII_LOWER_Z: u8 = b'z';
        let mut frequency = [0; 256];

        let input = s.as_bytes();

        // Count the frequency of each character in the input string
        for &byte in input {
            frequency[byte as usize] += 1;
        }

        let mut result = String::new();
        let mut last_char = 0;
        let mut repeat_count = 0;

        for _ in 0..input.len() {
            for char_code in (ASCII_LOWER_A..=ASCII_LOWER_Z).rev() {
                if frequency[char_code as usize] == 0 {
                    continue;
                }

                if char_code == last_char {
                    if repeat_count >= repeat_limit {
                        continue;
                    }
                    repeat_count += 1;
                } else {
                    repeat_count = 1;
                    last_char = char_code;
                }

                frequency[char_code as usize] -= 1;
                result.push(char_code as char);
                break;
            }
        }

        result
    }
}
