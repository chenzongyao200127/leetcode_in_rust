// 1039_Minimum_Score_Triangulation_of_Polygon
// https://leetcode.cn/problems/minimum-score-triangulation-of-polygon/

use std::collections::HashMap;
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        fn dfs(i: usize, j: usize, values: &[i32], memo: &mut HashMap<(usize, usize), i32>) -> i32 {
            if i + 1 == j {
                return 0;
            }
            if let Some(&result) = memo.get(&(i, j)) {
                return result;
            }
            let result = (i + 1..j)
                .map(|k| dfs(i, k, values, memo) + dfs(k, j, values, memo) + values[i] * values[k] * values[j])
                .min()
                .unwrap();
            memo.insert((i, j), result);
            result
        }

        let mut memo = HashMap::new();
        dfs(0, values.len() - 1, &values, &mut memo)
    }
}


impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        fn dfs(i: usize, j: usize, v: &[i32], tab: &mut Vec<Vec<i32>>) -> i32 {
            if tab[i][j] != i32::MAX {
                return tab[i][j];
            }
            for k in i + 1..j {
                tab[i][j] = tab[i][j].min(dfs(i, k, v, tab) + dfs(k, j, v, tab) + v[i] * v[k] * v[j]);
            }
            tab[i][j]
        }

        let n = values.len();
        let mut tab = vec![vec![i32::MAX; n]; n];

        for i in 0..n - 1 {
            tab[i][i + 1] = 0; // 两个点无法组成三角形
        }

        dfs(0, n - 1, &values, &mut tab)
    }
}
