use std::{collections::{HashSet, HashMap}, process::id};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = find_max_length(vec![0,0,0,1,1,1]);
    assert_eq!(ans, 6);
}

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);
    let mut compare = 0;
    for i in 0..nums.len() {
        if nums[i] == 1 {
            compare += 1;
        } else {
            compare -= 1;
        }
        if let Some(val) = map.get(&compare) {
            ans = ans.max(i as i32 - *val);
        } else {
            map.insert(compare, i as i32);
        }
    }
    ans
}