// 2485_Find_the_Pivot_Integer
// https://leetcode.cn/problems/find-the-pivot-integer/

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        if n == 1 {
            return 1
        }
        let mut pre_sum = 0 as i32;
        let mut suf_sum = n as i32;
        for i in 1..=n {
            pre_sum += i as i32;
        }
        for i in (1..=n-1).rev() {
            pre_sum -= i as i32 + 1;
            suf_sum += i as i32;
            if pre_sum == suf_sum {
                return i as i32;
            }
        }
        -1
    }
}



impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut a = 1;
        let mut b = n;
        let mut lo = 1;
        let mut hi = n;
        while lo < hi {
            if a < b {
                lo += 1;
                a += lo;
            } else {
                hi -= 1;
                b += hi;
            }
        }
        if a == b {
            lo
        } else {
            -1
        }
    }
}
