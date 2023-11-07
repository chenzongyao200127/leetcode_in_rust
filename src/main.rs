use core::num;
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



fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    println!("{}", length_of_lis(nums));  // Expected output: 4
}

