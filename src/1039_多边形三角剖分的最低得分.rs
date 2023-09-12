// 1039_多边形三角剖分的最低得分
// https://leetcode.cn/problems/minimum-score-triangulation-of-polygon/description/

use std::collections::HashMap;
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut map: HashMap<(usize, usize), i32> = HashMap::new();

        #[inline]
        fn dfs(values: &Vec<i32>, l: usize, r: usize, map: &mut HashMap<(usize, usize), i32>) -> i32 {
            if r - l + 1 < 3 {
                return 0
            }

            if let Some(&val) = map.get(&(l, r)) {
                return val
            }
            
            let mut min_val = i32::MAX;
            for k in l+1..r {
                let cur_val = values[l] * values[k] * values[r];
                min_val = min_val.min(dfs(values, l, k, map) + cur_val + dfs(values, k, r, map))
            }
            
            map.insert((l, r), min_val);
            return min_val
        }

        dfs(&values, 0, values.len()-1, &mut map)
    }
}