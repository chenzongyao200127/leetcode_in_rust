// 3340_检查平衡字符串
// https://leetcode.cn/problems/check-balanced-string/description/?envType=daily-question&envId=2025-03-14

// 给你一个仅由数字 0 - 9 组成的字符串 num。如果偶数下标处的数字之和等于奇数下标处的数字之和，则认为该数字字符串是一个 平衡字符串。
// 如果 num 是一个 平衡字符串，则返回 true；否则，返回 false。
impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let (even_sum, odd_sum) = num.chars().enumerate().fold((0, 0), |(even, odd), (i, c)| {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                (even + digit, odd)
            } else {
                (even, odd + digit)
            }
        });
        even_sum == odd_sum
    }
}
