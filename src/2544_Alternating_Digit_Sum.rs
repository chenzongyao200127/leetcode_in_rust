// 2544_Alternating_Digit_Sum
// https://leetcode.cn/problems/alternating-digit-sum/


// 给你一个正整数 n 。n 中的每一位数字都会按下述规则分配一个符号：

// 最高有效位 上的数字分配到 正 号。
// 剩余每位上数字的符号都与其相邻数字相反。
// 返回所有数字及其对应符号的和。
impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let n = n.to_string();
        let mut ans = 0;
        let mut flag = 1;
        n.chars().into_iter().for_each(|ch| {
            ans += (ch as u8 - '0' as u8) as i32 * flag;
            flag *= -1;
        });
        ans
    }
}

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        n.to_string().chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .enumerate()
        .fold(0, |acc, (idx, x)| {
            if idx % 2 == 0 { acc + x }
            else {acc - x}
        })
    }
}
