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

// 1535. Find the Winner of an Array Game
pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut wins = 0;
    let mut max = arr[0];
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
            wins = 1;
        } else {
            wins += 1;
        }
        if wins == k {
            return max;
        }
    }
    max
}

// 1542. Find Longest Awesome Substring

pub fn longest_awesome(s: String) -> i32 {
    let mut prefix: HashMap<i32, i32> = HashMap::new();
    prefix.insert(0, -1);
    let mut ans = 0;
    let mut sequence = 0;

    for (j, ch) in s.chars().enumerate() {
        let digit = ch.to_digit(10).unwrap() as i32;
        sequence ^= 1 << digit;

        if let Some(&i) = prefix.get(&sequence) {
            ans = ans.max(j as i32 - i);
        } else {
            prefix.insert(sequence, j as i32);
        }

        for k in 0..10 {
            let toggled_sequence = sequence ^ (1 << k);
            if let Some(&i) = prefix.get(&toggled_sequence) {
                ans = ans.max(j as i32 - i);
            }
        }
    }

    ans
}

// 1177. Can Make Palindrome from Substring
pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let s = s.as_bytes();
    let n = s.len();
    let mut prefix = vec![vec![0; 26]; n + 1];
    for i in 0..n {
        for j in 0..26 {
            prefix[i + 1][j] = prefix[i][j];
        }
        prefix[i + 1][(s[i] - b'a') as usize] += 1;
    }

    let mut ans = Vec::new();
    for query in queries.iter() {
        let mut count = 0;
        for j in 0..26 {
            count += (prefix[query[1] as usize + 1][j] - prefix[query[0] as usize][j]) % 2;
        }
        ans.push(count / 2 <= query[2]);
    }
    ans
}

// 846. Hand of Straights
pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    let group_size = group_size as usize;
    let mut hand = hand;
    hand.sort_unstable();
    let mut counter = HashMap::new();
    for &card in hand.iter() {
        *counter.entry(card).or_insert(0) += 1;
    }

    for &card in hand.iter() {
        if counter[&card] == 0 {
            continue;
        }
        for i in 0..group_size {
            if let Some(&count) = counter.get(&(card + i as i32)) {
                if count == 0 {
                    return false;
                }
                *counter.get_mut(&(card + i as i32)).unwrap() -= 1;
            } else {
                return false;
            }
        }
    }

    true
}

// 2225. 找出输掉零场或一场比赛的玩家
pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut freq = HashMap::new();
    for match_ in matches {
        let (winner, loser) = (match_[0], match_[1]);
        freq.entry(winner).or_insert(0);
        *freq.entry(loser).or_insert(0) += 1;
    }

    let mut ans = vec![Vec::new(), Vec::new()];
    for (key, value) in freq {
        if value < 2 {
            ans[value as usize].push(key);
        }
    }
    ans[0].sort_unstable();
    ans[1].sort_unstable();
    ans
}

// 2226. Maximum Candies Allocated to K Children
pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let candies: Vec<i64> = candies.iter().map(|&x| x as i64).collect();
    let mut low = 0;
    let mut high = candies.iter().sum::<i64>() / k;

    while low < high {
        let mid = (low + high + 1) / 2;
        let s: i64 = candies.iter().map(|&v| v as i64 / mid as i64).sum();

        if s >= k {
            low = mid; // mid is a valid solution, try for a bigger one
        } else {
            high = mid - 1; // mid is too big, try a smaller one
        }
    }

    low as i32
}

// 2831. 找出最长等值子数组
pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut indexes_map: HashMap<i32, Vec<usize>> = HashMap::new();

    // Store indices for each number
    for (i, &num) in nums.iter().enumerate() {
        indexes_map.entry(num).or_insert_with(Vec::new).push(i);
    }

    let mut max_equal_length = 0;

    for indexes in indexes_map.values() {
        let mut j = 0;
        for i in 0..indexes.len() {
            // Expand j to maintain the condition
            while j < indexes.len() && (indexes[j] - indexes[i] - (j - i)) <= k as usize {
                j += 1;
            }
            max_equal_length = max_equal_length.max(j - i);
        }
    }

    max_equal_length as i32
}

// 1673. Find the Most Competitive Subsequence
// 贪心 + 单调栈
pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut stack = Vec::new();
    let n = nums.len();
    let k = k as usize;

    for (i, &num) in nums.iter().enumerate() {
        while !stack.is_empty() && stack.len() + n - i > k && *stack.last().unwrap() > num {
            stack.pop();
        }
        if stack.len() < k {
            stack.push(num);
        }
    }

    stack
}

// 2903_找出满足差值条件的下标_I
pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let mut max_idx = 0;
    let mut min_idx = 0;
    for j in index_difference as usize..nums.len() {
        let i = j - index_difference as usize;
        if nums[i] > nums[max_idx] {
            max_idx = i;
        } else if nums[i] < nums[min_idx] {
            min_idx = i;
        }
        if nums[max_idx] - nums[j] >= value_difference {
            return vec![max_idx as i32, j as i32];
        }
        if nums[j] - nums[min_idx] >= value_difference {
            return vec![min_idx as i32, j as i32];
        }
    }
    vec![-1, -1]
}

// 2904. 最短且字典序最小的美丽子字符串
pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let mut indexs = vec![];
    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            indexs.push(i);
        }
    }
    if indexs.len() < k as usize {
        return "".to_string();
    }

    let mut ans = s.clone();
    for i in 0..indexs.len() {
        if i + k as usize - 1 >= indexs.len() {
            break;
        }
        let tmp = s[indexs[i]..=indexs[i + k as usize - 1]].to_string();
        if tmp.len() < ans.len() {
            ans = tmp;
        } else if tmp.len() == ans.len() && tmp < ans {
            ans = tmp;
        }
    }
    ans
}

// 1738. 找出第 K 大的异或坐标值
pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut prefix = vec![vec![0; n + 1]; m + 1];
    let mut xor = vec![0; m * n];
    let mut idx = 0;

    for i in 1..=m {
        for j in 1..=n {
            prefix[i][j] =
                prefix[i - 1][j] ^ prefix[i][j - 1] ^ prefix[i - 1][j - 1] ^ matrix[i - 1][j - 1];
            xor[idx] = prefix[i][j];
            idx += 1;
        }
    }

    xor.sort_unstable();
    xor[xor.len() - k as usize]
}

// 2028. 找出缺失的观测数据
pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let sum: i32 = rolls.iter().sum();
    let total = (m + n) * mean;
    let mut ans = vec![1; n as usize];

    if total - sum > 6 * n || total - sum < n {
        return vec![];
    }

    let mut diff = total - sum - n;
    for i in 0..n as usize {
        let x = diff.min(5);
        ans[i] += x;
        diff -= x;
    }

    ans
}

// 2951. 找出峰值
pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 1..mountain.len() - 1 {
        if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
            ans.push(i as i32);
        }
    }
    ans
}

// 1103. 分糖果 II
pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let num_people = num_people as usize;
    let mut ans = vec![0; num_people];
    let mut candies = candies;
    let mut i = 0;
    while candies > 0 {
        ans[i % num_people] += candies.min(i as i32 + 1);
        candies -= i as i32 + 1;
        i += 1;
    }
    ans
}

// 3067. 在带权树网络中统计可连接服务器对数目
pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
    // TODO: 6/5/2024
    unimplemented!()
}

// 3072. 将元素分配到两个数组中 II
pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    // TODO: 6/5/2024
    unimplemented!()
}

// 2938. 区分黑球与白球
pub fn minimum_steps(s: String) -> i64 {
    let mut ans = 0i64;
    let mut sum = 0i64;
    for &item in s.chars().collect::<Vec<_>>().iter() {
        if item == '1' {
            sum += 1;
        } else {
            ans += sum;
        }
    }
    ans
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
