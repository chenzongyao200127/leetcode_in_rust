// 599. Minimum Index Sum of Two Lists
// https://leetcode.cn/problems/minimum-index-sum-of-two-lists/

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map: HashMap<&str, usize> = HashMap::new();
        let mut ans_vec: Vec<(usize, usize)> = vec![];
        let mut ans = vec![];
        for i in 0..list2.len() {
            map.insert(&list2[i], i);
        }
        for i in 0..list1.len() {
            if let Some(&cnt) = map.get(&list1[i][..]) {
                ans_vec.push((i, cnt + i));
            }
        }
        ans_vec.sort_by_key(|x| x.1);
        let min = ans_vec[0].1;

        for (i, j) in ans_vec.into_iter() {
            if j == min {
                ans.push(list1[i].to_string());
            }
        }

        ans
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for (i, s) in list1.iter().enumerate() {
            map.insert(s, i);
        }
        let mut ans = vec![];
        let mut sum = usize::MAX;
        for (i, s) in list2.iter().enumerate() {
            if let Some(&j) = map.get(s) {
                if i + j > sum {
                    continue;
                }
                if i + j < sum {
                    ans.clear();
                    sum = i + j;
                }
                ans.push(s.to_string());
            }
        }
        ans
    }
}