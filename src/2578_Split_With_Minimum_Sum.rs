// 2578_最小和分割
// https://leetcode.cn/problems/split-with-minimum-sum/description/

// 给你一个正整数 num ，请你将它分割成两个非负整数 num1 和 num2 ，满足：

// num1 和 num2 直接连起来，得到 num 各数位的一个排列。
// 换句话说，num1 和 num2 中所有数字出现的次数之和等于 num 中所有数字出现的次数。
// num1 和 num2 可以包含前导 0 。
// 请你返回 num1 和 num2 可以得到的和的 最小 值。

// 注意：

// num 保证没有前导 0 。
// num1 和 num2 中数位顺序可以与 num 中数位顺序不同。

// 10 <= num <= 109

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut nums: Vec<char> = num.to_string().chars().filter(|&x| x != '0').collect();
        nums.sort();

        let mut n1 = 0;
        let mut n2 = 0;
        
        for (i, n) in nums.into_iter().enumerate() {
            let n = n.to_digit(10).unwrap() as i32;
            if i % 2 == 0 {
                n1 = n1 * 10 + n;
            } else {
                n2 = n2 * 10 + n;
            }
        }
        
        n1 + n2
    }
}
