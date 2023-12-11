// 1631_Path_With_Minimum_Effort
// https://leetcode.cn/problems/path-with-minimum-effort/description/

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();
        let m = heights.len();
        let n = heights[0].len();
        let mut max_cost = 0;
    
        for i in 0..m {
            for j in 0..n {
                for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let cost = (heights[i][j] - heights[ni as usize][nj as usize]).abs() as usize;
                        max_cost = max_cost.max(cost);
                        graph.entry((i, j)).or_insert_with(Vec::new).push(((ni as usize, nj as usize), cost));
                    }
                }
            }
        }
    
        fn is_available(
            k: usize,
            m: usize,
            n: usize,
            node: (usize, usize),
            graph: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
            visited: &mut HashSet<(usize, usize)>,
            memo: &mut HashMap<((usize, usize), usize), bool>,
        ) -> bool {
            if node == (m - 1, n - 1) {
                return true;
            }
            if let Some(&cached) = memo.get(&(node, k)) {
                return cached;
            }
    
            visited.insert(node);
            if let Some(neighbors) = graph.get(&node) {
                for &(next_node, cost) in neighbors {
                    if cost <= k && !visited.contains(&next_node) {
                        if is_available(k, m, n, next_node, graph, visited, memo) {
                            memo.insert((node, k), true);
                            return true;
                        }
                    }
                }
            }
    
            visited.remove(&node);
            memo.insert((node, k), false);
            false
        }
    
        let mut l = 0;
        let mut r = max_cost;
        let mut visited = HashSet::new();
        let mut memo = HashMap::new();
    
        while l < r {
            let mid = l + (r - l) / 2;
            visited.clear();
    
            if is_available(mid, m, n, (0, 0), &graph, &mut visited, &mut memo) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
    
        l as i32
    }
}