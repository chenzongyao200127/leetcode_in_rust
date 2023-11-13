// 421_Maximum_XOR_of_Two_Numbers_in_an_Array
// https://leetcode.cn/problems/maximum-xor-of-two-numbers-in-an-array/description/?envType=daily-question&envId=2023-11-04

// 超时
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len()-1 {
            for j in 0..nums.len() {
                ans = ans.max(nums[i] ^ nums[j])
            }
        }

        ans
    }
}


// 前缀树
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


// https://leetcode.cn/problems/maximum-xor-of-two-numbers-in-an-array/solutions/2511644/tu-jie-jian-ji-gao-xiao-yi-tu-miao-dong-1427d/
use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        let high_bit = 31 - mx.leading_zeros() as i32;

        let mut ans = 0;
        let mut mask = 0;
        let mut seen = HashSet::new();
        for i in (0..=high_bit).rev() { // 从最高位开始枚举
            seen.clear();
            mask |= 1 << i;
            let new_ans = ans | (1 << i); // 这个比特位可以是 1 吗？
            for &x in &nums {
                let x = x & mask; // 低于 i 的比特位置为 0
                if seen.contains(&(new_ans ^ x)) {
                    ans = new_ans; // 这个比特位可以是 1
                    break;
                }
                seen.insert(x);
            }
        }
        ans
    }
}


use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        // Find the maximum number in the given list to determine the highest bit we need to consider
        let max_num = *nums.iter().max().unwrap();
        let highest_bit = 31 - max_num.leading_zeros() as i32;

        let mut result_xor = 0;
        let mut current_mask = 0;
        let mut prefix_set = HashSet::new();
        
        // Iterate from the highest bit to the lowest bit
        for i in (0..=highest_bit).rev() {
            // Clear the prefix set for each iteration
            prefix_set.clear();
            
            // Update the mask to consider bits up to i
            current_mask |= 1 << i;
            
            // Attempt to set the ith bit of the result
            let potential_xor = result_xor | (1 << i);
            
            for &num in &nums {
                // Only consider the prefix of the number (up to the ith bit)
                let prefix = num & current_mask;
                
                // If there exists another number in the set such that their XOR is the potential XOR,
                // this means the ith bit can be set to 1
                if prefix_set.contains(&(potential_xor ^ prefix)) {
                    result_xor = potential_xor;
                    break;
                }
                prefix_set.insert(prefix);
            }
        }

        result_xor
    }
}
