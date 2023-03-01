// 171. Excel Sheet Column Number
// https://leetcode.cn/problems/excel-sheet-column-number/

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ans = 0;
        let s: Vec<char> = column_title.chars().rev().collect();
        for i in 0..s.len() {
            let mut c_num = (s[i] as u8 - 'A' as u8 + 1) as i32;
            for _ in 0..i {
                c_num *= 26;
            }
            ans += c_num;
        }

        ans
    }
}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ans: i32 = 0;
        let mut exp: u32 = 0;

        for ele in column_title.bytes().rev() {
            ans += (ele - b'A' + 1) as i32 * 26_i32.pow(exp);
            exp += 1;
        }

        ans
    }
}
