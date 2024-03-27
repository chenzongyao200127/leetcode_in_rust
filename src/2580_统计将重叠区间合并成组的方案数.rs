// 2580_统计将重叠区间合并成组的方案数
// https://leetcode.cn/problems/count-ways-to-group-overlapping-ranges/description/

impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_unstable();
        let n = ranges.len();
        let mut i = 0;
        let mut ans = 1;
        while i < n {
            let mut r = ranges[i][1];
            let mut j = i + 1;
            while j < n && ranges[j][0] <= r {
                if r < ranges[j][1] {
                    r = ranges[j][1];
                }
                j += 1;
            }
            ans = (ans as i64 * 2 % 1_000_000_007) as i32;
            i = j;
        }
        ans
    }
}
