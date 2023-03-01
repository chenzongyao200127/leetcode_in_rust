// 357. Count Numbers with Unique Digits
// https://leetcode.cn/problems/count-numbers-with-unique-digits/

// 357. 统计各位数字都不同的数字个数
// 给你一个整数 n ，统计并返回各位数字都不同的数字 x 的个数，其中 0 <= x < 10^n 。
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 { return 1; }
        if n == 1 { return 10; }
        
        let mut tmp = 9;
        let mut ans = 9;
        for _ in 0..n-1 {
            ans *= tmp;
            tmp -= 1;
        }

        ans + Solution::count_numbers_with_unique_digits(n-1)
    }
}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut ret = 1;
        for len in 1..=n {
            let mut cnt = 9;
            let mut x = 9;
            for _ in 1..len {
                cnt *= x;
                x -= 1;
            }
            ret += cnt;
        }
        ret
    }
}