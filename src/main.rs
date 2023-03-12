
pub fn main() {
    let ans = count_subgraphs_for_each_diameter(4, vec![vec![1,2],vec![2,3],vec![2,4]]);
    assert_eq!(ans, vec![3,4,0]);
}

pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut roads = vec![];
    
}


// pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
//     if nums.len() <= 1 {
//         return 0;
//     }
//     let mut ans = 0;
//     for i in 0..nums.len()-1 {
//         let mut pre_xor = vec![0; 20];
//         let mut tmp = nums[i];
//         let mut idx = 0;
//         while tmp != 0 {
//             if tmp & 1 == 1 {
//                 pre_xor[idx] = 1;
//             }
//             tmp >>= 1;
//             idx += 1;
//         }
//         for j in i+1..nums.len() {
//             let mut tmp = nums[j];
//             let mut idx = 0;
//             let mut tmp_xor = vec![0; 20];
//             while tmp != 0 {
//                 if tmp & 1 == 1 {
//                     tmp_xor[idx] = 1;
//                 }
//                 tmp >>= 1;
//                 idx += 1;
//             }
//             for i in 0..pre_xor.len() {
//                 pre_xor[i] = pre_xor[i] ^ tmp_xor[i];
//             }
//             if pre_xor == vec![0; 20] {
//                 ans += 1;
//             }
//         }
//     }
//     ans
// }

// pub fn max_score(nums: Vec<i32>) -> i32 {
//     let mut nums = nums;
//     nums.sort_by_key(|&x| -x.abs());
//     let mut prefix_sum = 0;
//     let mut cnt = 0;
//     for num in &nums {
//         prefix_sum += num;
//         if *num > 0 {
//             cnt += 1;
//         }
//     }
//     if prefix_sum >= 0 {
//         cnt
//     } else {
//         cnt - 1
//     }
// }

// pub fn max_score(nums: Vec<i32>) -> i32 {
//     let mut nums = nums;
//     let mut ans = 0;
//     nums.sort_unstable_by(|a, b| b.cmp(&a));
//     println!("{:?}", nums);
//     let mut pre_sum = 0;
//     for i in 0..nums.len() {
//         pre_sum += nums[i];
//         if pre_sum > 0 {
//             ans += 1;
//         }
//     }

//     ans
// }
