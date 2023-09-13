//  473_火柴拼正方形
//  https://leetcode.cn/problems/matchsticks-to-square/description/

// 你将得到一个整数数组 matchsticks ，其中 matchsticks[i] 是第 i 个火柴棒的长度
// 你要用 所有的火柴棍 拼成一个正方形。你 不能折断 任何一根火柴棒，但你可以把它们连在一起，而且每根火柴棒必须 使用一次 。

// 如果你能使这个正方形，则返回 true ，否则返回 false 。

// 输入: matchsticks = [1,1,2,2,2]
// 输出: true
// 解释: 能拼成一个边长为2的正方形，每边两根火柴。
// 示例 2:

// 输入: matchsticks = [3,3,3,3,4]
// 输出: false
// 解释: 不能用所有火柴拼成一个正方形。
 

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let n = matchsticks.len();

        matchsticks.sort_unstable();

        let total = matchsticks.iter().sum::<i32>();
        
        if total % 4 != 0 {
            return false;
        }

        let target = total / 4;

        let mut dp = vec![-1; 1 << n];

        dp[0] = 0;

        for mask in 0..1 << n {
            if dp[mask as usize] == -1 {
                continue;
            }

            for j in 0..n {
                let new_mask = mask | (1 << j);
                if mask & (1 << j) == 0 && dp[mask] + matchsticks[j] <= target {
                    dp[new_mask] = (dp[mask] + matchsticks[j]) % target;
                }
            }
        }

        dp[(1 << n) - 1] == 0
    }
}