// 828_统计子串中的唯一字符
// https://leetcode.cn/problems/count-unique-characters-of-all-substrings-of-a-given-string/description/

// 贡献法
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut last_position = vec![-1; 26];
        let mut contribution = vec![0; 26];
        let mut total = 0;
        let mut curr_sum = 0;

        for (i, ch) in s.chars().enumerate() {
            let idx = (ch as u8 - b'A') as usize;
            curr_sum -= contribution[idx];
            contribution[idx] = i as i32 - last_position[idx];
            curr_sum += contribution[idx];
            total += curr_sum;
            last_position[idx] = i as i32;
        }

        total
    }
}