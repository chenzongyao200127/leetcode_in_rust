// 1851_Minimum_Interval_to_Include_Each_Query
// https://leetcode.cn/problems/minimum-interval-to-include-each-query/

// 给你一个二维整数数组 intervals ，其中 intervals[i] = [lefti, righti] 
// 表示第 i 个区间开始于 lefti 、结束于 righti（包含两侧取值，闭区间）。区间的 长度 定义为区间中包含的整数数目，
// 更正式地表达是 righti - lefti + 1 。

// 再给你一个整数数组 queries 。第 j 个查询的答案是满足 lefti <= queries[j] <= righti 的 
// 长度最小区间 i 的长度 。如果不存在这样的区间，那么答案是 -1 。

// 以数组形式返回对应查询的所有答案。


use std::collections::BinaryHeap;
use std::cmp::Reverse;
// 写的非常优美
impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (v[0], -v[1]));
    
        let mut queries: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();
        queries.sort_by_key(|&(_, x)| x);
    
        let mut heap = BinaryHeap::new();
        let mut res = vec![-1; queries.len()];
        let mut j = 0;
        for (i, qi) in queries {
            while j < intervals.len() && intervals[j][0] <= qi {
                let len = intervals[j][1] - intervals[j][0] + 1;
                heap.push((Reverse(len), Reverse(intervals[j][1])));
                j += 1;
            }
            while let Some((_, Reverse(right))) = heap.peek() {
                if *right < qi {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some((Reverse(len), _)) = heap.peek() {
                res[i] = *len;
            }
        }
        res
    }
}
