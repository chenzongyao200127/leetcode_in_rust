// 1015_Smallest_Integer_Divisible_by_K
// https://leetcode.cn/problems/smallest-integer-divisible-by-k/


impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let mut temp = 1;
        let mut len = 1;

        while temp % k != 0 {
            temp = temp % k;
            temp = temp * 10 + 1;
            len += 1;
        }
        
        len as i32
    }
}

