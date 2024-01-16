use std::collections::HashMap;

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let digits = n.to_string().chars().collect::<Vec<char>>();
        let mut memo = HashMap::new();
        Self::dp(0, 0, true, false, &digits, &mut memo)
    }

    fn dp(
        index: usize,
        used_digits_mask: i32,
        is_within_limit: bool,
        has_started: bool,
        digits: &Vec<char>,
        memo: &mut HashMap<(usize, i32, bool, bool), i32>,
    ) -> i32 {
        if index == digits.len() {
            return if has_started { 1 } else { 0 };
        }

        if let Some(&cached) = memo.get(&(index, used_digits_mask, is_within_limit, has_started)) {
            return cached;
        }

        let mut total_count = 0;
        if !has_started {
            total_count += Self::dp(index + 1, used_digits_mask, false, false, digits, memo);
        }

        let start_digit = if !has_started { 1 } else { 0 };
        let end_digit = if is_within_limit {
            digits[index].to_digit(10).unwrap() as i32
        } else {
            9
        };

        for digit in start_digit..=end_digit {
            if used_digits_mask & (1 << digit) == 0 {
                let new_mask = used_digits_mask | (1 << digit);
                let still_within_limit = is_within_limit && digit == end_digit as i32;
                total_count +=
                    Self::dp(index + 1, new_mask, still_within_limit, true, digits, memo);
            }
        }

        memo.insert(
            (index, used_digits_mask, is_within_limit, has_started),
            total_count,
        );
        total_count
    }
}

fn main() {
    let result = Solution::count_special_numbers(100);
    println!("Number of special numbers: {}", result);
}
