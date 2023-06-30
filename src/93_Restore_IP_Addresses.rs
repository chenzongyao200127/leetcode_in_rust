// 93_Restore_IP_Addresses
// https://leetcode.cn/problems/restore-ip-addresses/

// 有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。

// 例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
// 给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。
// 你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ip = vec![];
        let mut ans: Vec<String> = vec![];
        let s: Vec<char> = s.chars().collect();
        Self::dfs(&mut ip, &s, &mut ans, 0);

        ans
    }

    
    pub fn dfs(ip: &mut Vec<String>, s: &Vec<char>, ans: &mut Vec<String>, idx: usize) {
        if idx == s.len() && ip.len() == 4 {
            ans.push(ip.clone().join("."));
        }
        if idx >= s.len() || ip.len() >= 4 {
            return;
        }
        if s[idx] == '0' {
            ip.push("0".to_string());
            Self::dfs(ip, s, ans, idx+1);
            ip.pop();
        } else {
            let num = (s[idx] as u8 - '0' as u8) as usize;
            ip.push(num.to_string());
            Self::dfs(ip, s, ans, idx+1);
            ip.pop();
        }
        
        if idx + 1 < s.len() {
            let num = (s[idx] as u8 - '0' as u8) as usize * 10 + (s[idx + 1] as u8 - '0' as u8) as usize;
            // println!("{:?}", num);
            if num >= 10 && num <= 99 {
                ip.push(num.to_string());
                Self::dfs(ip, s, ans, idx+2);
                ip.pop();
            }
        }

        if idx + 2 < s.len() {
            let num = (s[idx] as u8 - '0' as u8) as usize * 100  + (s[idx + 1] as u8 - '0' as u8) as usize * 10 
                + (s[idx + 2] as u8 - '0' as u8) as usize;
            // println!("{:?}", num);
            if num >= 100 && num <= 255 {
                ip.push(num.to_string());
                Self::dfs(ip, s, ans, idx+3);
                ip.pop();
            }
        }
    }
}



use std::net::SocketAddr;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s_len = s.len();
        if s_len < 4 || s_len > 12 {
            return vec![];
        }
        let mut result = vec![];

        for i in 1..s_len - 2 {
            for j in i + 1..s_len - 1 {
                for k in j + 1..s_len {
                    let parts = format!("{}.{}.{}.{}:0", &s[..i], &s[i..j], &s[j..k], &s[k..]);
                    if parts.parse::<SocketAddr>().is_ok() {
                        result.push(parts.strip_suffix(":0").unwrap().to_string());
                    }
                }
            }
        }
        result
    }
}


// 可读性优化
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ip = vec![];
        let mut ans: Vec<String> = vec![];
        let s: Vec<char> = s.chars().collect();
        dfs(&mut ip, &s, &mut ans, 0);

        ans
    }
}

fn dfs(ip: &mut Vec<String>, s: &Vec<char>, ans: &mut Vec<String>, idx: usize) {
    if idx == s.len() && ip.len() == 4 {
        ans.push(ip.clone().join("."));
        return;
    }
    if idx >= s.len() || ip.len() >= 4 {
        return;
    }

    for i in 1..=3 {
        if let Some(num) = parse_num(s, idx, i) {
            ip.push(num.to_string());
            dfs(ip, s, ans, idx + i);
            ip.pop();
        }
    }
}

fn parse_num(s: &Vec<char>, idx: usize, len: usize) -> Option<usize> {
    if idx + len > s.len() {
        return None;
    }

    let num = s[idx..idx + len]
        .iter()
        .fold(0, |acc, &c| acc * 10 + (c as u8 - '0' as u8) as usize);

    match len {
        1 => Some(num),
        2 if num >= 10 => Some(num),
        3 if num >= 100 && num <= 255 => Some(num),
        _ => None,
    }
}