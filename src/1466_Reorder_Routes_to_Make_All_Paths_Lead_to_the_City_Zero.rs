// 1466_Reorder_Routes_to_Make_All_Paths_Lead_to_the_City_Zero
// https://leetcode.cn/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/description/

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut map = vec![vec![]; n];
        for line in &connections {
            let (from, to) = (line[0] as usize, line[1] as usize);
            map[from].push((to, 1));
            map[to].push((from, 0));
        }
        let mut ans = 0;
        fn dfs(from: usize, to: usize, map: &Vec<Vec<(usize, i32)>>, ans: &mut i32) {
            for kvp in &map[from] {
                if kvp.0 != to {
                    *ans += kvp.1;
                    dfs(kvp.0, from, map, ans)
                }
            }
        }
        dfs(0, n, &map, &mut ans);
        ans
    }
}


use std::collections::HashSet;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for c in &connections {
            let (f, t) = (c[0] as usize, c[1] as usize);
            g[f].push(t);
            g[t].push(f);
        }

        let paths: HashSet<(i32, i32)> = connections.iter()
            .flat_map(|x| vec![(x[0], x[1])])
            .collect();

        let mut ans = 0;
        let mut visited: HashSet<usize> = HashSet::new();

        fn dfs(node: usize, g: &[Vec<usize>], paths: &HashSet<(i32, i32)>, ans: &mut i32, visited: &mut HashSet<usize>) {
            visited.insert(node);

            for &n in &g[node] {
                if !visited.contains(&n) {
                    if paths.contains(&(node as i32, n as i32)) {
                        *ans += 1;
                    }

                    dfs(n, g, paths, ans, visited);
                }
            }
        }

        dfs(0, &g, &paths, &mut ans, &mut visited);
        ans
    }
}



impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut st = vec![false; n];

        for e in connections.iter() {
            graph[e[0] as usize].push((e[1], true));
            graph[e[1] as usize].push((e[0], false));
        }

        let mut cnt = 0;

        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        st[0] = true;

        while ! q.is_empty() {
            let u = q.pop_front().unwrap();
            for &(v, flag) in graph[u as usize].iter() {
                if st[v as usize] {
                    continue;
                }
                st[v as usize] = true;
                if flag {
                    cnt += 1;
                }
                q.push_back(v);
            }
        }

        cnt
    }
}