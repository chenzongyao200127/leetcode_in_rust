// 172. Factorial Trailing Zeroes
// https://leetcode.cn/problems/factorial-trailing-zeroes/

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..n+1 {
            ans += count_factor_5(i);
        }

        ans
    }
}

pub fn count_factor_5 (n: i32) -> i32 {
    let mut ans = 0;
    if n % 5 != 0 {
        return 0;
    } else {
        ans += 1;
        ans += count_factor_5(n/5);
    }

    ans 
}


impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut ans = 0;
        while n > 0 {
            n /= 5;
            ans += n;
        }
        ans
    }
}