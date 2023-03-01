// 670. Maximum Swap
// https://leetcode.cn/problems/maximum-swap/

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut s:Vec<char> = num.to_string().chars().collect();
        let mut s_ord = s.clone();
        s_ord.sort_by(|a, b| b.partial_cmp(a).unwrap());
        // println!("{:?}",(s, s_ord));
        let mut old = '0';
        let mut new = '0';
        for i in 0..s.len() {
            if s[i] != s_ord[i] {
                old = s[i];
                new = s_ord[i];
                s[i] = s_ord[i];
                break;
            }
        }
        for i in (0..s.len()).rev() {
            if s[i] == new {
                s[i] = old;
                break;
            }
        }

        s.iter().collect::<String>().parse::<i32>().unwrap()
    }
}


impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let mut max_is = vec![0; num.len()];
        let mut max_i = num.len() - 1;
        let mut max_v = 0;
        for (i, &n) in num.iter().enumerate().rev() {
            max_is[i] = max_i;
            if max_v < n {
                max_v = n;
                max_i = i;
            }
        }
        for i in (0..num.len()) {
            if num[i] < num[max_is[i]] {
                let tmp = num[i];
                num[i] = num[max_is[i]];
                num[max_is[i]] = tmp;
                break;
            }
        }
        num.into_iter().fold(0, |acc, x| { acc * 10 + x}) as i32
    }
}