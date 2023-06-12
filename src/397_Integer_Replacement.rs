// 397_Integer_Replacement
// https://leetcode.cn/problems/integer-replacement/

use std::collections::HashMap;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut memo = HashMap::new();
        if n == 1 {
            return 0;
        }
        if let Some(&result) = memo.get(&n) {
            return result;
        }
        let result = if n % 2 == 0 {
            1 + Self::integer_replacement(n / 2)
        } else {
            2 + std::cmp::min(
                Self::integer_replacement(n / 2),
                Self::integer_replacement(n / 2 + 1),
            )
        };
        memo.insert(n, result);

        result
    }
}


// 超时
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0;n + 1];
        for i in 2..=n {
            if i & 1 == 0 {
                // i 是偶数
                dp[i] = dp[i / 2] + 1;
            }else {
                // i 是奇数
                dp[i] = (dp[(i + 1) / 2] + 1).min(dp[i - 1]) + 1;
            }
        }
        return dp[n];
    }
}



impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        //懒得管边界了
        if n == i32::MAX{
            return 32;
        }
        if n == 1 {
            return 0;
        }
        if n & 1 == 0 {
            // n 是偶数
            return Self::integer_replacement(n >> 1) + 1;
        } else {
            return Self::integer_replacement(n + 1).min(Self::integer_replacement(n - 1)) + 1;
        }
    }
}


impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        use std::collections::VecDeque;
        let mut fifo = VecDeque::with_capacity(1024);
        fifo.push_back((n as i64, 0));
        while let Some((t, cnt)) = fifo.pop_front() {
            if t == 1 { return cnt }
            if t & 1 == 0 {
                fifo.push_back((t >> 1, cnt + 1));
            } else {
                fifo.push_back((t + 1, cnt + 1));
                fifo.push_back((t - 1, cnt + 1));
            }
        }
        -1
    }
}


impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n;
        let mut ans = 0;
        while n > 1 {
            if n == 3 {
                return ans + 2;
            }
            ans += match n & 3 {
                0 => {
                    n >>= 2;
                    2
                }
                1 => {
                    n >>= 2;
                    3
                }
                2 => {
                    n >>= 1;
                    1
                }
                _ => {
                    n = (n >> 2) + 1;
                    3
                }
            };
        }
        ans
    }
}



/// self  和 Self 的异同
/// 在 Rust 中，self 和 Self 有特定的含义和用途。它们之间的主要区别如下：

/// self: self 是一个实例方法的第一个参数，表示方法的调用者。
/// 当你在方法内部使用 self 时，你可以访问和修改实例的字段以及调用其他实例方法。self 可以有以下几种形式：
    // &self: 这表示一个对调用者的不可变引用，意味着你不能修改实例的字段。它通常用于访问实例的字段或调用其他不可变方法。
    // &mut self: 这表示一个对调用者的可变引用，允许你修改实例的字段。它通常用于修改实例的状态或调用其他可变方法。
    // self: 这表示调用者的所有权被移动到方法中。这种形式不常用，主要用于需要消耗实例或转移所有权的情况。

/// Self: Self 是一个类型别名，表示实现当前 trait 或方法的类型。
/// 它通常用于返回值类型、泛型约束或作为其他类型的一部分。
/// 这里有一些使用 Self 的示例：
    // 作为返回值类型：当一个方法需要返回实现 trait 的类型的实例时，可以使用 Self 作为返回值类型。
    // 在泛型约束中：当你需要将一个类型参数约束为实现某个 trait 时，可以使用 Self 作为约束。

struct Foo {
    value: i32,
}

impl Foo {
    // &self 表示一个不可变引用
    fn get_value(&self) -> i32 {
        self.value
    }

    // &mut self 表示一个可变引用
    fn increment(&mut self) {
        self.value += 1;
    }

    // self 表示所有权被移动
    fn consume(self) -> i32 {
        self.value
    }

    // Self 作为返回值类型
    fn new(value: i32) -> Self {
        Foo { value }
    }
}