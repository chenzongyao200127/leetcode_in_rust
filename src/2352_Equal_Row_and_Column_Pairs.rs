// 2352_Equal_Row_and_Column_Pairs
// https://leetcode.cn/problems/equal-row-and-column-pairs/
use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut cnt = HashMap::new();
    
        for r in &grid {
            *cnt.entry(r.clone()).or_insert(0) += 1;
        }
    
        let mut ans = 0;
        for c in 0..n {
            let col: Vec<_> = grid.iter().map(|r| r[c]).collect();
            ans += cnt.get(&col).unwrap_or(&0);
        }
    
        ans
    }
}


impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for row in grid.iter() {
            *map.entry(row).or_insert(0) += 1;
        }
        let (mut ans, mut temp) = (0, vec![0; grid.len()]);
        for col in 0..grid.len() {
            for row in 0..grid.len() {
                temp[row] = grid[row][col];
            }
            ans += map.get(&temp).unwrap_or(&0);
        }
        ans
    }
}