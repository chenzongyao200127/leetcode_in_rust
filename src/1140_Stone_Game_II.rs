// 1140. Stone Game II
// https://leetcode.cn/problems/stone-game-ii/

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let len = piles.len();
        let mut sum = 0;
        let mut dp = vec![vec![0;len+1];len];
        for i in (0..len).rev() {
            sum += piles[i];
            for m in 1..=len {
                if i + 2*m >= len {
                    dp[i][m] = sum;
                } else {
                    for x in 1..(2*m+1) {
                        dp[i][m] = dp[i][m].max(sum - dp[i+x][m.max(x)]);
                    }
                }
            }
        }
    
        dp[0][1]
    }
}