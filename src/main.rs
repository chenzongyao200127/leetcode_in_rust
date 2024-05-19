use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let k = k as usize;
    let c = candidates as usize;
    if c * 2 + k > n {
        costs.sort_unstable();
        return costs[..k].iter().map(|&x| x as i64).sum();
    }

    let mut pre = BinaryHeap::new();
    let mut suf = BinaryHeap::new();
    for i in 0..c {
        pre.push(-costs[i]); // 加负号，变成最小堆
        suf.push(-costs[n - 1 - i]);
    }

    let mut ans = 0;
    let mut i = c;
    let mut j = n - 1 - c;
    for _ in 0..k {
        if pre.peek().unwrap() >= suf.peek().unwrap() {
            ans -= pre.pop().unwrap() as i64;
            pre.push(-costs[i]);
            i += 1;
        } else {
            ans -= suf.pop().unwrap() as i64;
            suf.push(-costs[j]);
            j -= 1;
        }
    }
    ans
}

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let n = quality.len();
    let k = k as usize;
    let mut id = (0..n).collect::<Vec<_>>();
    // 按照 r 值排序
    id.sort_unstable_by(|&i, &j| (wage[i] * quality[j]).cmp(&(wage[j] * quality[i])));

    let mut h = BinaryHeap::new();
    let mut sum_q = 0;
    for i in 0..k {
        h.push(quality[id[i]]);
        sum_q += quality[id[i]];
    }

    // 选 r 值最小的 k 名工人
    let mut ans = sum_q as f64 * wage[id[k - 1]] as f64 / quality[id[k - 1]] as f64;

    // 后面的工人 r 值更大
    // 但是 sum_q 可以变小，从而可能得到更优的答案
    for i in k..n {
        let q = quality[id[i]];
        if q < *h.peek().unwrap() {
            sum_q -= h.pop().unwrap() - q;
            h.push(q);
            ans = ans.min(sum_q as f64 * wage[id[i]] as f64 / q as f64);
        }
    }
    ans
}

// https://leetcode.cn/problems/maximum-profit-in-job-scheduling/description/?envType=daily-question&envId=2024-05-04
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs = start_time
        .iter()
        .zip(end_time.iter())
        .zip(profit.iter())
        .map(|((&s, &e), &p)| (s, e, p))
        .collect::<Vec<_>>();
    jobs.sort_unstable_by(|a, b| a.1.cmp(&b.1)); // 按照结束时间排序

    let n = jobs.len();
    let mut f = vec![0; n + 1];
    for (i, &(st, _, p)) in jobs.iter().enumerate() {
        let j = jobs[..i].partition_point(|job| job.1 <= st);
        // 状态转移中，为什么是 j 不是 j+1：上面算的是 > st，-1 后得到 <= st，但由于还要 +1，抵消了
        f[i + 1] = f[i].max(f[j] + p);
    }
    f[n]
}

// 2391. 收集垃圾的最少总时间
pub fn calculate_garbage_collection_cost(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut total_cost = garbage[0].len() as i32;

    let mut unique_garbage_types = HashSet::new();

    for (garbage_at_house, &travel_distance) in garbage.iter().rev().zip(travel.iter().rev()) {
        unique_garbage_types.extend(garbage_at_house.chars());

        total_cost +=
            garbage_at_house.len() as i32 + travel_distance * unique_garbage_types.len() as i32;
    }

    total_cost
}

// 定义边，包括目标节点和权重
type Edge = (usize, u32);

// 带权重的图，使用邻接表表示
type Graph = HashMap<usize, Vec<Edge>>;

// 为了在优先队列中使用，需要定义状态和它的排序方式
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: usize,
}

// 为 State 实现 Ord，以便它可以在 BinaryHeap 中正确排序
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // 注意我们需要最小堆，所以使用 other 比较 self
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra 算法的实现
fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, u32> {
    // 用于跟踪到每个节点的最小成本
    let mut min_cost = HashMap::new();
    // 使用二叉堆作为优先队列
    let mut heap = BinaryHeap::new();

    // 初始化
    min_cost.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    // 探索图
    while let Some(State { cost, position }) = heap.pop() {
        // 如果我们找到了一个更小的路径，则跳过处理
        if let Some(&known_cost) = min_cost.get(&position) {
            if cost > known_cost {
                continue;
            }
        }

        // 探索所有邻居
        if let Some(edges) = graph.get(&position) {
            for &(neighbor, weight) in edges {
                let next = State {
                    cost: cost + weight,
                    position: neighbor,
                };

                if next.cost < *min_cost.get(&neighbor).unwrap_or(&u32::MAX) {
                    heap.push(next);
                    min_cost.insert(neighbor, next.cost);
                }
            }
        }
    }

    min_cost
}

// https://leetcode.cn/problems/word-break/description/
// 给你一个字符串 s 和一个字符串列表 wordDict 作为字典。
// 如果可以利用字典中出现的一个或多个单词拼接出 s 则返回 true。
// 注意：不要求字典中出现的单词全部都使用，并且字典中的单词可以重复使用。
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    let word_set = word_dict.into_iter().collect::<HashSet<_>>();
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && word_set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

// 教练使用整数数组 actions 记录一系列核心肌群训练项目编号。
// 为增强训练趣味性，需要将所有奇数编号训练项目调整至偶数编号训练项目之前。
// 请将调整后的训练项目编号以 数组 形式返回。
pub fn training_plan(actions: Vec<i32>) -> Vec<i32> {
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for &action in actions.iter() {
        if action % 2 == 0 {
            odd.push(action);
        } else {
            even.push(action);
        }
    }
    even.append(&mut odd);
    even
}

// 2244. 完成所有任务需要的最少轮数
pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let counter = tasks.iter().fold(HashMap::new(), |mut map, &task| {
        *map.entry(task).or_insert(0) += 1;
        map
    });
    let mut ans = 0;
    for (_, v) in counter.iter() {
        if *v == 1 {
            return -1;
        }
        ans += v / 3 + if v % 3 == 0 { 0 } else { 1 };
    }

    ans
}

// 250_Count_Number_of_Rectangles_Containing_Each_Point
pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    let mut count = vec![0; points.len()];
    for (i, point) in points.iter().enumerate() {
        for rectangle in rectangles.iter() {
            if point[0] >= 0
                && point[0] <= rectangle[0]
                && point[1] >= 0
                && point[1] <= rectangle[1]
            {
                count[i] += 1;
            }
        }
    }
    count
}

// 2589. Minimum Time to Complete All Tasks
pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
    let mut tasks = tasks;
    tasks.sort_unstable_by_key(|task| task[1]);
    let mut timeline = vec![0; 2001];

    for task in tasks.iter() {
        let (start, end, mut duration) = (task[0], task[1], task[2]);
        for time in start..=end {
            if duration == 0 {
                break;
            }
            // 尽可能的复用时间
            if timeline[time as usize] == 1 {
                duration -= 1;
            }
        }
        // 尽可能的推迟任务的执行时间
        for time in (start..=end).rev() {
            if duration == 0 {
                break;
            }
            if timeline[time as usize] == 0 {
                timeline[time as usize] = 1;
                duration -= 1;
            }
        }
    }

    let mut ans = 0;
    for time in timeline.iter() {
        if *time != 0 {
            ans += 1;
        }
    }

    ans
}

// 8. String to Integer (atoi)
pub fn my_atoi(s: String) -> i32 {
    let mut s = s.trim_start();
    let mut sign = 1;
    if s.starts_with('-') {
        sign = -1;
        s = &s[1..];
    } else if s.starts_with('+') {
        s = &s[1..];
    }

    let mut ans = 0;
    for c in s.chars() {
        if !c.is_ascii_digit() {
            break;
        }
        let digit = c.to_digit(10).unwrap() as i32;
        if ans > (i32::MAX - digit) / 10 {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }
        ans = ans * 10 + digit;
    }

    sign * ans
}

// 12. Integer to Roman
pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut ans = String::new();
    let symbols = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    for (value, symbol) in symbols {
        while num >= value {
            ans.push_str(symbol);
            num -= value;
        }
    }

    ans
}

// 13. Roman to Integer
pub fn roman_to_int(s: String) -> i32 {
    let mut ans = 0;
    let mut prev = 0;
    let symbols = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];

    for c in s.chars().rev() {
        let value = symbols.iter().find(|&&(symbol, _)| symbol == c).unwrap().1;
        if value < prev {
            ans -= value;
        } else {
            ans += value;
        }
        prev = value;
    }

    ans
}

// 38. Count and Say
pub fn count_and_say(n: i32) -> String {
    let mut ans = "1".to_string();
    for _ in 1..n {
        let mut temp = String::new();
        let mut i = 0;
        while i < ans.len() {
            let mut j = i + 1;
            while j < ans.len() && ans.as_bytes()[i] == ans.as_bytes()[j] {
                j += 1;
            }
            temp.push_str(&(j - i).to_string());
            temp.push(ans.as_bytes()[i] as char);
            i = j;
        }
        ans = temp;
    }
    ans
}

// 1953. Maximum Number of Weeks for Which You Can Work
pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    // Clone the input vector and sort it in non-decreasing order
    let mut milestones = milestones;
    milestones.sort_unstable();

    // Initialize a variable to store the sum of milestones
    let mut sum = 0;

    // Iterate through milestones except the last one
    for &milestone in milestones.iter().take(milestones.len() - 1) {
        // Add each milestone to the sum
        sum += milestone as i64;
    }

    // Get the value of the maximum milestone
    let max = milestones[milestones.len() - 1] as i64;

    // Check if the sum of milestones is greater than or equal to the maximum milestone
    if sum >= max {
        // If so, return the sum plus the maximum milestone
        sum + max
    } else {
        // Otherwise, return twice the sum plus one
        2 * sum + 1
    }
}

// 826. Most Profit Assigning Work
pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
    let mut jobs = difficulty.iter().zip(profit.iter()).collect::<Vec<_>>();
    jobs.sort_unstable();
    worker.sort_unstable();

    let mut max_profit = 0;
    let mut i = 0;
    let mut ans = 0;
    for ability in worker.iter() {
        while i < jobs.len() && jobs[i].0 <= ability {
            max_profit = max_profit.max(*jobs[i].1);
            i += 1;
        }
        ans += max_profit;
    }

    ans
}

// 到达第 K 级台阶的方案数
// https://leetcode.cn/contest/weekly-contest-398/problems/find-number-of-ways-to-reach-the-k-th-stair/
pub fn ways_to_reach_stair(k: i32) -> i32 {
    
}

fn main() {
    // 创建图
    let mut graph = Graph::new();
    graph.insert(0, vec![(1, 4), (2, 1)]);
    graph.insert(1, vec![(3, 1)]);
    graph.insert(2, vec![(1, 2), (3, 5)]);
    graph.insert(3, vec![(4, 3)]);
    graph.insert(4, vec![]);

    // 计算从顶点 0 开始的最短路径
    let distances = dijkstra(&graph, 0);
    println!("最短路径: {:?}", distances);
}
