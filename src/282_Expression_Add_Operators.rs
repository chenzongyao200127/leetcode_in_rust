// 282_Expression_Add_Operators
// https://leetcode.cn/problems/expression-add-operators/submissions/

// 282_Expression_Add_Operators
// https://leetcode.cn/problems/expression-add-operators/submissions/

// 存在bug
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let num = num.as_bytes();
        let mut ans = Vec::new();
        
        fn dfs(num: &[u8], target: i32, left: usize, prev_num: i32, cur_ans: i32, expression: &mut String, ans: &mut Vec<String>) {
            if left == num.len() {
                if cur_ans == target {
                    ans.push(expression.clone());
                }
                return;
            }

            let mut cur_num = 0;
            let start_len = expression.len();

            for i in left..num.len() {
                if i != left && num[left] == b'0' {
                    break;
                }

                cur_num = cur_num * 10 + (num[i] - b'0') as i32;

                if expression.len() == 0 {
                    expression.push_str(&cur_num.to_string());
                    dfs(num, target, i + 1, cur_num, cur_num, expression, ans);
                    expression.truncate(start_len);
                    continue;
                }

                if (String::from_utf8(num[left..].to_vec()).unwrap().parse::<i64>().unwrap() as i32) >= (target - cur_ans).abs() {
                    expression.push_str(&format!("+{}", cur_num));
                    dfs(num, target, i + 1, cur_num, cur_ans + cur_num, expression, ans);
                    expression.truncate(start_len);
                    
                    expression.push_str(&format!("-{}", cur_num));
                    dfs(num, target, i + 1, -cur_num, cur_ans - cur_num, expression, ans);
                    expression.truncate(start_len);
                }

                expression.push_str(&format!("*{}", cur_num));
                dfs(num, target, i + 1, prev_num * cur_num, cur_ans - prev_num + prev_num * cur_num, expression, ans);
                expression.truncate(start_len);
            }
        }
        
        dfs(num, target, 0, 0, 0, &mut String::new(), &mut ans);
        ans
    }
}