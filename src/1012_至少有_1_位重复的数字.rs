// 1012_至少有_1_位重复的数字
// https://leetcode.cn/problems/numbers-with-repeated-digits/

// 给定正整数 n，返回在 [1, n] 范围内具有 至少 1 位 重复数字的正整数的个数。

// 示例 1：
// 输入：n = 20
// 输出：1
// 解释：具有至少 1 位重复数字的正数（<= 20）只有 11 。

// 示例 2：
// 输入：n = 100
// 输出：10
// 解释：具有至少 1 位重复数字的正数（<= 100）有 11，22，33，44，55，66，77，88，99 和 100 。

// 示例 3：
// 输入：n = 1000
// 输出：262

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let n_str = n.to_string(); // 将数字n转化为字符串
        let len = n_str.len();
        
        // 数位DP函数，用于计算不包含重复数字的数的数量。
        fn count_numbers_without_repeated_digits(
            index: usize,
            used_digits_mask: i32,
            is_limit: bool,
            has_started: bool,
            n_str: &str,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
        ) -> i32 {
            if index == n_str.len() {
                // 当前已经处理了所有的数位
                return if has_started { 1 } else { 0 };
            }

            // 使用备忘录技巧来优化重复的子问题
            if memo[index][used_digits_mask as usize][is_limit as usize][has_started as usize] != -1 {
                return memo[index][used_digits_mask as usize][is_limit as usize][has_started as usize];
            }

            let mut total_count = 0;

            // 当前数位还没开始的情况
            if !has_started {
                total_count = count_numbers_without_repeated_digits(index + 1, used_digits_mask, false, false, &n_str, memo);
            }

            let start_digit = if has_started { 0 } else { 1 }; // 确定当前数位的起始值
            let end_digit = if is_limit { n_str.chars().nth(index).unwrap().to_digit(10).unwrap() as i32 } else { 9 }; // 确定当前数位的结束值

            // 遍历当前数位的所有可能的数值
            for digit in start_digit..=end_digit {
                if used_digits_mask >> digit & 1 == 0 { // 检查数字是否已被使用过
                    total_count += count_numbers_without_repeated_digits(index + 1, used_digits_mask | (1 << digit), is_limit && digit == end_digit, true, &n_str, memo);
                }
            }

            // 保存计算的结果到备忘录
            memo[index][used_digits_mask as usize][is_limit as usize][has_started as usize] = total_count;
            
            total_count
        }

        // 初始化备忘录
        let mut memo = vec![vec![vec![vec![-1; 2]; 2]; 1024]; len];

        n - count_numbers_without_repeated_digits(0, 0, true, false, &n_str, &mut memo)
    }
}