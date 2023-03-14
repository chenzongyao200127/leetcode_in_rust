use std::collections::HashMap;
use std::collections::VecDeque;

pub fn main() {
    // let ans = count_subgraphs_for_each_diameter(4, vec![vec![1,2],vec![2,3],vec![2,4]]);
    // assert_eq!(ans, vec![3,4,0]);

    let ans = min_window("ADOBECODEBANC".to_owned(), "ABC".to_string());
    assert_eq!(ans, "BANC".to_string());
}

pub fn min_window(s: String, t: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let len = s.len();
    let t: Vec<char> = t.chars().collect();
    let mut target_window = vec![0; 100];
    t.iter().for_each(|&ch| target_window[ch as usize - 'A' as usize] += 1);

    let mut cnt_window = vec![0; 100];
    let mut idx_window = vec![-1; 100];
    let mut left = 0;
    let mut right = 

    "1".to_owned()

}