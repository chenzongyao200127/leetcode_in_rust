// 263. Ugly Number
// https://leetcode.cn/problems/ugly-number/

// 丑数 就是只包含质因数 2、3 和 5 的正整数。
impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n==0 {
            return false;
        }
        while n%2==0{
            n/=2;
        }
        while n%3==0{
            n/=3;
        }
        while n%5==0{
            n/=5;
        }
        n==1
    }
}

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let uglys = vec![1,2,3,5];
        if uglys.contains(&n) {
            return true
        }
        if n == 0 {
            return false;
        }
        
        if n%2 == 0 {
            return Solution::is_ugly(n/2);
        } else if n%3 == 0 {
            return Solution::is_ugly(n/3);
        } else if n%5 == 0 {
            return Solution::is_ugly(n/5);
        } else {
            false
        }
    }
    }