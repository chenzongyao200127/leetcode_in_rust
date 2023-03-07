// 57. Insert Interval
// https://leetcode.cn/problems/insert-interval/

// 57. Insert Interval
// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
// Return intervals after the insertion.

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        // println!("{:?}", intervals);
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 1..intervals.len() {
            if intervals[i][0] <= right {
                right = right.max(intervals[i][1]);
            } else {
                ans.push(vec![left, right]);
                left = intervals[i][0];
                right = intervals[i][1];
            }
        }
        ans.push(vec![left, right]);

        ans
    }
}


use std::cmp::{min, max};
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut merged = vec![];
        for interval in intervals {
            if interval[1] < new_interval[0] {
                merged.push(interval);
            } else if new_interval[1] >= interval[0] {
                new_interval[0] = min(new_interval[0],interval[0]);
                new_interval[1] = max(new_interval[1], interval[1]);
            } else {
                merged.push(new_interval);
                new_interval = interval;
            }
        }
        merged.push(new_interval);
        merged
    }
}