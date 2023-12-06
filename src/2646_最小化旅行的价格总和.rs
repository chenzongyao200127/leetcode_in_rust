// 2646_最小化旅行的价格总和
// https://leetcode.cn/problems/minimize-the-total-price-of-the-trips/description/

use std::collections::HashSet;

impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, prices: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let mut total_visits = vec![0; n as usize];
        let mut graph = vec![vec![]; n as usize];

        // Construct the graph
        for edge in &edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            graph[from].push(to);
            graph[to].push(from);
        }

        let mut visited = HashSet::new();

        // Recursive function to find paths and update total visits
        fn find_paths(start: usize, end: usize, graph: &Vec<Vec<usize>>, local_visits: &mut Vec<i32>, visited: &mut HashSet<usize>, total_visits: &mut Vec<i32>) {
            visited.insert(start);
            local_visits[start] += 1;

            if start == end {
                for (i, &visit_count) in local_visits.iter().enumerate() {
                    total_visits[i] += visit_count;
                }
                return;
            }

            for &neighbor in &graph[start] {
                if !visited.contains(&neighbor) {
                    find_paths(neighbor, end, graph, local_visits, visited, total_visits);
                }
            }

            local_visits[start] -= 1;
            visited.remove(&start);
        }

        // Process each trip
        for trip in &trips {
            let start_point = trip[0] as usize;
            let end_point = trip[1] as usize;

            visited.clear();
            let mut local_visits = vec![0; n as usize];
            find_paths(start_point, end_point, &graph, &mut local_visits, &mut visited, &mut total_visits);
        }

        // Dynamic programming function to calculate minimum total price
        fn calculate_min_price(node: usize, parent: usize, prices: &Vec<i32>, total_visits: &Vec<i32>, graph: &Vec<Vec<usize>>) -> (i32, i32) {
            let mut cost_with_full_price = prices[node] * total_visits[node];
            let mut cost_with_half_price = prices[node] / 2 * total_visits[node];

            for &child in &graph[node] {
                if child != parent {
                    let (full_cost, half_cost) = calculate_min_price(child, node, prices, total_visits, graph);
                    cost_with_full_price += full_cost.min(half_cost);
                    cost_with_half_price += full_cost;
                }
            }

            (cost_with_full_price, cost_with_half_price)
        }
        
        // Calculate and return the minimum total price
        let (full_cost, half_cost) = calculate_min_price(0, usize::MAX, &prices, &total_visits, &graph);
        full_cost.min(half_cost)
    }
}
