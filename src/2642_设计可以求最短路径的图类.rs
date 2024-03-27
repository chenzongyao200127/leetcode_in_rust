use std::cmp::Reverse;
use std::collections::BinaryHeap;

// A Graph structure which holds the adjacency list representation of the graph.
struct Graph {
    graph: Vec<Vec<(i32, i32)>>, // Each node (i32) has a list of edges represented as (destination, cost).
}

impl Graph {
    // Constructs a new Graph with 'n' nodes and edges provided as a vector of vectors.
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut graph = vec![Vec::new(); n as usize]; // Initialize the adjacency list.
        for edge in edges {
            let x = edge[0]; // Source node.
            let y = edge[1]; // Destination node.
            let cost = edge[2]; // Cost of the edge.
            graph[x as usize].push((y, cost)); // Add the edge to the graph.
        }
        Graph { graph }
    }

    // Adds a new edge to the graph.
    fn add_edge(&mut self, edge: Vec<i32>) {
        let x = edge[0]; // Source node.
        let y = edge[1]; // Destination node.
        let cost = edge[2]; // Cost of the edge.
        self.graph[x as usize].push((y, cost)); // Add the edge to the adjacency list.
    }

    // Finds the shortest path from node1 to node2 using Dijkstra's algorithm.
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut pq = BinaryHeap::new(); // Priority queue (min-heap) to select the node with the smallest distance.
        let mut dist = vec![std::i32::MAX; self.graph.len()]; // Distance vector initialized with MAX values.
        dist[node1 as usize] = 0; // Distance to the starting node is 0.
        pq.push((Reverse(0), node1)); // We use Reverse to turn the max-heap into a min-heap.

        // While there are nodes to process in the priority queue.
        while let Some((Reverse(cost), cur)) = pq.pop() {
            // If the current node is the target, return the cost.
            if cur == node2 {
                return cost;
            }
            // Explore the neighbors of the current node.
            for &(next, ncost) in &self.graph[cur as usize] {
                // If a shorter path to 'next' is found, update the distance and add to the priority queue.
                if dist[next as usize] > cost + ncost {
                    dist[next as usize] = cost + ncost;
                    pq.push((Reverse(cost + ncost), next));
                }
            }
        }
        -1 // If the node is not reachable, return -1.
    }
}
