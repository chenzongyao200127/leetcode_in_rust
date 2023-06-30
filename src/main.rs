
pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; colsum.len()]; 2];
    let mut upper_remaining = upper;
    let mut lower_remaining = lower;

    for i in 0..colsum.len() {
        if colsum[i] == 2 {
            matrix[0][i] = 1;
            matrix[1][i] = 1;
            upper_remaining -= 1;
            lower_remaining -= 1;
        }
    }

    for i in 0..colsum.len() {
        if colsum[i] == 1 {
            if upper_remaining > 0 {
                matrix[0][i] = 1;
                upper_remaining -= 1;
            } else {
                matrix[1][i] = 1;
                lower_remaining -= 1;
            }
        }
    }

    if upper_remaining == 0 && lower_remaining == 0 {
        matrix
    } else {
        vec![]
    }
}

// pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut matrix = vec![vec![0; colsum.len()]; 2];
//     for i in 0..colsum.len() {
//         if colsum[i] == 2 {
//             matrix[0][i] = 1;
//             matrix[1][i] = 1;    
//         }
//     }

//     dfs(0, upper, lower, &colsum, &mut matrix);
//     return matrix
// }

// pub fn dfs(cur_col: usize, upper: i32, lower: i32, colsum: &Vec<i32>, matrix: &mut Vec<Vec<i32>>) {
//     if cur_col == colsum.len() {
//         if colsum[cur_col] == 1 {
//             matrix[0][cur_col] = 1;
//             if is_valid(upper, lower, matrix) {
//                 return
//             }
//             matrix[0][cur_col] = 0;
//             matrix[1][cur_col] = 0;
//             if is_valid(upper, lower, matrix) {
//                 return
//             }
//         } else {
//             if is_valid(upper, lower, matrix) {
//                 return
//             }
//         }
//     }
//     for i in 0..=1 {
//         matrix[0][cur_col] = 1;
//         let mut next_col = i+1;
//         while next_col < colsum.len() && colsum[next_col] != 1 {
//             next_col += 1;
//         }
//         dfs(next_col, upper, lower, colsum, matrix);
//         matrix[0][cur_col] = 0;
//     }
// }

// pub fn is_valid(upper: i32, lower: i32, matrix: &Vec<Vec<i32>>) -> bool {
//     if matrix[0].iter().sum() == upper && matrix[1].iter().sum() == lower {
//         return true;
//     }
//     false
// }

pub fn is_circular_sentence(sentence: String) -> bool {
    let words = sentence.split(' ').collect::<Vec<&str>>();
    if sentence.starts_with(' ') || sentence.ends_with(' ') {
        return false;
    }
    if sentence.chars().into_iter().filter(|ch| !(ch.is_alphabetic() || ch.is_whitespace())).count() != 0 {
        return  false;
    }
    for i in 0..words.len() {
        // println!("{:?}", &words[i][words[i].len()-1..words[i].len()]);
        // println!("{:?}", &words[(i+1+words.len())%words.len()][0..1]);
        if &words[i][words[i].len()-1..words[i].len()] != &words[(i+1+words.len())%words.len()][0..1] {
            return false;
        }
    }
    true
}

// 有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。

// 例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
// 给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。
// 你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut ip = vec![];
    let mut ans: Vec<String> = vec![];
    let s: Vec<char> = s.chars().collect();
    dfs(&mut ip, &s, &mut ans, 0);

    ans
}

pub fn dfs(ip: &mut Vec<String>, s: &Vec<char>, ans: &mut Vec<String>, idx: usize) {
    if idx == s.len() && ip.len() == 4 {
        ans.push(ip.clone().join("."));
    } else if idx >= s.len() || ip.len() >= 4 {
        return;
    }
    // println!("{:?}", ip);
    if idx >= s.len() {
        return;
    }
    if s[idx] == '0' {
        ip.push("0".to_string());
        dfs(ip, s, ans, idx+1);
        ip.pop();
    } else {
        let num = (s[idx] as u8 - '0' as u8) as usize;
        ip.push(num.to_string());
        dfs(ip, s, ans, idx+1);
        ip.pop();
    }
    
    if idx + 1 < s.len() {
        let num = (s[idx] as u8 - '0' as u8) as usize * 10 + (s[idx + 1] as u8 - '0' as u8) as usize;
        // println!("{:?}", num);
        ip.push(num.to_string());
        dfs(ip, s, ans, idx+2);
        ip.pop();
    }

    if idx + 2 < s.len() {
        let num = (s[idx] as u8 - '0' as u8) as usize * 100  + (s[idx + 1] as u8 - '0' as u8) as usize * 10 
            + (s[idx + 2] as u8 - '0' as u8) as usize;
        // println!("{:?}", num);
        if num >= 100 && num <= 255 {
            ip.push(num.to_string());
            dfs(ip, s, ans, idx+3);
            ip.pop();
        }
    }
}

fn main() {
    println!("ans: {:?}", restore_ip_addresses("19216844".to_string()))
}