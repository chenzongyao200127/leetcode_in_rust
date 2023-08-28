// 691_Stickers_to_Spell_Word
// https://leetcode.cn/problems/stickers-to-spell-word/description/

// 以下是一个可能的解决方案：

// 对于每个贴纸，我们可以计算出它包含的每个字母的频率。我们还需要计算目标字符串target中每个字母的频率。

// 定义一个递归函数search，它的参数是一个数组，表示目标字符串中还需要的每个字母的数量。
// 这个函数返回拼出剩余的目标字符串所需的最小贴纸数量。

// 在search函数中，我们可以尝试使用每个贴纸，
// 然后递归地计算使用这个贴纸后还需要的最小贴纸数量。我们选择最小的一个作为结果。

// 为了加速计算，我们可以使用记忆化的手段。如果我们已经计算过一个特定的目标字符串的状态
// （即每个字母所需的数量），那么就不需要再次计算它。


use std::collections::HashMap;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        // stickers_freq 存储每个贴纸的每个字母的频率
        let mut stickers_freq = vec![vec![0; 26]; stickers.len()];

        // target_freq 存储目标字符串中每个字母的频率
        let mut target_freq = vec![0; 26];

        // 计算贴纸中每个字母的频率
        for (i, sticker) in stickers.iter().enumerate() {
            for ch in sticker.chars() {
                stickers_freq[i][(ch as u8 - b'a') as usize] += 1;
            }
        }
        
        // 计算目标字符串中每个字母的频率
        for ch in target.chars() {
            target_freq[(ch as u8 - b'a') as usize] += 1;
        }

        // 使用哈希映射进行记忆化
        let mut memo = HashMap::new();
        let res = Self::search(&stickers_freq, target_freq, &mut memo);

        // 如果结果为 i32::MAX，返回-1表示无法拼凑目标字符串，否则返回结果
        if res == i32::MAX {
            return -1;
        }
        res
    }

    fn search(stickers: &Vec<Vec<i32>>, mut target: Vec<i32>, memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
        // 如果此状态已经计算过，则直接返回
        if let Some(&cached) = memo.get(&target) {
            return cached;
        }

        let mut ans = i32::MAX;
        
        // 如果所有的目标字母都已经满足，则返回 0
        if target.iter().all(|&num| num == 0) {
            ans = 0;
        } else {
            // 遍历每个贴纸并尝试使用它
            for sticker in stickers {
                // 如果贴纸中不包含我们需要的字母，则跳过此贴纸
                if sticker[target.iter().position(|&ch| ch > 0).unwrap()] == 0 {
                    continue;
                }
                
                let mut new_target = target.clone();
                // 使用贴纸后，更新目标字符串的状态
                for (j, &num) in sticker.iter().enumerate() {
                    new_target[j] = (new_target[j] - num).max(0);
                }
                
                // 递归地搜索使用此贴纸后的最小数量
                let temp_ans = Self::search(stickers, new_target, memo);
                if temp_ans != i32::MAX {
                    ans = ans.min(1 + temp_ans);
                }
            }
        }
        
        // 将计算的结果存储到哈希映射中
        memo.insert(target, ans);
        ans
    }
}

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let state = 1 << n;
        let mut dp = vec![i32::MAX; state as usize];
        let target_ch: Vec<char> = target.chars().collect();
    
        dp[0] = 0;
        for s in 0..state as usize {
            if dp[s] == i32::MAX {
                continue;
            }
    
            for strick in stickers.iter() {
                let mut nxt = s;
                for ch in strick.chars() {
                    for k in 0..n {
                        if ch == target_ch[k] && nxt & (1 << k) == 0 {
                            nxt |= 1 << k;
                            break;
                        }
                    }
                }
    
                dp[nxt] = dp[nxt].min(dp[s] + 1);
            }
        }
    
    
        if dp[dp.len() - 1] == i32::MAX {
            -1
        } else {
            dp[dp.len() - 1]
        }
    }
}


// GPT-4
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let m = 1 << n;
        let mut dp: Vec<i32> = vec![i32::MAX; m];
        dp[0] = 0;

        for i in 0..m {
            if dp[i] == i32::MAX {
                continue;
            }
            
            for sticker in &stickers {
                let mut nxt = i;
                for ch in sticker.chars() {
                    for (k, t_ch) in target.chars().enumerate() {
                        if t_ch == ch && nxt & (1 << k) == 0 {
                            nxt |= 1 << k;
                            break;
                        }
                    }
                }
                dp[nxt] = dp[nxt].min(dp[i] + 1);
            }
        }

        if dp[m - 1] != i32::MAX {
            dp[m - 1]
        } else {
            -1
        }
    }
}