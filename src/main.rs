use std::{collections::HashMap, vec};

pub fn main() {
    // let ans = trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
    // assert_eq!(6, ans);

    // let ans = trap(vec![4,2,0,3,2,5]);
    // assert_eq!(9, ans);

    let ans = find_longest_subarray(vec!["1".to_string(),"A".to_string(),"B".to_string()]);
    assert_eq!(ans, vec!["1".to_string(), "A".to_string()]);
}

pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
    let mut new_arr = vec![];
    for s in array.iter() {
        if let Ok(_) = s.parse::<i32>() {
            new_arr.push(1);
        } else {
            new_arr.push(-1);
        }
    }
    let mut max = i32::MIN;
    let mut max_end = 0 as usize;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);
    let mut pre_sum = 0;
    for i in 0..new_arr.len() {
        pre_sum += new_arr[i];
        if map.contains_key(&pre_sum) {
            let len = i as i32 - map.get(&pre_sum).unwrap();
            if len > max {
                max = len;
                max_end = i;
            }
        } else {
            map.insert(pre_sum, i as i32);
        }
    }

    let mut ans = vec![];
    // println!("{:?}", (max, max_end));
    for i in max_end+1-max as usize..=max_end {
        ans.push(array[i].clone());
    }
    ans
}