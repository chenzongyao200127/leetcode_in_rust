use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut degree = vec![0; n as usize + 1];
        let mut graph: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize + 1];
        let mut edge_set: HashSet<(i32, i32)> = HashSet::new();

        // 建立图的邻接表表示和边的集合
        for edge in &edges {
            let (mut x, mut y) = (edge[0], edge[1]);
            if x > y {
                std::mem::swap(&mut x, &mut y);
            }
            degree[x as usize] += 1;
            degree[y as usize] += 1;
            graph[x as usize].insert(y);
            graph[y as usize].insert(x);
            edge_set.insert((x, y));
        }

        let mut ans = i32::MAX;

        // 遍历每条边
        for &(x, y) in &edge_set {
            // 寻找与x和y都邻接的点z
            for &z in graph[x as usize].intersection(&graph[y as usize]) {
                ans = ans.min(degree[x as usize] + degree[y as usize] + degree[z as usize] - 6);
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
