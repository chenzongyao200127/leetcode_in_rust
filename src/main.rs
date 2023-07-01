pub fn add_operators(num: String, target: i32) -> Vec<String> {
    let num = num.as_bytes();
    let mut ans = Vec::new();
    
    fn dfs(num: &[u8], target: i32, left: usize, prev_num: i64, cur_ans: i64, expression: &mut String, ans: &mut Vec<String>) {
        if left == num.len() {
            if cur_ans == target as i64 {
                ans.push(expression.clone());
            }
            return;
        }

        let mut cur_num = 0 as i64;
        let start_len = expression.len();

        for i in left..num.len() {
            if i != left && num[left] == b'0' {
                break;
            }


            cur_num = cur_num as i64 * 10 + (num[i] - b'0') as i64;

            if expression.len() == 0 {
                expression.push_str(&cur_num.to_string());
                dfs(num, target, i + 1, cur_num, cur_num, expression, ans);
                expression.truncate(start_len);
                continue;
            }

            if (String::from_utf8(num[left..].to_vec()).unwrap().parse::<i64>().unwrap() as i32) >= ((target as i64 - cur_ans as i64) as i32).abs() {
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


fn main() {
    println!("ans: {:?}", add_operators("2147483648".to_string(),-2147483648))
}