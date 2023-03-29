// 1641_Count_Sorted_Vowel_Strings
// https://leetcode.cn/problems/count-sorted-vowel-strings/

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut ans = 0;
        for ch in vec!['a', 'e', 'i', 'o', 'u'] {
            ans += count(n, ch);
        }
        ans
    }
}

pub fn count(level: i32, ch: char) -> i32 {
    if level == 1 {
        return 1;
    }

    match ch {
        'a' => return count(level-1, 'a'),
        'e' => return (count(level-1, 'a') + count(level-1, 'e')),
        'i' => return (count(level-1, 'a') + count(level-1, 'e') + count(level-1, 'i')),
        'o' => return (count(level-1, 'a') + count(level-1, 'e') + count(level-1, 'i') + count(level-1, 'o')),
        _ => return (count(level-1, 'a') + count(level-1, 'e') + count(level-1, 'i') + count(level-1, 'o') + count(level-1 ,'u')),
    }
}

// 数学
// https://leetcode.cn/problems/count-sorted-vowel-strings/solution/zhong-xue-shu-xue-ke-pu-n-ge-xiao-qiu-fang-dao-m-g/
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
    }
}

// DP
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = vec![1; 5];
        for _ in 0..n-1 {
            for j in 1..5 {
                dp[j] += dp[j-1];
            }
        }
        dp.iter().sum::<i32>()
    }
}