// 461. Hamming Distance
// https://leetcode.cn/problems/hamming-distance/

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x^y).count_ones() as i32
    }
}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut max = 0;
        while z > 0 {
            if z % 2 == 1 {
                max += 1;
            }
            z = z >> 1;
        }
        return max;
    }
}

impl Solution {
    pub fn hamming_distance1(x: i32, y: i32) -> i32 {
        let mut a = x ^ y;
        let mut r = 0;

        while a > 0 {
            if a & 1 == 1 {
                r += 1;
            }

            a >>= 1;
        }

        r
    }
    
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let (mut s, mut r) = (x ^ y, 0);
        while s > 0 {
            s = s & (s - 1);
            r += 1;
        }

        r
    }
}