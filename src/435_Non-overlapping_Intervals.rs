// 435. Non-overlapping Intervals
// https://leetcode.cn/problems/non-overlapping-intervals/

// 贪心
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        let mut intervals = intervals;
        let mut len = 1;
        intervals.sort_unstable_by(|x, y| x[1].cmp(&y[1]).then(x[0].cmp(&y[0])));
        let mut right = intervals[0][1];
        for pair in intervals.iter() {
            if pair[0] >= right {
                right = pair[1];
                len += 1;
            }
        }
        intervals.len() as i32 - len
    }
}

// 动态规划
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        let mut intervals = intervals;
        intervals.sort_unstable_by(|x, y| x[0].cmp(&y[0]).then(x[1].cmp(&y[1])));
        let len = intervals.len();
        let mut dp = vec![1; len];
        for i in 1..len {
            for j in 0..i {
                if intervals[j][1] <= intervals[i][0] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        
        dp.sort_unstable();
        len  as i32 - dp[dp.len()-1]
    }
}




