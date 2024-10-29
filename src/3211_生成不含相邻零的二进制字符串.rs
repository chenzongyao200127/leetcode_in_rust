// 3211_生成不含相邻零的二进制字符串

// 给你一个正整数 n。
// 如果一个二进制字符串 x 的所有长度为 2 的子字符串中包含 至少 一个 "1"，则称 x 是一个 有效 字符串。
// 返回所有长度为 n 的 有效 字符串，可以以任意顺序排列。

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let max_size = 1 << n;
        let mut result = Vec::new();

        // Generate valid strings
        for i in 0..max_size {
            let mut valid = true;
            for j in 0..(n - 1) {
                if ((i >> j) & 3) == 0 {
                    valid = false;
                    break;
                }
            }

            if valid {
                // Convert number to binary string
                let mut s = String::with_capacity(n as usize);
                for j in 0..n {
                    let bit = (i >> (n - 1 - j)) & 1;
                    s.push(if bit == 1 { '1' } else { '0' });
                }
                result.push(s);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_strings() {
        let n = 2;
        let result = valid_strings(n);
        assert_eq!(result, vec!["11", "10", "01"]);

        let n = 3;
        let result = valid_strings(n);
        assert_eq!(result.len(), 6);
    }
}
