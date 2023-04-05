// # 2427_Number_of_Common_Factors
// # https://leetcode.cn/problems/number-of-common-factors/

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=a.min(b) as usize {
            if a as usize % i == 0 && b as usize % i == 0 {
                ans += 1;
            }
        }
        
        ans
    }
}



use std::cmp::min;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let min_val = min(a, b);
        let mut result = 0;

        for i in 1..(min_val + 1) {
            if a % i == 0 && b % i == 0 {
                result += 1;
            }
        }

        result
    }
}