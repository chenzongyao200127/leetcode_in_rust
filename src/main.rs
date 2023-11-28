use std::process::id;
use std::vec;
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

fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mod_val = 1_000_000_007;
    let mut dp = vec![0; len];
    let mut stack: Vec<usize> = vec![];

    dp[0] = arr[0];
    stack.push(0);

    for i in 1..len {
        while let Some(&j) = stack.last() {
            if arr[j] > arr[i] {
                stack.pop();
            } else {
                break;
            }
        }

        let &j = stack.last().unwrap_or(&i);
        if j == i {
            dp[i] = arr[i] * (i+1) as i32;
        } else {
            dp[i] = (dp[j] + arr[i] * (i-j) as i32) % mod_val;
        }

        stack.push(i);
    }

    (dp.iter().sum::<i32>()) % mod_val
}

use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }

    // 调整长度，保证 0 <= right.len() - left.len() <= 1
    // 从而保证可以在正中间插入删除元素
    fn balance(&mut self) {
        if self.left.len() > self.right.len() {
            self.right.push_front(self.left.pop_back().unwrap());
        } else if self.right.len() > self.left.len() + 1 {
            self.left.push_back(self.right.pop_front().unwrap());
        }
    }

    fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.balance();
    }

    fn push_middle(&mut self, val: i32) {
        if self.left.len() < self.right.len() {
            self.left.push_back(val);
        } else {
            self.right.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.balance();
    }

    fn pop_front(&mut self) -> i32 {
        if self.right.is_empty() { // 整个队列为空
            return -1;
        }
        let val = if self.left.is_empty() {
            self.right.pop_front().unwrap()
        } else {
            self.left.pop_front().unwrap()
        };
        self.balance();
        val
    }

    fn pop_middle(&mut self) -> i32 {
        if self.right.is_empty() { // 整个队列为空
            return -1;
        }
        if self.left.len() == self.right.len() {
            return self.left.pop_back().unwrap();
        }
        self.right.pop_front().unwrap()
    }

    fn pop_back(&mut self) -> i32 {
        if self.right.is_empty() { // 整个队列为空
            return -1;
        }
        let val = self.right.pop_back().unwrap();
        self.balance();
        val
    }
}

fn main() {
    // let ans = find_closest_elements(vec![1,2,3,4,5], 4, 3);
    // println!("{:?}", ans);

    let ans = sum_subarray_mins(vec![3,1,2,4]);
    println!("{:?}", ans);
}
