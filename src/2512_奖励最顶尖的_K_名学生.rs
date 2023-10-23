// 2512_奖励最顶尖的_K_名学生
// https://leetcode.cn/problems/reward-top-k-students/description/

use std::collections::HashSet;

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let positive_feedback_set = positive_feedback.iter().collect::<HashSet<_>>();
        let negative_feedback_set = negative_feedback.iter().collect::<HashSet<_>>();

        let mut score = vec![0; student_id.len()];

        let report = report
            .into_iter()
            .map(|x| {
                   x.to_string()
                    .split_whitespace() 
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<_>>();

        for i in 0..report.len() {
            for word in &report[i] {
                if positive_feedback_set.contains(word) {
                    score[i] += 3;
                } else if negative_feedback_set.contains(word) {
                    score[i] -= 1;
                }
            }
        }

        let mut ans: Vec<_> = score.iter().zip(&student_id).collect();

        ans.sort_by(|(s1, i1), (s2, i2)| {
            match s2.cmp(s1) {
                std::cmp::Ordering::Equal => i1.cmp(i2),
                order => order,
            }
        });

        ans.into_iter()
            .take(k as usize)
            .map(|(_, b)| *b)
            .collect()
    }
}