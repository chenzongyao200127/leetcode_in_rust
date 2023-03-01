// 168. Excel Sheet Column Title
// https://leetcode.cn/problems/excel-sheet-column-title/

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut ans = "".to_string();
        let mut n = column_number;
        while n > 0 {
            let pop = (n-1) % 26;
            n = (n-1) / 26;
            ans.push(('A' as u8 + pop as u8) as char);
        }

        ans.chars().rev().collect::<String>()
    }
}

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut ans = String::new();
        let mut num = column_number;
        while num > 0 {
            let last = (num - 1) % 26;
            let c = ('A' as u8) + (last as u8);
            ans.push(c as char);
            num = (num - 1) / 26;
        }
        return ans.chars().rev().collect();
    }
}