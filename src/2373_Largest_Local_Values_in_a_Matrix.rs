// 2373. Largest Local Values in a Matrix
// https://leetcode.cn/problems/largest-local-values-in-a-matrix/

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = grid.len();
        let mut ans = vec![vec![0; len-2]; len-2];
        for i in 2..len {
            for j in 2..len {
                ans[i-2][j-2] = largest_nine(&grid, i, j);
            }
        }
        ans
    }
}

pub fn largest_nine(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut max = 0;
    for i in x-2..=x {
        for j in y-2..=y {
            max = max.max(grid[i][j]);
        }
    }

    max
}

    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let mut s: HashSet<i32> = HashSet::new();
        for (i, val) in nums.iter().enumerate() {
            for diff_y in 0..=value_diff {
                if s.contains(&(val+diff_y)) || s.contains(&(val-diff_y)) { // to be optimised
                    return true;
                }
            }
    
            s.insert(*val);
            if s.len() >= (index_diff + 1) as usize {
                let offset = i - index_diff as usize;
                s.remove(&nums[offset]);
            }
        }
        false
    }