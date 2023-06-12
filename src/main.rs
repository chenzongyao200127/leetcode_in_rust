

use std::collections::HashMap;

pub fn integer_replacement(n: i32) -> i32 {
    let mut memo = HashMap::new();
    if n == 1 {
        return 0;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = if n % 2 == 0 {
        1 + integer_replacement(n / 2)
    } else {
        2 + std::cmp::min(
            integer_replacement(n / 2),
            integer_replacement(n / 2 + 1),
        )
    };
    memo.insert(n, result);

    result
}

fn main() {

}
