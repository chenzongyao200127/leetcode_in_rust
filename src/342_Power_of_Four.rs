// 342. Power of Four
// https://leetcode.cn/problems/power-of-four/

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut n = n;
        while n != 0 && n%4 == 0 {
            n /= 4;
        }

        n == 1
    }
}


impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n == 0 {
            return false;
        } 
        if n == 1 {
            return true;
        }

        if n % 4 == 0 {
            return Solution::is_power_of_four(n/4);
        } else {
            return false;
        }
    }
}