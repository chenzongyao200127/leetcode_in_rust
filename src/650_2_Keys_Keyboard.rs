// 650_2_Keys_Keyboard
// https://leetcode.cn/problems/2-keys-keyboard/


// 最初记事本上只有一个字符 'A' 。你每次可以对这个记事本进行两种操作：

// Copy All（复制全部）：复制这个记事本中的所有字符（不允许仅复制部分字符）。
// Paste（粘贴）：粘贴 上一次 复制的字符。
// 给你一个数字 n ，你需要使用最少的操作次数，在记事本上输出 恰好 n 个 'A' 。返回能够打印出 n 个 'A' 的最少操作次数。

// 质因素分解
impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let mut ans = 0;
        for i in 2..=n {
            if i * i > n {
                break;
            }
            while n % i == 0 {
                n = n / i as i32;
                ans += i as i32;
            }
        }
        if n > 1 {
            return ans + n
        }
        ans
    }
}