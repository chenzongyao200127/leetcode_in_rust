// 693. Binary Number with Alternating Bits
// https://leetcode.cn/problems/binary-number-with-alternating-bits/

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut pre = if 1 == n & 1 { 0 } else { 1 };
        let mut n = n;
        while n != 0 {
            if (n & 1) != 1-pre {
                return false;
            }
            pre = n & 1;
            n >>= 1;
        }

        true
    }
}


impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = n + (n >> 1);
        x & (x + 1) == 0
    }
}