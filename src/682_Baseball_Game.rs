// 682. Baseball Game
// https://leetcode.cn/problems/baseball-game/
// 0ms 100%
// 2.1 MB 33.3%
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut scores = Vec::new();
        operations.iter().for_each(|op| {
            match op.as_str() {
                "+" => { scores.push(scores[scores.len()-1] + scores[scores.len() - 2]); },
                "C" => { scores.pop(); },
                "D" => { scores.push(scores[scores.len()-1] * 2); },
                _ => { scores.push(op.parse().unwrap()); },
            }
        });

        scores.iter().sum()
    }
}
