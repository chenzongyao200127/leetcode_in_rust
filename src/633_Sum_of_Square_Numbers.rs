// 633. Sum of Square Numbers
// https://leetcode.cn/problems/sum-of-square-numbers/

// floor(): Returns the largest integer less than or equal to self.
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let upper = (c as f64 / 2.0 ).sqrt().floor() as i32;

        for i in 0..=upper{
            let b = c - i * i;
            let sqrt_b = (b as f64).sqrt() as i32;
            if sqrt_b * sqrt_b == b {
                return true;
            }
        }

        false
    }
}


impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0 as i128;
        let mut b = (c as f64).sqrt().ceil() as i128;
        let mut total = a*a + b*b;
        while a <= b {
            if total == c as i128 { 
                return true; 
            } else if total < c as i128  {
                a += 1;
            } else {
                b -= 1;
            }
            total = a*a + b*b;
        } 

        false
    }
}