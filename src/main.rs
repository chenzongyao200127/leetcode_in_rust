pub fn main() {
    let ans = best_team_score(vec![4,6,5,7,8,2], vec![3,2,4,6,5,2]);
    assert_eq!(ans, 19);

    let ans = best_team_score(vec![4,5,6,5], vec![2,1,2,1]);
    assert_eq!(ans, 16);
}

pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut players = vec![];
    for i in 0..scores.len() {
        players.push((scores[i], ages[i]));
    }
    players.sort_unstable_by(|a, b| ((a.0).cmp(&(b.0))).then((a.1).cmp(&(b.1))));
    // println!("{:?}", players);
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
    // println!("{:?}", dp);

    dp.into_iter().max().unwrap()
}