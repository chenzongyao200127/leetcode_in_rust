// 1637_Widest_Vertical_Area_Between_Two_Points_Containing_No_Points
// https://leetcode.cn/problems/widest-vertical-area-between-two-points-containing-no-points/

use std::collections::HashSet;
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for vec in points {
            set.insert(vec[0]);
        }
        let mut tmp = vec![];
        set.iter().for_each(|&x| {tmp.push(x)});
        tmp.sort_unstable();
        let mut ans = 0;
        for i in 1..tmp.len() {
            ans = ans.max(tmp[i] - tmp[i-1]);
        }
        ans
    }
}


impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut tmp = vec![];
        for vec in points {
            tmp.push(vec[0]);
        }
        tmp.sort_unstable();
        let mut ans = 0;
        for i in 1..tmp.len() {
            ans = ans.max(tmp[i] - tmp[i-1]);
        }
        ans
    }
}


impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|x, y| {
            x[0].cmp(&y[0])
        });
        let mut ans = 0;
        for v in points.windows(2).into_iter() {
            ans = std::cmp::max(ans,v[1][0]-v[0][0])
        }

        ans
    }
}
