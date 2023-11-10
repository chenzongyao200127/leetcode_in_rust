use std::vec;
use std::{collections::HashSet, process::id};

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

use std::collections::HashMap;
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


fn main() {
    let ans = successful_pairs(vec![9,39], vec![35,40,22,37,29,22], 320);
    println!("{:?}", ans);
}

