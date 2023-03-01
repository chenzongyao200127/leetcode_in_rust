// 326. Power of Three
// https://leetcode.cn/problems/power-of-three/

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 { return false }
        if n == 1 {
            return true
        }
        if n%3 == 0 {
            return Solution::is_power_of_three(n/3);
        } else {
            return false;
        }
    }
}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        const _3_POWER_19: i32 = 1162261467;
        n > 0 && _3_POWER_19 % n == 0
    }
}

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n != 0 && n%3 == 0 {
            n /= 3;
        }
        return n == 1
    }
}