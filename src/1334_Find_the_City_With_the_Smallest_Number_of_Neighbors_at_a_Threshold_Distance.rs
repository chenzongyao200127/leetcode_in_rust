// 1334_Find_the_City_With_the_Smallest_Number_of_Neighbors_at_a_Threshold_Distance
// https://leetcode.cn/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/description/


// Wrong 
// 错误解法，会错过最短路径
use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut graph = vec![vec![0; n as usize]; n as usize];
        let mut suss = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            let f_node = edges[i][0];
            let t_node = edges[i][1];
            let weight = edges[i][2];

            graph[f_node as usize][t_node as usize] = weight;
            graph[t_node as usize][f_node as usize] = weight;
            suss[f_node as usize].push(t_node);
            suss[t_node as usize].push(f_node);
        }    

        // println!("{:?}", graph);

        let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
        
        #[inline]
        fn dfs(
            suss: &Vec<Vec<i32>>, 
            graph: &Vec<Vec<i32>>, 
            memo: &mut HashMap<(usize, usize, usize), usize>, 
            start_node: usize,
            from_ndoe: usize,
            distance_threshold: i32,
            neighbors: &mut Vec<HashSet<usize>>,
            visited: &mut HashSet<usize>,
        ) {
            // println!("suss[from_ndoe]: {:?}", suss[from_ndoe]);
            // println!("visited: {:?}", visited);
            // println!("distance_threshold: {:?}", distance_threshold);
            for &next_node in suss[from_ndoe].iter() {
                let cur_d = graph[from_ndoe][next_node as usize];
                if !visited.contains(&(next_node as usize)) && cur_d <= distance_threshold {
                    neighbors[start_node].insert(next_node as usize);
                    visited.insert(next_node as usize);
                    let new_distance = distance_threshold - cur_d;
                    if new_distance < 0 {
                        return;
                    } else {
                        dfs(suss, graph, memo, start_node, next_node as usize, new_distance, neighbors, visited)   
                    }
                }
            }
        }

        for node in 0..n as usize {
            let mut visited = HashSet::new();
            visited.insert(node);
            dfs(&suss, &graph, &mut memo, node, node, distance_threshold, &mut neighbors, &mut visited);
            // println!("==============");
        }

        // println!("{:?}", neighbors);

        let mut neighbors = neighbors.iter().map(|s| s.len()).enumerate().collect::<Vec<_>>();
        neighbors.sort_by(|&a, &b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        neighbors[0].0 as i32
    }
}


impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut graph = vec![vec![i32::MAX; n as usize]; n as usize];

        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u][v] = w;
            graph[v][u] = w;
        }

        // Initialize the graph with 0 distance to self
        for i in 0..n as usize {
            graph[i][i] = 0;
        }

        // Floyd-Warshall Algorithm to compute shortest paths
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    // Only perform the addition if neither distance is i32::MAX
                    if graph[i][k] != i32::MAX && graph[k][j] != i32::MAX {
                        graph[i][j] = std::cmp::min(graph[i][j], graph[i][k] + graph[k][j]);
                    }
                }
            }
        }

        // Count reachable cities for each city
        let reachable_counts = graph.iter()
            .map(|row| row.iter().filter(|&&d| d <= distance_threshold).count() - 1)
            .enumerate()
            .collect::<Vec<_>>();

        // Find the city with the minimum number of reachable cities
        reachable_counts.iter().min_by_key(|&&(i, count)| (count, -(i as i32))).unwrap().0 as i32
    }
}


impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut w = vec![vec![i32::MAX / 2; n]; n]; // 除 2 防止加法溢出
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let wt = e[2];
            w[x][y] = wt;
            w[y][x] = wt;
        }

        let mut f = w;
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    f[i][j] = f[i][j].min(f[i][k] + f[k][j]);
                }
            }
        }

        let mut ans = 0;
        let mut min_cnt = n;
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..n {
                if j != i && f[i][j] <= distance_threshold {
                    cnt += 1;
                }
            }
            if cnt <= min_cnt { // 相等时取最大的 i
                min_cnt = cnt;
                ans = i;
            }
        }
        ans as i32
    }
}