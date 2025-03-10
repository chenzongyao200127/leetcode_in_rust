// 2269_找到一个数字的_K_美丽值
// https://leetcode.cn/problems/find-the-k-beauty-of-a-number/description/?envType=daily-question&envId=2025-03-10
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let s = num.to_string();
        let n = s.len();
        let mut res = 0;
        let k = k as usize;

        for i in 0..=n - k {
            if let Ok(tmp) = s[i..i + k].parse::<i32>() {
                if tmp != 0 && num % tmp == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}
