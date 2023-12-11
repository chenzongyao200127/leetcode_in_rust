use std::collections::HashSet;
use std::collections::HashMap;


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
        .map(|x| {x
                .to_string()
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

    ans.sort_by(|(s1, i1), (s2, i2)| {
        match s2.cmp(s1) {
            std::cmp::Ordering::Equal => i1.cmp(i2),
            order => order,
        }
    });

    ans.into_iter()
        .take(k as usize)
        .map(|(_, b)| *b)
        .collect()
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
    details.iter()
           .filter(|s| s[s.len()-4..s.len()-2].parse::<i32>().unwrap_or(0) >= 60)
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
                    new_dp[j] += dp[j-t];
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

    let words_sets = words.iter().map(|s| {
        str_to_set(&s)
    }).collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..words_sets.len()-1 {
        for j in i+1..words_sets.len() {
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
        map.entry(n).and_modify(|v| { v.push(i); }).or_insert_with(|| {
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
            if i* 2 >= len {
                s += (i * 2 - len) * (a[i] - a[i-1]);
            } else {
                s -= (len - i * 2) * (a[i] - a[i-1]);
            }
            arr[a[i]] = s as  i64
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

    spells.iter().map(|&spell| (potions.len() - search(&potions, spell, success)) as i32).collect()
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

    vec![*idx_map.get(&nums[l]).unwrap() as i32, *idx_map.get(&nums[r]).unwrap() as i32]
}

pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut indexed_diff: Vec<_> = arr.iter()
                                      .enumerate()
                                      .map(|(idx, &n)| (idx, (n - x).abs()))
                                      .collect();

    // Partially sort the array to find the kth element
    let kth = k as usize - 1;
    indexed_diff.select_nth_unstable_by(kth, |a, b| {
        a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
    });

    // Extract the first k elements and sort them
    let mut result: Vec<i32> = indexed_diff[..=kth].iter().map(|&(idx, _)| arr[idx]).collect();
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

    if nums[0] < nums[nums.len()-1] {
        return nums[0];
    }

    if nums[nums.len()-1] < nums[0] && nums[0] <= nums[nums.len()-2] {
        return nums[nums.len()-1];
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

    return nums[l]
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
    fn find_position(n: i32, position_map: &HashMap<i32, (usize, usize)>)-> (usize, usize) {
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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

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

pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
    
    let mut total_cnts = vec![0; n as usize];

    let mut g = vec![vec![]; n as usize];
    for e in &edges {
        let x = e[0] as usize;
        let y = e[1] as usize;
        g[x].push(y); // 记录每个点的邻居
        g[y].push(x);
    }

    let mut visited: HashSet<usize> = HashSet::new();

    #[inline]
    fn get_paths(start: usize, end: usize, g: &Vec<Vec<usize>>, cnts: &mut Vec<i32>, visited: &mut HashSet<usize>, total_cnts: &mut Vec<i32>) {
        // Add the current node to the visited set
        visited.insert(start);
    
        // Increment the count for the current node
        cnts[start] += 1;
    
        // Base case: if the current node is the end node, return.
        if start == end {
            for (i, cnt) in cnts.iter().enumerate() {
                total_cnts[i] += cnt;
            }
            return;
        }
    
        // Explore neighbors
        for &neig in &g[start] {
            if !visited.contains(&neig) {
                get_paths(neig, end, g, cnts, visited, total_cnts);
            }
        }
    
        // Decrement the count and remove the current node from the visited set on backtracking
        cnts[start] -= 1;
        visited.remove(&start);
    }

    for trip in trips.iter() {
        let start = trip[0] as usize;
        let end = trip[1] as usize;

        visited.clear();
        let mut local_cnts = vec![0; n as usize];
        get_paths(start, end, &g, &mut local_cnts, &mut visited, &mut total_cnts);
    }

    #[inline]
    fn dp(node: usize, parent: usize, price: &Vec<i32>, total_cnts: &Vec<i32>, g: &Vec<Vec<usize>>) -> (i32, i32) {
        let mut res_with_halved = price[node] * total_cnts[node];
        let mut res_without_halved = price[node] / 2 * total_cnts[node];

        for &child in g[node].iter() {
            if child == parent {
                continue;
            }

            let (x, y) = dp(child, node, price, total_cnts, g);
            res_with_halved += x.min(y);
            res_without_halved += x;
        }

        return (res_with_halved, res_without_halved)
    }
    
    let (x, y) = dp(0, usize::MAX, &price, &total_cnts, &g);
    x.min(y)
}


pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g = vec![vec![]; n as usize];
    for c in &connections {
        let (f, t) = (c[0] as usize, c[1] as usize);
        g[f].push(t);
        g[t].push(f);
    }

    // println!("{:?}", g);

    let paths: HashSet<(i32, i32)> = connections.iter()
        .flat_map(|x| vec![(x[0], x[1])])
        .collect();

    let mut ans = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    fn dfs(node: usize, g: &[Vec<usize>], paths: &HashSet<(i32, i32)>, ans: &mut i32, visited: &mut HashSet<usize>) {
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
    // construct a graph
    let mut g: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let m = heights.len();
    let n = heights[0].len();
    let mut max_cost = 0;

    for i in 0..m {
        for j in 0..n {
            // 上下左右
            for (dx, dy) in &[(-1,0), (1,0), (0,1), (0,-1)] {
                let di = i as i32 + dx;
                let dj = j as i32 + dy;
                let mut cost = usize::MAX;
                if di >= 0 && di <= m as i32 - 1 && dj >= 0 && dj <= n as i32 - 1 {
                    cost = (heights[di as usize][dj as usize] - heights[i][j]).abs() as usize;
                    max_cost = max_cost.max(cost);
                    g.entry((i, j)).and_modify(|v| v.push(cost)).or_insert(vec![cost]);
                    continue;
                }
                g.entry((i, j)).and_modify(|v| v.push(cost)).or_insert(vec![cost]);
            }
        }
    }

    // given threshold k check if available    
    #[inline]
    fn is_available(
        k: i32, 
        m: usize, 
        n: usize, 
        curr_node: (usize, usize), 
        g: &HashMap<(usize, usize), Vec<usize>>, 
        visited: &mut HashSet<(usize, usize)>
    ) -> bool {
        if curr_node == (m-1, n-1) {
            return true;
        }
    
        if visited.contains(&curr_node) {
            return false;
        }
    
        visited.insert(curr_node);
    
        for (idx, (dx, dy)) in vec![(-1,0), (1,0), (0,1), (0,-1)].iter().enumerate() {
            let next_node = ((curr_node.0 as i32 + dx) as usize, (curr_node.1 as i32 + dy) as usize);
    
            // Check for bounds and visited
            if next_node.0 < m && next_node.1 < n && g.get(&curr_node).unwrap()[idx] < k as usize && !visited.contains(&next_node) {
                if is_available(k, m, n, next_node, g, visited) {
                    return true;
                }
            }
        }
    
        visited.remove(&curr_node); // Backtracking
        false
    }
    

    // binary search
    let mut l = 0;
    let mut r = max_cost + 1;
    let mut visited = HashSet::new();

    while l < r {
        let mid = l + (r - l) / 2;
        println!("{:?}", (l , r, mid));
        if is_available(mid as i32, m, n, (0, 0), &g, &mut visited) {
            r = mid;
        } else {
            l = mid + 1
        }
        visited.clear()
    }

    return l as i32;
}


fn main() {
    // let ans = find_closest_elements(vec![1,2,3,4,5], 4, 3);
    // println!("{:?}", ans);

    let ans =minimum_effort_path(vec![vec![1,2,2],vec![3,8,2],vec![5,3,5]]);
    println!("{:?}", ans);
}
