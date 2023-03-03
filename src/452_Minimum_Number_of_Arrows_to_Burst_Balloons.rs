// 452. Minimum Number of Arrows to Burst Balloons
// https://leetcode.cn/problems/minimum-number-of-arrows-to-burst-balloons/

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut points = points;
        points.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
        let mut end = i32::MAX;
        for point in points {
            if point[0] <= end {
                end = end.min(point[1]);
            } else {
                end = point[1];
                ans += 1;
            }
        }

        ans+1
    }
}

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
      if points.is_empty() {
        return 0;
      }
      points.sort_unstable_by_key(|x| x[0]);
      let mut ret = 1;
      let mut bm = i32::MAX;
      for i in points.iter() {
        if i[0] > bm {
          ret += 1;
          bm = i[1];
        } else {
          bm = std::cmp::min(bm, i[1]);
        }
      }
      ret
    }
  }
  