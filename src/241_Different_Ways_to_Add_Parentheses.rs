// 241. 为运算表达式设计优先级
// https://leetcode.cn/problems/different-ways-to-add-parentheses/

// 给你一个由数字和运算符组成的字符串 expression ，按不同优先级组合数字和运算符，计算并返回所有可能组合的结果。你可以 按任意顺序 返回答案。
// 生成的测试用例满足其对应输出值符合 32 位整数范围，不同结果的数量不超过 104 。

use std::collections::HashMap;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut memo: HashMap<String, Vec<i32>> = HashMap::new();
        Self::compute_with_memo(&expression, &mut memo)
    }

    fn compute_with_memo(expression: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(cached_result) = memo.get(expression) {
            return cached_result.clone();
        }

        let mut res = vec![];
        let express_char_vec: Vec<char> = expression.chars().collect();
        for i in 0..expression.len() {
            let op = express_char_vec[i];
            if op == '+' || op == '-' || op == '*' {
                let left_res = Self::compute_with_memo(&expression[..i], memo);
                let right_res = Self::compute_with_memo(&expression[i+1..], memo);
                for x in left_res.iter() {
                    for y in right_res.iter() {
                        match op {
                            '+' => res.push(x + y),
                            '-' => res.push(x - y),
                            _  => res.push(x * y),
                        }
                    }
                }
            }
        }

        if res.is_empty() {
            res.push(expression.parse::<i32>().unwrap())
        }

        memo.insert(expression.to_string(), res.clone());
        res
    }
}
