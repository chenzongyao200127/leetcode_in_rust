pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let n = matrix[0].len();
    let mut ans = 0;

    // Iterate through all combinations using bitmasks
    for mask in 0u32..(1 << n) {
        if mask.count_ones() == num_select as u32 {
            let mut cnt = 0;
            'outer: for row in &matrix {
                for (j, &item) in row.iter().enumerate() {
                    // If the column is not selected but the cell is 1, skip this row
                    if ((mask >> j) & 1) == 0 && item == 1 {
                        continue 'outer;
                        // break;
                    }
                }
                // Row is valid, increment count
                cnt += 1;
            }
            ans = ans.max(cnt);
        }
    }

    ans
}

pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let n = heights.len();
    let mut answer = vec![0; n]; // Initialize answer vector with zeros
    let mut stack: Vec<i32> = Vec::new();

    for (idx, &h) in heights.iter().enumerate().rev() {
        while !stack.is_empty() && *stack.last().unwrap() < h {
            stack.pop();
            answer[idx] += 1;
        }

        if !stack.is_empty() {
            answer[idx] += 1;
        }

        stack.push(h)
    }

    answer
}

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut ans = Vec::new();
    let mut deque = VecDeque::new();

    for i in 0..n {
        // 移除超出窗口范围的元素
        while !deque.is_empty() && deque.front().unwrap() < &(i as i32 - k as i32 + 1) {
            deque.pop_front();
        }

        // 移除比当前元素小的元素，因为它们不可能成为最大值
        while !deque.is_empty() && nums[*deque.back().unwrap() as usize] < nums[i] {
            deque.pop_back();
        }

        deque.push_back(i as i32);

        if i >= k - 1 {
            ans.push(nums[*deque.front().unwrap() as usize]);
        }
    }

    ans
}

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    const ASCII_LOWER_A: u8 = b'a';
    const ASCII_LOWER_Z: u8 = b'z';
    let mut frequency = [0; 256];

    let input = s.as_bytes();

    // Count the frequency of each character in the input string
    for &byte in input {
        frequency[byte as usize] += 1;
    }

    let mut result = String::new();
    let mut last_char = 0;
    let mut repeat_count = 0;

    for _ in 0..input.len() {
        for char_code in (ASCII_LOWER_A..=ASCII_LOWER_Z).rev() {
            if frequency[char_code as usize] == 0 {
                continue;
            }

            if char_code == last_char {
                if repeat_count >= repeat_limit {
                    continue;
                }
                repeat_count += 1;
            } else {
                repeat_count = 1;
                last_char = char_code;
            }

            frequency[char_code as usize] -= 1;
            result.push(char_code as char);
            break;
        }
    }

    result
}

struct Graph {
    g: Vec<HashMap<i32, i32>>,
    dp: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = vec![HashMap::new(); n as usize];
        let mut dp = vec![vec![i32::MAX; n as usize]; n as usize];

        for i in 0..n as usize {
            dp[i][i] = 0;
        }

        for edge in &edges {
            let (from, to, value) = (edge[0], edge[1], edge[2]);
            if from < n && to < n {
                g[from as usize].insert(to, value);
                dp[from as usize][to as usize] = value;
            }
        }

        // Initialize the dp matrix with direct edge weights
        for i in 0..n as usize {
            for (&j, &cost) in &g[i] {
                dp[i][j as usize] = cost;
            }
        }

        // Apply Floyd-Warshall to populate initial dp matrix
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    let via_k = dp[i][k].saturating_add(dp[k][j]);
                    dp[i][j] = dp[i][j].min(via_k);
                }
            }
        }

        Graph { g, dp }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let (from, to, value) = (edge[0], edge[1], edge[2]);
        if (from as usize) < self.g.len() && (to as usize) < self.g.len() {
            self.g[from as usize].insert(to, value);
            for i in 0..self.g.len() {
                for j in 0..self.g.len() {
                    let new_via_from_to = self.dp[i][from as usize]
                        .saturating_add(value)
                        .saturating_add(self.dp[to as usize][j]);
                    self.dp[i][j] = self.dp[i][j].min(new_via_from_to);
                }
            }
        }
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let res = self.dp[node1 as usize][node2 as usize];
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

// https://leetcode.cn/problems/two-sum/?envType=study-plan-v2&envId=top-100-liked
// use std::collections::HashMap;
// use std::collections::HashSet;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let c = nums[i];
        let t = target - c;
        if set.contains(&t) {
            return vec![i as i32, *map.get(&t).unwrap()];
        } else {
            set.insert(c);
            map.insert(c, i as i32);
        }
    }

    unreachable!()
}

pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
    let MOD = (1000_000_000 + 7) as i32;
    ranges.sort_by(|a, b| a[0].cmp(&(b[0])));
    println!("{:?}", ranges);

    let mut splits = vec![];
    let mut l = ranges[0][0];
    let mut r = ranges[0][1];

    for i in 1..ranges.len() {
        let x = ranges[i][0];
        let y = ranges[i][1];

        // 有交集 expand
        if x <= r {
            r = r.max(y);
        } else {
            // 无交集
            splits.push(vec![l, r]);
            l = x;
            r = y;
        }
    }
    splits.push(vec![l, r]);

    let n = splits.len();
    println!("{:?}", splits);
    let two: i32 = 2;
    two.pow(n as u32) % MOD
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key = chars.into_iter().collect::<String>();
        map.entry(key).or_default().push(s);
    }
    map.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    println!("Hello World");
    let ans = count_ways(vec![vec![0, 0], vec![8, 9], vec![12, 13], vec![1, 3]]);
    assert_eq!(ans, 16)
}
