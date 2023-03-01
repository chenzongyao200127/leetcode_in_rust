// 166. Fraction to Recurring Decimal
// https://leetcode.cn/problems/fraction-to-recurring-decimal/

use std::collections::HashSet;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut is_neg = false;
        if numerator.signum() != denominator.signum() {
            is_neg = true;
        }

        if numerator == 0 {
            return "0".to_string();
        }

        let mut ans = String::new();
        let mut numerator = numerator.abs() as i128;
        let denominator = denominator.abs() as i128;
        let mut consult = numerator / denominator;
        let mut remainder = numerator % denominator;
        let mut set: HashSet<i128> = HashSet::new();
                    
        if remainder == 0 {
            if is_neg {
                return "-".to_string() + &consult.abs().to_string()
            } else {
                return consult.abs().to_string()
            }
        }

        ans.push_str(&(consult.abs().to_string()));
        ans.push('.');
        numerator = remainder * 10;
        let mut idx = 0;
        let mut map = vec![];
        
        while !set.contains(&numerator) {
            map.push((numerator, idx));
            set.insert(numerator);
            consult = numerator / denominator;
            remainder = numerator % denominator;
            ans.push_str(&(consult.abs().to_string()));
            if remainder == 0 {
                if is_neg {
                    return "-".to_string() + &ans
                } else {
                    return ans
                }
            }
            numerator = remainder * 10;
            idx += 1;
        }


        let mut locat = 0;
        for i in map {
            if i.0 == numerator {
                locat = idx - i.1;
            }
        }
        let mut result = "".to_string();
        if is_neg {
            result.push('-');
        }
        result.push_str(&ans[0..ans.len()-locat]);
        result.push('(');
        result.push_str(&ans[ans.len()-locat..ans.len()]);
        result.push(')');

        result
    }
}

use std::collections::HashMap;

impl Solution {
    // https://github.com/shionryuu/leetcode-rust
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = i64::from(numerator);
        let denominator = i64::from(denominator);

        if numerator == 0 {
            return String::from("0");
        }

        let flag = if numerator / numerator.abs() == denominator / denominator.abs() {
            String::new()
        } else {
            String::from("-")
        };
        let numerator = numerator.abs();
        let denominator = denominator.abs();
        let (divisor, reminder) = (numerator / denominator, numerator % denominator);
        if reminder == 0 {
            return format!("{flag}{}", divisor.to_string());
        }

        let mut decimal = vec![];
        let mut numerator = reminder;
        let mut repeat = HashMap::new();

        while numerator != 0 {
            numerator *= 10;
            if let Some(&idx) = repeat.get(&numerator) {
                decimal.insert(idx, b'(');
                decimal.push(b')');
                break;
            }
            repeat.insert(numerator, decimal.len());

            if numerator >= denominator {
                decimal.push(b'0' + (numerator / denominator) as u8);
                numerator %= denominator;
            } else {
                decimal.push(b'0');
            }
        }

        format!("{flag}{divisor}.{}", String::from_utf8(decimal).unwrap())
    }
}