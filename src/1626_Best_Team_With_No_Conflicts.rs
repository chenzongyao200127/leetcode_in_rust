// 1626_Best_Team_With_No_Conflicts
// https://leetcode.cn/problems/best-team-with-no-conflicts/

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players = vec![];
        for i in 0..scores.len() {
            players.push((scores[i], ages[i]));
        }
        players.sort_unstable_by(|a, b| ((a.0).cmp(&(b.0))).then((a.1).cmp(&(b.1))));
        let mut dp = vec![0; scores.len()];
        dp[0] = players[0].0;
        for i in 1..scores.len() {
            let mut tmp = players[i].0;
            for j in 0..i {
                if players[j].1 <= players[i].1 {
                    tmp = tmp.max(dp[j] + players[i].0);
                }
            }
            dp[i] = tmp;
        }

        dp.into_iter().max().unwrap()
    }
}