use std::collections::HashMap;
use std::collections::HashSet;

pub fn top_students(
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    student_id: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    let positive_feedback_set = positive_feedback.iter().collect::<HashSet<_>>();
    let negative_feedback_set = negative_feedback.iter().collect::<HashSet<_>>();

    let mut score = vec![0; student_id.len()];

    let report = report
        .into_iter()
        .map(|x| {
            x.to_string()
                .split_whitespace() // Using split_whitespace to handle potential spaces
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>();

    for i in 0..report.len() {
        for word in &report[i] {
            if positive_feedback_set.contains(word) {
                score[i] += 3;
            } else if negative_feedback_set.contains(word) {
                score[i] -= 1;
            }
        }
    }

    let mut ans: Vec<_> = score.iter().zip(&student_id).collect();

    ans.sort_by(|(s1, i1), (s2, i2)| match s2.cmp(s1) {
        std::cmp::Ordering::Equal => i1.cmp(i2),
        order => order,
    });

    ans.into_iter().take(k as usize).map(|(_, b)| *b).collect()
}

pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort_by(|a, b| b.cmp(a));

    let mut l = 0;
    let mut r = citations.len() as i32 - 1;

    while l <= r {
        let mid = l + (r - l) / 2;

        if mid + 1 <= citations[mid as usize] {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    r + 1
}

pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .iter()
        .filter(|s| s[s.len() - 4..s.len() - 2].parse::<i32>().unwrap_or(0) >= 60)
        .count() as i32
}

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;

    if target < n || target > k * n {
        return 0;
    }

    let mut dp = vec![0; target as usize + 1];
    let mut new_dp = vec![0; target as usize + 1];

    for i in 1..=k {
        if i <= target {
            dp[i as usize] = 1;
        }
    }

    for _ in 2..=n {
        for j in 1..=target as usize {
            for t in 1..=k as usize {
                if j >= t {
                    new_dp[j] += dp[j - t];
                    new_dp[j] %= MOD;
                }
            }
        }
        dp = new_dp.clone();
        new_dp = vec![0; target as usize + 1];
    }

    dp[target as usize]
}

pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut pre = 0;
    satisfaction.sort_unstable_by(|a, b| b.cmp(a));
    for (_, &v) in satisfaction.iter().enumerate() {
        if pre + v >= 0 {
            pre += v;
            ans += pre;
        } else {
            break;
        }
    }

    ans
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut tails = vec![nums[0]];

    for &n in &nums[1..] {
        let mut l = 0;
        let mut r = tails.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if tails[mid] < n {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        if l == tails.len() {
            tails.push(n)
        } else {
            tails[l] = n
        }
    }

    tails.len() as i32
}

pub fn max_product(words: Vec<String>) -> i32 {
    #[inline]
    fn str_to_set(str: &str) -> HashSet<char> {
        let mut set = HashSet::new();

        for c in str.chars() {
            set.insert(c);
        }

        set
    }

    let words_sets = words.iter().map(|s| str_to_set(&s)).collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..words_sets.len() - 1 {
        for j in i + 1..words_sets.len() {
            let intersection: HashSet<_> = words_sets[i].intersection(&words_sets[j]).collect();
            if intersection.is_empty() {
                ans = ans.max(words[i].len() * words[j].len());
            }
        }
    }

    ans as i32
}

struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [None, None],
        }
    }
}

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut root = TrieNode::new();
    let mut max_xor = 0;

    for &n in nums.iter() {
        let mut node = &mut root;
        for i in (0..32).rev() {
            let bit = (n >> i) & 1;
            if node.children[bit as usize].is_none() {
                node.children[bit as usize] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[bit as usize].as_mut().unwrap()
        }
    }

    for &n in nums.iter() {
        let mut node = &root;
        let mut cur_xor = 0;
        for i in (0..32).rev() {
            let bit = (n >> i) & 1;
            let opposite_bit = bit ^ 1;

            if let Some(opposite_child) = &node.children[opposite_bit as usize] {
                cur_xor |= 1 << i;
                node = opposite_child;
            } else {
                node = &node.children[bit as usize].as_ref().unwrap();
            }
        }

        max_xor = max_xor.max(cur_xor)
    }

    max_xor
}

pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let mut s: Vec<_> = s.chars().collect();
    s.push('0');
    let mut idx = 0;
    while idx < s.len() && s[idx] != '0' {
        idx += 1;
    }

    let mut cnts_0: Vec<i32> = vec![];
    let mut cnts_1: Vec<i32> = vec![];
    let mut len = 0;
    let mut pre = '0';
    while idx < s.len() {
        if s[idx] == pre {
            len += 1;
        } else {
            if pre == '0' {
                cnts_1.push(len);
            } else {
                cnts_0.push(len);
            }
            len = 1
        }
        pre = s[idx];
        idx += 1
    }

    let mut ans = 0;
    // println!("{:?}", (cnts_0.clone(),cnts_1.clone()));
    for i in 0..cnts_0.len().min(cnts_1.len()) {
        ans = ans.max(cnts_0[i].min(cnts_1[i]))
    }

    ans
}

pub fn distance(nums: Vec<i32>) -> Vec<i64> {
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        map.entry(n)
            .and_modify(|v| {
                v.push(i);
            })
            .or_insert_with(|| {
                let mut v = vec![];
                v.push(i);
                v
            });
    }

    let mut arr = vec![0; nums.len()];
    for a in map.values() {
        let len = a.len();
        let mut s = 0;
        for x in a {
            s += x - a[0]
        }
        arr[a[0]] = s as i64;

        for i in 1..len {
            if i * 2 >= len {
                s += (i * 2 - len) * (a[i] - a[i - 1]);
            } else {
                s -= (len - i * 2) * (a[i] - a[i - 1]);
            }
            arr[a[i]] = s as i64
        }
    }

    arr
}

// 超时
// pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
//     let mut ans = vec![];
//     spells.iter().for_each(|&s| {
//         ans.push(potions.iter().filter(|&p| (*p as i64 * s as i64) >= success).collect::<Vec<_>>().len() as i32);
//     });

//     ans
// }

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();

    #[inline]
    fn search(potions: &Vec<i32>, spell: i32, success: i64) -> usize {
        let mut l = 0;
        let mut r = potions.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if (potions[mid] as i64 * spell as i64) < success {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        l
    }

    spells
        .iter()
        .map(|&spell| (potions.len() - search(&potions, spell, success)) as i32)
        .collect()
}

pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
    let mut position: HashMap<i32, usize> = row.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut swaps = 0;

    let mut row = row;
    let n = row.len();

    for i in (0..n).step_by(2) {
        let x = row[i];
        let y = if x % 2 == 0 { x + 1 } else { x - 1 };

        if row[i + 1] != y {
            let partner_idx = position[&y];
            row.swap(i + 1, partner_idx);
            position.insert(row[partner_idx], partner_idx);
            position.insert(row[i + 1], i + 1);
            swaps += 1;
        }
    }

    swaps
}

pub fn min_operations(mut nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    // Early return if the last elements are the max of their respective vectors
    if nums1.last() == nums1.iter().max() && nums2.last() == nums2.iter().max() {
        return 0;
    }

    // Function to calculate operations needed for a given configuration
    fn calc_operations(nums1: &[i32], nums2: &[i32]) -> i32 {
        let mut operations = 0;
        let last_num1 = *nums1.last().unwrap();
        let last_num2 = *nums2.last().unwrap();

        for (&num1, &num2) in nums1.iter().zip(nums2.iter()).take(nums1.len() - 1) {
            if num1 <= last_num1 && num2 <= last_num2 {
                continue;
            } else if num1 <= last_num2 && num2 <= last_num1 {
                operations += 1;
            } else {
                return nums1.len() as i32;
            }
        }
        operations
    }

    let operations_normal = calc_operations(&nums1, &nums2);

    // Store the length in a variable to avoid simultaneous mutable and immutable borrows
    let nums1_len = nums1.len();

    // Swap the last elements of nums1 and nums2
    nums1.swap(nums1_len - 1, nums2.len() - 1);

    let operations_swapped = calc_operations(&nums1, &nums2) + 1;

    let min_operations = operations_normal.min(operations_swapped);

    if min_operations == nums1.len() as i32 {
        -1
    } else {
        min_operations
    }
}

pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort_unstable();
    let mut idx_map = HashMap::new();
    for i in 0..nums.len() {
        idx_map.insert(nums[i], i);
    }

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l <= r {
        let tmp = nums[l] + nums[r];
        if tmp < target {
            l += 1
        } else if tmp > target {
            r -= 1
        } else {
            break;
        }
    }

    vec![
        *idx_map.get(&nums[l]).unwrap() as i32,
        *idx_map.get(&nums[r]).unwrap() as i32,
    ]
}

pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut indexed_diff: Vec<_> = arr
        .iter()
        .enumerate()
        .map(|(idx, &n)| (idx, (n - x).abs()))
        .collect();

    // Partially sort the array to find the kth element
    let kth = k as usize - 1;
    indexed_diff.select_nth_unstable_by(kth, |a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    // Extract the first k elements and sort them
    let mut result: Vec<i32> = indexed_diff[..=kth]
        .iter()
        .map(|&(idx, _)| arr[idx])
        .collect();
    result.sort_unstable();

    result
}

pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let mid = l + (r - l) / 2;
        if nums[mid] > nums[mid + 1] {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    l as i32
}

pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    while r < nums.len() {
        if nums[l] == nums[r] {
            while r < nums.len() && nums[l] == nums[r] {
                l += 1;
                r += 1;
            }
        } else {
            l += 2;
            r += 2;
        }
    }
    if l == nums.len() - 1 {
        ans += 1
    }
    ans
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    if nums[0] < nums[nums.len() - 1] {
        return nums[0];
    }

    if nums[nums.len() - 1] < nums[0] && nums[0] <= nums[nums.len() - 2] {
        return nums[nums.len() - 1];
    }

    #[inline]
    fn check(nums: &[i32], idx: usize) -> bool {
        if nums[idx] >= nums[0] {
            true
        } else {
            false
        }
    }

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let mid = l + (r - l) / 2;
        println!("{:?}", (l, r, mid));
        if check(&nums, mid) {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    return nums[l];
}

// Quotation Mark: the entity is &quot; and symbol character is ".
// Single Quote Mark: the entity is &apos; and symbol character is '.
// Ampersand: the entity is &amp; and symbol character is &.
// Greater Than Sign: the entity is &gt; and symbol character is >.
// Less Than Sign: the entity is &lt; and symbol character is <.
// Slash: the entity is &frasl; and symbol character is /.

pub fn entity_parser(text: String) -> String {
    let mut result = text;
    result = result.replace("&quot;", "\"");
    result = result.replace("&apos;", "'");
    result = result.replace("&amp;", "&");
    result = result.replace("&gt;", ">");
    result = result.replace("&lt;", "<");
    result = result.replace("&frasl;", "/");
    result
}

pub fn unique_letter_string(s: String) -> i32 {
    let len = s.len();
    let mut cnts = vec![];
    let mut cur_cnt = vec![0; 26]; // 对于大写字母，数组大小为 26

    for byte in s.as_bytes() {
        // 计算大写字母的索引
        if *byte >= b'A' && *byte <= b'Z' {
            let idx = *byte - b'A';
            cur_cnt[idx as usize] += 1;
            cnts.push(cur_cnt.clone());
        }
    }

    let mut ans = 0;
    for i in 0..len {
        for j in i + 1..len {
            let mut unique = true;
            for idx in 0..26 {
                // 检查是否有字符在子字符串中出现超过一次
                if cnts[j][idx] - cnts[i][idx] > 1 {
                    unique = false;
                    break;
                }
            }
            if unique {
                ans += 1;
            }
        }
    }

    ans
}

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    #[inline]
    fn find_position(n: i32, position_map: &HashMap<i32, (usize, usize)>) -> (usize, usize) {
        *position_map.get(&n).unwrap()
    }

    let mut row = vec![HashSet::new(); mat.len()];
    let mut col = vec![HashSet::new(); mat[0].len()];
    let mut position_map: HashMap<i32, (usize, usize)> = HashMap::new();

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            position_map.insert(mat[i][j], (i, j));
        }
    }

    for i in 0..arr.len() {
        let n = arr[i];
        let (x, y) = find_position(n, &position_map);
        row[x].insert(n);
        col[y].insert(n);
        // println!("{:?}", row[x]);
        // println!("{:?}", low[y]);
        if row[x].len() == mat[0].len() || col[y].len() == mat.len() {
            return i as i32;
        }
    }

    unimplemented!()
}

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let k = k as usize; // Convert k to usize for indexing
    let total_points: i32 = card_points.iter().sum();
    let window_size = card_points.len() - k;
    let mut min_subarray_sum = card_points.iter().take(window_size).sum::<i32>();
    let mut current_sum = min_subarray_sum;

    for i in 0..k {
        // Slide the window by one position
        current_sum = current_sum - card_points[i] + card_points[i + window_size];
        min_subarray_sum = min_subarray_sum.min(current_sum);
    }

    total_points - min_subarray_sum
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::process::id;
use std::rc::Rc;

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
        if let Some(x) = node {
            let mut x = x.borrow_mut();
            dfs(x.right.as_ref(), s);
            *s += x.val;
            x.val = *s;
            dfs(x.left.as_ref(), s);
        }
    }
    let mut s = 0;
    dfs(root.as_ref(), &mut s);
    root
}

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g = vec![vec![]; n as usize];
    for c in &connections {
        let (f, t) = (c[0] as usize, c[1] as usize);
        g[f].push(t);
        g[t].push(f);
    }

    // println!("{:?}", g);

    let paths: HashSet<(i32, i32)> = connections
        .iter()
        .flat_map(|x| vec![(x[0], x[1])])
        .collect();

    let mut ans = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    fn dfs(
        node: usize,
        g: &[Vec<usize>],
        paths: &HashSet<(i32, i32)>,
        ans: &mut i32,
        visited: &mut HashSet<usize>,
    ) {
        visited.insert(node);
        // println!("{:?}", g[node]);
        // println!("visited: {:?}", visited);
        // println!("current node:{:?}", node);

        for &n in &g[node] {
            // println!("node -> n{:?}", (node, n));
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
                    graph
                        .entry((i, j))
                        .or_insert_with(Vec::new)
                        .push(((ni as usize, nj as usize), cost));
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

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![0; n];

    dp[0] = cost[0];
    dp[1] = cost[1];

    for i in 2..n {
        dp[i] = cost[i] + dp[i - 1].min(dp[i - 2]);
    }

    dp[n - 1].min(dp[n - 2])
}

pub fn make_smallest_palindrome(s: String) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    let n = cs.len();
    for i in 0..n / 2 {
        let j = n - 1 - i;
        cs[i] = std::cmp::min(cs[i], cs[j]);
        cs[j] = cs[i];
    }
    cs.into_iter().collect()
}

// Import BTreeMap from the standard collections library
use std::collections::BTreeMap;

// Define the CountIntervals struct
struct CountIntervals {
    // A BTreeMap to store intervals in a sorted order.
    mp: BTreeMap<i32, i32>,
    // An integer to keep track of the total count of unique elements in all intervals.
    cnt: i32,
}

// Implementation block for CountIntervals
impl CountIntervals {
    // Constructor for CountIntervals
    fn new() -> Self {
        CountIntervals {
            // Initialize an empty BTreeMap for storing intervals
            mp: BTreeMap::new(),
            // Initialize the count to 0
            cnt: 0,
        }
    }

    // Method to add a new interval to the collection
    fn add(&mut self, mut left: i32, mut right: i32) {
        // Find the last interval that might overlap with the new interval
        let mut interval_index = self.mp.range(..=right).next_back();

        // Loop to merge overlapping intervals
        while let Some((&l, &r)) = interval_index {
            // Break the loop if there is no overlap
            if l > right || r < left {
                break;
            }

            // Update the bounds of the new interval to encompass the current interval
            left = left.min(l);
            right = right.max(r);

            // Update the count by removing the size of the current interval
            self.cnt -= r - l + 1;
            // Remove the current interval from the map
            self.mp.remove(&l);

            // Look for the next interval that might overlap
            interval_index = self.mp.range(..=right).next_back();
        }

        // Add the size of the new/merged interval to the total count
        self.cnt += right - left + 1;
        // Insert the new/merged interval into the map
        self.mp.insert(left, right);
    }

    // Method to get the total count of unique elements in all intervals
    fn count(&self) -> i32 {
        self.cnt
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */

// 二维前缀和 + 二维差分数组
pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();

    // 初始化二维前缀和
    // 定义sum[i+1][j+1]为左上角为[0,0]，右下角为[i,j]的子矩阵元素和
    // 计算任意矩阵的元素和
    // 假设子矩阵的左上角为[r1, c1], 右下角为[r2-1, c2-1]
    // 则该子矩阵的和为 sum[r2][c2] - sum[r1][c2] - sum[r2][c1] + sum[r1][c1]
    let mut s = vec![vec![0; n + 1]; m + 1];
    for (i, row) in grid.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + x
        }
    }

    // 计算二维差分
    let mut d = vec![vec![0; n + 2]; m + 2];
    for i2 in stamp_height as usize..=m {
        for j2 in stamp_width as usize..=n {
            let i1 = i2 - stamp_height as usize + 1;
            let j1 = j2 - stamp_width as usize + 1;
            if s[i2][j2] - s[i2][j1 - 1] - s[i1 - 1][j2] + s[i1 - 1][j1 - 1] == 0 {
                d[i1][j1] += 1;
                d[i1][j2 + 1] -= 1;
                d[i2 + 1][j1] -= 1;
                d[i2 + 1][j2 + 1] += 1;
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            d[i + 1][j + 1] += d[i + 1][j] + d[i][j + 1] - d[i][j];
            if v == 0 && d[i + 1][j + 1] == 0 {
                return false;
            }
        }
    }

    true
}

struct MagicDictionary {
    set: HashSet<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            set: HashSet::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.set = dictionary.into_iter().collect();
    }

    fn search(&self, search_word: String) -> bool {
        self.set.iter().any(|word| {
            if word.len() == search_word.len() {
                word.chars()
                    .zip(search_word.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1
            } else {
                false
            }
        })
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut l = 0;
    let mut r = cols - 1;

    while l <= r {
        let mid = l + (r - l) / 2;

        // Find the maximum element in the middle column
        let (mut idx, mut max_num_in_center) = (0, 0);
        for i in 0..rows {
            if mat[i][mid] > max_num_in_center {
                idx = i;
                max_num_in_center = mat[i][mid];
            }
        }

        // Check if the maximum element is a peak
        let left_is_smaller = mid == 0 || max_num_in_center > mat[idx][mid - 1];
        let right_is_smaller = mid == cols - 1 || max_num_in_center > mat[idx][mid + 1];

        if left_is_smaller && right_is_smaller {
            return vec![idx as i32, mid as i32];
        } else if !left_is_smaller {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    unreachable!()
}

use rand::Rng; // For random number generation
use std::iter::Iterator; // For iterator methods

struct Solution {
    cumulative_weights: Vec<f64>,
}

impl Solution {
    fn new(weights: Vec<i32>) -> Self {
        let total_weight: f64 = weights.iter().map(|&w| w as f64).sum();
        let cumulative_weights: Vec<f64> = weights
            .iter()
            .map(|&w| w as f64 / total_weight)
            .scan(0.0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect();

        Solution { cumulative_weights }
    }

    fn pick_index(&self) -> usize {
        let rand_num = rand::thread_rng().gen::<f64>();
        match self
            .cumulative_weights
            .binary_search_by(|&w| w.partial_cmp(&rand_num).unwrap())
        {
            Ok(index) => index,
            Err(index) => index,
        }
    }
}

fn main() {
    let solution = Solution::new(vec![1, 3, 2]); // Example weights
    println!("Picked index: {}", solution.pick_index());
}
