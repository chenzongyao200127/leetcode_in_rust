// 2611_Mice_and_Cheese
// https://leetcode.cn/problems/mice-and-cheese/

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut rewards: Vec<_> = reward1.into_iter().zip(reward2).collect();
        rewards.sort_unstable_by(|&a, &b| (b.0 - b.1).cmp(&(a.0 - a.1)));
        let mut ans = 0;
        for i in 0..rewards.len() {
            if i < k as usize {
                ans += rewards[i].0;
            } else {
                ans += rewards[i].1;
            }
        }
        ans
    }
}

// 迭代器写法
impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut rewards: Vec<_> = reward1.into_iter().zip(reward2).collect();
        let k = k as usize;
        
        rewards.sort_unstable_by(|a, b| (b.0 - b.1).cmp(&(a.0 - a.1)));
        
        rewards.into_iter()
            .enumerate()
            .map(|(i, (r1, r2))| if i < k { r1 } else { r2 })
            .sum()
    }
}


impl Solution {
    // 2023-04-02, 闰二月十二, 癸卯年, 兔, 乙卯月, 庚寅日
    // https://github.com/shionryuu/leetcode-rust
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let ans = reward2.iter().sum::<i32>();

        let mut diffs = reward1
            .iter()
            .zip(reward2.iter())
            .map(|t| t.0 - t.1)
            .collect::<Vec<_>>();
        diffs.sort_unstable_by_key(|v| -v);

        ans + diffs[0..k as usize].iter().sum::<i32>()
    }
}