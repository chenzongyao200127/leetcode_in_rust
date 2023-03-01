// 29. Divide Two Integers
// https://leetcode.cn/problems/divide-two-integers/



impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let ans = Self::d(dividend as i64, divisor as i64);
        if ans > i32::MAX as i64 {
            i32::MAX
        } else if ans < i32::MIN as i64 {
            i32::MIN
        } else {
            ans as i32
        }
    }

    pub fn d(dividend: i64, divisor: i64) -> i64 {
        // 符号位， true表示为正数
        let is_na = dividend.signum() == divisor.signum();

        let (mut dividend, divisor) = (dividend.abs(), divisor.abs());

        if dividend < divisor {
            return 0;
        }

        let mut start = divisor;
        let mut ans = 1;
        while dividend > start + start {
            ans += ans;
            start += start;
        }

        ans += Self::d(dividend - start, divisor);

        if !is_na {
            ans = 0 - ans;
        }

        ans
    }
}



