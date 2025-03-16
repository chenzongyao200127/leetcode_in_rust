// 2272_最大波动的子字符串
// https://leetcode.cn/problems/substring-with-largest-variance/description/?envType=daily-question&envId=2025-03-16

// 字符串的 波动 定义为子字符串中出现次数 最多 的字符次数与出现次数 最少 的字符次数之差。
// 给你一个字符串 s ，它只包含小写英文字母。请你返回 s 里所有 子字符串的 最大波动 值。
// 子字符串 是一个字符串的一段连续字符序列。

// * Think about how to solve the problem if the string had only two distinct characters.
// * If we replace all occurrences of the first character by +1 and those of the second character by -1, can we efficiently calculate the largest possible variance of a string with only two distinct characters? -> prefix sum
// * Now, try finding the optimal answer by taking all possible pairs of characters into consideration.

use std::cmp::max;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        // TODO: Implement the solution
    }
}
