use std::{collections::{HashSet, HashMap}, process::id, u64::MAX};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = candy(vec![1,2,87,87,87,2,1]);
    assert_eq!(ans, 13);
}


pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut kids = ratings.clone();
    kids.sort_unstable();
    let mut ans = kids.len();
    let mut pre = kids[0];
    for i in 1..ratings.len() {
        if kids[i] > pre {
            ans += 1;
        }
        pre = kids[i];
    }
    ans as i32
}