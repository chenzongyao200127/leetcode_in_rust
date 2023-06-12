// 1617_Count_Subtrees_With_Max_Distance_Between_Cities
// https://leetcode.cn/problems/count-subtrees-with-max-distance-between-cities/

// 3/12/2023 每日一题
/// 树上任意两节点之间最长的简单路径即为树的「直径」，可以参考「树的直径」、「二叉树的直径」等相关解法。
/// 一颗树可以有多条直径，但直径的长度都是一样的，计算树的直径长度有常见的两种方法:
/// - DP (最远距离 + 次远距离 )
/// - 深度优先搜索 (两次DFS x -> y -> z) 节点 y 与 节点 z 之间的距离即为直径的长度。

use std::collections::VecDeque;
impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        
        // Build the tree
        let mut g = vec![VecDeque::new(); n];
        for edge in &edges {
            let x = (edge[0] - 1) as usize;
            let y = (edge[1] - 1) as usize;
            g[x].push_back(y);
            g[y].push_back(x);
        }

        let mut ans = vec![0; n - 1];
        
        // Binary enumeration
        for mask in 3..(1 << n) {   // 需要至少两个点
            if mask & (mask - 1) == 0 {
                continue;
            }
            
            // 求树的直径
            let mut vis = 0;
            let mut diameter = 0;

            fn dfs(x: usize, g: &Vec<VecDeque<usize>>, mask: usize, vis: &mut usize, diameter: &mut usize) -> i32 {
                *vis |= 1 << x; // 标记 x 访问过

                let mut max_len = 0;
                for &y in &g[x] {
                    if (*vis >> y & 1) == 0 && (mask >> y & 1) == 1 { // y 没有访问过且在 mask 中
                        let ml = dfs(y, g, mask, vis, diameter) + 1;
                        *diameter = (*diameter).max(max_len as usize + ml as usize);
                        max_len = max_len.max(ml);
                    }
                }
                max_len
            }

            // 从一个在 mask 中的点开始递归
            dfs((mask as usize).trailing_zeros() as usize, &g, mask, &mut vis, &mut diameter); 

            if vis == mask {
                ans[diameter - 1] += 1;
            }
        }

        ans
    }
}


impl Solution {
    /// 暴力枚举 i 和 j 作为直径的两个端点 ，那么从 i 到 j 的这条简单路径是直径，这上面的每个点都必须选。
    /// 这条路上还有哪些点是可以选的，哪些点是不能选的？
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize - 1].push(edge[1] as usize - 1);
            g[edge[1] as usize - 1].push(edge[0] as usize - 1);
        }
        let mut result = vec![0; n - 1];
        
        let mut dis = vec![vec![0; n]; n];
        fn dfs(g: &Vec<Vec<usize>>, start: usize, u: usize, fa: usize, dis: &mut Vec<Vec<i32>>) {
            for &v in &g[u] {
                if v != fa {
                    dis[start][v] = dis[start][u] + 1;
                    dfs(g, start, v, u, dis);
                }
            }
        }
        for i in 0..n {
            dfs(&g, i, i, n, &mut dis);
        }
        fn dfs2(g: &Vec<Vec<usize>>, u: usize, fa: usize, dis: &Vec<Vec<i32>>, i: usize, j: usize, d: i32) -> i32 {
            let mut cnt = 1;  // 选u
            for &v in &g[u] {
                if v != fa {
                    if (dis[i][v] < d || dis[i][v] == d && v > j) && (dis[j][v] < d || dis[j][v] == d && v > i) {
                        cnt *= dfs2(g, v, u, dis, i, j, d);  // 这个子树的可选数，乘法原理
                    }
                }
            }
            if dis[i][u] + dis[j][u] > d {
                cnt += 1; // 不选u
            }
            cnt
        }
        for i in 0..n {
            for j in i + 1..n {
                let d = dis[i][j];
                result[d as usize - 1] += dfs2(&g, i, n, &dis, i, j, d);
            }
        }
        result
    }
}