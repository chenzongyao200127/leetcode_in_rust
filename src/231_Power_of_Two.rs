// 231. Power of Two
// https://leetcode.cn/problems/power-of-two/

impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        return (n > 0) && (n & n-1 == 0)
    }
}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n==1{
            return true;
        }
        if n==0||(n&1)!=0{
            return false;
        }
        return Self::is_power_of_two(n/2);
    }
}
