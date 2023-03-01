// 233. Number of Digit One
// https://leetcode.cn/problems/number-of-digit-one/

// 给定一个整数 n，计算所有小于等于 n 的非负整数中数字 1 出现的个数。
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut digit = 1;
        let mut res = 0;
        let mut high = n / 10;
        let mut cur = n % 10;
        let mut low = 0;
    
        while high != 0 || cur != 0 {
            if cur == 0 {
                res += high * digit;
            } else if cur == 1 {
                res += high * digit + low + 1;
            } else {
                res += (high + 1) * digit;
            }
            low += cur * digit;
            cur = high % 10;
            high /= 10;
            digit *= 10;
        }
    
        res
    }
}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut mulk = 1_i32;
        let mut ans = 0;
        let mut _k = 0;
        while n >= mulk {
            ans += (n / (mulk * 10)) * mulk + mulk.min(0.max(n % (mulk * 10) - mulk + 1));
            _k += 1;
            mulk *= 10;
        }
    
        ans
    }
}

impl Solution {
	pub fn count_digit_one(n: i32) -> i32 {
		fn cnt(x: i32) -> i32 {
			if x <= 0 { 0 } else { x * (u32::pow(10, (x - 1) as u32) as i32) }
		}
		let mut ans = 0;
		let s: Vec<char> = format!("{}", n).chars().collect();
		for i in 0..s.len() {
			ans += (s[i].to_digit(10).unwrap() as i32) * cnt((s.len() - i - 1) as i32);
			if s[i] == '1' {
				ans += if i + 1 < s.len() {
					let mut r = 0;
					for j in i + 1..s.len() {
						r = r * 10 + (s[j].to_digit(10).unwrap() as i32);
					}
					r + 1
				} else { 1 };
			} else if s[i] != '0' {
				ans += u32::pow(10, (s.len() - i - 1) as u32) as i32;
			}
		}
		ans
	}
}

// Out of memory
// impl Solution {
//     pub fn count_digit_one(n: i32) -> i32 {
//         if n == 0 { return 0; }
//         let mut ans = 0;
//         let mut dp = vec![0; n as usize + 1];
//         dp[1] = 1;
//         for i in 0..=n as usize {
//             dp[i] = dp[i/10] + dp[i%10];
//             ans += dp[i];
//         }
    
//         ans
//     }
// }