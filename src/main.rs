use std::collections::HashSet;
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
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let to_delete_set: HashSet<_> = to_delete.into_iter().collect();
        let mut roots = Vec::new();
        
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            is_root: bool,
            to_delete_set: &HashSet<i32>,
            roots: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node_ref) = node {
                let deleted;
                {
                    let node_borrow = node_ref.borrow();
                    deleted = to_delete_set.contains(&node_borrow.val);
                }
                let next_node_ref = node_ref.clone();
                {
                    let mut node_borrow = next_node_ref.borrow_mut();
                    node_borrow.left = dfs(node_borrow.left.take(), deleted, &to_delete_set, roots);
                    node_borrow.right = dfs(node_borrow.right.take(), deleted, &to_delete_set, roots);
                }
                if deleted {
                    None
                } else {
                    if is_root {
                        roots.push(Some(next_node_ref.clone()));
                    }
                    Some(next_node_ref)
                }
            } else {
                None
            }
        }

        dfs(root, true, &to_delete_set, &mut roots);
        
        roots
    }
}

pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
    let mut price = price;
    let len = price.len();
    price.sort_unstable();
    let mut left = 0;
    let mut right = price[len-1] - price[0];
    while left < right {
        let mid = (left + right + 1) / 2;
        if check(&price, k, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    return left;

}

fn check(price: &Vec<i32>, k: i32, tastiness: i32) -> bool {
    let mut prev = i32::MIN;
    let mut cnt = 0;
    for p in price.iter() {
        if *p - prev >= tastiness {
            cnt += 1;
            prev = *p;
        }
    }
    return cnt >= k;
}

pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut flags = vec![];
    words.iter().for_each(|word| {
        let word: Vec<char> = word.chars().collect();
        if vowels.contains(&word[0]) && vowels.contains(&word[word.len()-1]) {
            flags.push(true);
        } else {
            flags.push(false);
        }
    });
    let mut prefix = vec![0];
    let mut pre_sum = 0;
    for i in 0..flags.len() {
        if flags[i] == true {
            pre_sum += 1;
        }
        prefix.push(pre_sum);
    }
    let mut ans = vec![];
    for query in queries {
        let left = query[0] as usize;
        let right = query[1] as usize;
        ans.push(prefix[right+1] - prefix[left] )
    }

    ans
}


pub fn distinct_averages(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut sum_set: HashSet<i32> = HashSet::new();
    nums.sort_unstable();
    for i in 0..nums.len()/2 {
        sum_set.insert(nums[i] + nums[nums.len() - i - 1]);
    }
    sum_set.len() as i32
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![]
    }

    let phone = vec!["abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back("".to_string());
    for digit in digits.chars() {
        for _ in 0..queue.len() {
            let tmp = queue.pop_front().unwrap().to_string();
            for letter in phone[(digit as u8 - '0' as u8 ) as usize].chars() {
                let mut str = tmp.clone();
                str.push(letter);
                queue.push_back(str.clone());
            }
        }
    }

    queue.into_iter().collect::<Vec<String>>()
}

// pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
//     let rewards: Vec<_> = reward1.into_iter().zip(reward2).collect();
//     let k = k as usize;
    
//     rewards.into_iter()
//         .sorted_unstable_by(|&a, &b| (b.0 - b.1).cmp(&(a.0 - a.1)))
//         .enumerate()
//         .map(|(i, (r1, r2))| if i < k { r1 } else { r2 })
//         .sum()
// }
pub fn main() {

}