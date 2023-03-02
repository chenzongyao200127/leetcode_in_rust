use std::collections::{HashSet, HashMap};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = print_bin(0.625);
    assert_eq!(ans, "0.101".to_string());
}

pub fn print_bin(num: f64) -> String {
    let mut ans = "0.".to_string();
    let mut dig = -1;
    let mut n = num;
    let base = 2.0 as f64;
    while n != 0.0 {
        if n >= base.powi(dig) {
            n = n - base.powi(dig);
            ans.push('1');
        } else {
            ans.push('0');
        }
        dig -= 1;
        if dig == -30 {
            break;
        }
    }

    if n == 0.0 {
        ans
    } else {
        "ERROR".to_string()
    }
}


// pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//     let mut ans = vec![];
//     let mut set: HashSet<Vec<i32>> = HashSet::new();
//     for i in 0..nums.len()-3 {
//         for j in i+1..nums.len()-2 {
//             for m in j+1..nums.len()-1 {
//                 for n in m+1..nums.len() {
//                     if nums[i]+nums[j]+nums[m]+nums[n] == target {
//                         set.insert(vec![nums[i],nums[j],nums[m],nums[n]]);
//                     }
//                 }
//             }
//         }
//     }
//     for item in set {
//         ans.push(item);
//     }
//     ans
// }