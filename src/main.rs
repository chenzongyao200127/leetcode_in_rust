

pub fn num_distinct(s: String, t: String) -> i32 {
    let s: Vec<_> = s.chars().collect();
    let t: Vec<_> = t.chars().collect();

    let mut dp = vec![vec![0; s.len()+1]; t.len()+1];

    for i in 0..s.len() {
        if s[i] == t[0] {
            dp[1][i+1] = dp[1][i] + 1;
        } else {
            dp[1][i+1] = dp[1][i];
        }
    }

    for i in 1..t.len() {
        for j in 0..s.len() {
            if s[j] == t[i] {
                dp[i+1][j+1] = dp[i+1][j] + dp[i][j];
            } else {
                dp[i+1][j+1] = dp[i+1][j]
            }
        }
    }

    println!("{:?}", dp);

    dp[t.len()][s.len()]
}




pub fn min_cut(s: String) -> i32 {
    let mut s1 = s.clone();
    let mut s2: String;
    let mut cnt = 0;

    loop {
        s2 = s1.chars().rev().collect();
        match longest_common_substring(&s1, &s2) {
            Some(common) => {
                s1 = remove_substring_from(&s1, common);
                cnt += 1;
            },
            None => break,
        }
    }

    cnt
}

pub fn longest_common_substring<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let mut longest_len = 0;
    let mut longest_pos = 0;

    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1_bytes[i-1] == s2_bytes[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
                if dp[i][j] > longest_len {
                    longest_len = dp[i][j];
                    longest_pos = i - longest_len;
                }
            }
        }
    }

    if longest_len == 0 {
        None
    } else {
        Some(&s1[longest_pos..longest_pos + longest_len])
    }
}

pub fn remove_substring_from(s: &str, sub: &str) -> String {
    s.replace(sub, "")
}


use std::collections::HashMap;

pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
        return false;
    }

    if desired_total <= max_choosable_integer {
        return true;
    }

    fn dfs(
        state: i32, 
        cur_total: i32, 
        max_choosable_integer: i32, 
        desired_total: i32, 
        memo: &mut HashMap<i32, bool>
    ) -> bool {
        if let Some(&v) = memo.get(&state) {
            return v;
        }

        for i in (1..=max_choosable_integer).rev() {
            if (state & (1 << (i - 1))) == 0 {
                if cur_total + i >= desired_total 
                    || !dfs(
                        state | (1 << (i - 1)), 
                        cur_total + i, 
                        max_choosable_integer, 
                        desired_total, 
                        memo
                    ) {
                    memo.insert(state, true);
                    return true;
                }
            }
        }

        memo.insert(state, false);
        return false;
    }

    let mut memo = HashMap::new();
    dfs(0, 0, max_choosable_integer, desired_total, &mut memo)
}



pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let n = seats.len();
    let mut l: Option<usize> = None; // Initialize l as None.
    let mut max_distance = 0;

    for i in 0..n {
        if seats[i] == 1 {
            if l.is_none() { // Check if l is None.
                max_distance = i as i32;
            } else {
                // Unwrap safely because we checked with is_none().
                max_distance = max_distance.max((i - l.unwrap()) as i32 / 2);
            }
            l = Some(i);
        }
    }

    // For the case where the last seated person is not at the end.
    if seats[n-1] == 0 {
        max_distance = max_distance.max((n - 1 - l.unwrap()) as i32);
    }

    max_distance
}



    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        if sum < target as i64 {
            return -1;
        }

        let mut count = HashMap::new();
        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut operations: i64 = 0;
        let mut total_sum: i64 = 0;

        for i in 0..31 {
            total_sum += *count.get(&(1 << i)).unwrap_or(&0) as i64 * (1 << i) as i64;

            if (target >> i & 1) == 0 {
                continue;
            }

            total_sum -= (1 << i) as i64;

            if total_sum >= 0 {
                continue;
            }

            for j in (i + 1)..31 {
                if let Some(cnt) = count.get_mut(&(1 << j)) {
                    if *cnt > 0 {
                        operations += j as i64 - i as i64;
                        *cnt -= 1;
                        total_sum += (1 << j) as i64;
                        break;
                    }
                }
            }
        }

        operations as i32
    }



pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let states = 1 << n;
    let mut dp = vec![i32::MAX; states as usize];
    let target_ch: Vec<char> = target.chars().collect();
    dp[0] = 0;

    for s in 0..states as usize {
        if dp[s] == i32::MAX {
            continue;
        }

        for strick in stickers.iter() {
            let mut nxt = s;
            for ch in strick.chars() {
                for k in 0..n {
                    if ch == target_ch[k] && s & (1 << k) == 0 {
                        nxt |= 1 << k;
                        println!("{:?}", nxt);
                        break;
                    }
                }
            }

            dp[nxt] = dp[nxt].min(dp[s] + 1);
        }
    }


    if dp[dp.len() - 1] == i32::MAX {
        -1
    } else {
        dp[dp.len() - 1]
    }
}


pub fn num_factored_binary_trees(arr: Vec<i32>) -> i64 {
    const MOD: i32 = 1_000_000_007;
    
    let mut arr = arr.clone();
    arr.sort();
    
    let index: HashMap<i32, usize> = arr.iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();
    
    let mut dp = vec![1; arr.len()];
    
    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i] % arr[j] == 0 {
                let right_child = arr[i] / arr[j];
                if let Some(&right_index) = index.get(&right_child) {
                    dp[i] = (dp[i] + dp[j] as i64 * dp[right_index] as i64 % MOD as i64) % MOD as i64;
                }
            }
        }
    }
    
    dp.iter().sum::<i64>() % MOD as i64
}

pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let total: i32 = nums.iter().sum();
    if total % k != 0 {
        return false;
    }

    let target = total / k;
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(a));
    
    if nums[0] > target {
        return false;
    }

    let mut visited = vec![false; nums.len()];
    can_partition(&nums, &mut visited, 0, k, 0, target)
}

fn can_partition(nums: &Vec<i32>, visited: &mut Vec<bool>, start: usize, k: i32, curr_sum: i32, target: i32) -> bool {
    if k == 1 {
        return true;
    }

    if curr_sum == target {
        return  can_partition(nums, visited, 0, k - 1, 0, target);
    }
    
    for i in start..nums.len() {
        if !visited[i] && curr_sum + nums[i] <= target {
            visited[i] = true;
            if  can_partition(nums, visited, i + 1, k, curr_sum + nums[i], target) {
                return true;
            }
            visited[i] = false;
        }
    }
    false
}

pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let total = total as i64;
    let cost1 = cost1 as i64;
    let cost2 = cost2 as i64;
    let mut ans = 0;
    let mut cnt = 0;

    while cnt * cost1 <= total {
        let t = total - cost1 * cnt;
        if t < cost2 {
            ans += 1
        } else {
            ans += t / cost2 + 1;
        }
        cnt += 1;
    }

    ans
}

pub fn minimum_operations(num: String) -> i32 {
    // Function to find the last occurrences of two target characters
    fn find_last_two_indices(num: &str, target1: char, target2: char) -> (i32, i32) {
        let idx2 = num.rfind(target2).unwrap_or(usize::MAX);
        if idx2 == usize::MAX {
            return (-1, -1);
        }
        let idx1 = num[..idx2 as usize].rfind(target1).unwrap_or(usize::MAX);
        (idx1 as i32, idx2 as i32)
    }

    // Function to count deletions required to ensure no other characters between idx1 and idx2
    fn count_deletions(num_len: usize, idx1: i32, idx2: i32) -> i32 {
        if idx1 == -1 || idx2 == -1 {
            return std::i32::MAX;
        }
        ((num_len - 1) as i32 - idx2 + idx2 - idx1 - 1) as i32
    }

    let num_len = num.len();
    
    // For a single character string
    if num_len == 1 {
        return if num != "0" { 1 } else { 0 };
    }

    // Special case for a single '0' in the string
    let single_zero = if num.contains('0') {
        (num_len - 1) as i32
    } else {
        std::i32::MAX
    };

    // Find the indices for the combinations '00', '25', '50', and '75'
    let idx00 = find_last_two_indices(&num, '0', '0');
    let idx25 = find_last_two_indices(&num, '2', '5');
    let idx50 = find_last_two_indices(&num, '5', '0');
    let idx75 = find_last_two_indices(&num, '7', '5');

    // Return the minimum of all the scenarios
    [
        single_zero,
        count_deletions(num_len, idx00.0, idx00.1),
        count_deletions(num_len, idx25.0, idx25.1),
        count_deletions(num_len, idx50.0, idx50.1),
        count_deletions(num_len, idx75.0, idx75.1),
        num_len as i32,
    ]
    .iter()
    .cloned()
    .min()
    .unwrap()
}

pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i32 {
    let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let modulo = modulo as i64;
    let k = k as i64;
    
    // Initialize a vector to store the prefix sums.
    let mut prefix = vec![0; nums.len() + 1];
    
    // Initialize a HashMap to count occurrences of prefix sums modulo the given value.
    let mut count: HashMap<i64, i64> = HashMap::new();
    count.insert(0, 1);
    
    let mut res = 0;

    for i in 0..nums.len() {
        // If the current number modulo is k, then increment the prefix sum.
        if nums[i] % modulo == k {
            prefix[i + 1] = prefix[i] + 1;
        } else {
            prefix[i + 1] = prefix[i];
        }
        
        // Calculate the target value we expect to have seen in previous prefix sums.
        let target = (prefix[i + 1] - k).rem_euclid(modulo);
        
        // If the target value has been seen before, add its count to the result.
        res += *count.get(&target).unwrap_or(&0);
        
        // Increment the count of the current prefix sum modulo.
        let entry = count.entry(prefix[i + 1].rem_euclid(modulo)).or_insert(0);
        *entry += 1;
    }

    res as i32
}


struct Codec {
    
}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        self._serialize(root, &mut res);
        res
    }
    
    fn _serialize(&self, node: Option<Rc<RefCell<TreeNode>>>, res: &mut String) {
        if let Some(n) = node {
            res.push_str(&n.borrow().val.to_string());
            res.push(',');
            self._serialize(n.borrow().left.clone(), res);
            self._serialize(n.borrow().right.clone(), res);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals: Vec<i32> = data.split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        self._deserialize(&mut vals, i32::MIN, i32::MAX)
    }
    
    fn _deserialize(&self, vals: &mut Vec<i32>, lower: i32, upper: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let val = *vals.first().unwrap();
        if val < lower || val > upper {
            return None;
        }
        
        vals.remove(0);
        let root = Rc::new(RefCell::new(TreeNode::new(val)));
        root.borrow_mut().left = self._deserialize(vals, lower, val);
        root.borrow_mut().right = self._deserialize(vals, val, upper);
        
        Some(root)
    }
}

use::std::cmp::max;

pub fn max_coins(mut nums: Vec<i32>) -> i32 {
    let mut new_nums = vec![1];
    new_nums.append(&mut nums);
    new_nums.push(1);
    println!("{:?}", new_nums);
    let l = new_nums.len();

    let mut dp = vec![vec![0; l]; l];

    for gap in 2..l {
        for i in 0..l-gap {
            let j = i + gap;
            for k in i+1..j {
                dp[i][j] = max(dp[i][j], dp[i][k] + dp[k][j] + new_nums[i] * new_nums[k] * new_nums[j]);
            }
        }
    }

    dp[0][l-1]
}

use std::rc::Rc;
use std::cell::RefCell;

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

pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    #[inline]
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = node {
            let node = node.borrow();

            let (l_rob, l_not_rob) = dfs(&node.left);
            let (r_rob, r_not_rob) = dfs(&node.right);
            
            let rob = l_not_rob + r_not_rob + node.val;
            let not_rob = l_rob.max(l_not_rob) + r_rob.max(r_not_rob);
            
            (rob, not_rob)
        } else {
            (0, 0)
        }
    }

    dfs(&root).0.max(dfs(&root).1)
}

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    fn can_fixed(time: i64, ranks: &Vec<i32>, cars: i32) -> bool {
        let mut tmp = 0;
        for &r in ranks {
            tmp += ((time / r as i64) as f64).sqrt().floor() as i32;
        }
        tmp >= cars
    }
    
    let mut ranks = ranks;
    ranks.sort();
    
    let mut l = 0;
    let mut r = (ranks[ranks.len() - 1] as i64) * (cars as i64) * (cars as i64) + 1;
    while l < r {
        let mid = (l + r) / 2;
        if can_fixed(mid, &ranks, cars) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}


use std::collections::VecDeque;

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prereq_cnts = vec![0; num_courses as usize];

    let mut course_map = HashMap::new();

    for pair in &prerequisites {
        let (course, prereq_course) = (pair[0] as usize, pair[1] as usize);
        prereq_cnts[course] += 1;
        course_map.entry(prereq_course).or_insert(vec![]).push(course);
    }

    let mut queue = VecDeque::new();

    for (i, &cnt) in prereq_cnts.iter().enumerate() {
        if cnt == 0 {
            queue.push_back(i);
        }
    }

    let mut res = vec![];

    while let Some(course) = queue.pop_front() {
        res.push(course as i32);

        if let Some(next_course_seqs) = course_map.get(&course) {
            for &next_course in next_course_seqs {
                prereq_cnts[next_course] -= 1;
                if prereq_cnts[next_course] == 0 {
                    queue.push_back(next_course);
                }
            }
        }
    }

    if res.len() == num_courses as usize {
        res
    } else {
        vec![]
    }
}


fn main() {
    println!("{:?}", find_order(4, vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]])); // True
}
