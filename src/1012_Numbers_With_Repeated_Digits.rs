// 1012_Numbers_With_Repeated_Digits
// https://leetcode.cn/problems/numbers-with-repeated-digits/

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            let mut digits = Vec::new();
            let mut num = i;
            while num > 0 {
                digits.push(num % 10);
                num /= 10;
            }
            if digits.len() != digits.iter().collect::<std::collections::HashSet<_>>().len() {
                count += 1;
            }
        }
        count
    }
}